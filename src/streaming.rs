use futures_util::StreamExt;
use reqwest::Client;
use serde_json::Value;
use std::io::{self, Write};

const UPDATE_INTERVAL: usize = 20;
const BUFFER_SIZE: usize = 512;

pub struct StreamStats {
    pub token_count: usize,
    pub elapsed_secs: f64,
    pub tokens_per_sec: f64,
}

impl StreamStats {
    pub fn new(token_count: usize, elapsed_secs: f64) -> Self {
        let tokens_per_sec = if elapsed_secs > 0.0 {
            token_count as f64 / elapsed_secs
        } else {
            0.0
        };
        Self {
            token_count,
            elapsed_secs,
            tokens_per_sec,
        }
    }
}

pub async fn stream_response(
    client: &Client,
    model: &str,
    prompt: &str,
) -> Result<(String, StreamStats), Box<dyn std::error::Error>> {
    let mut full_response = String::new();
    let mut output_buffer = String::with_capacity(BUFFER_SIZE);
    let start_time = std::time::Instant::now();
    let mut token_count = 0;
    let mut last_update = 0;

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": model,
            "prompt": prompt,
            "stream": true
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()).into());
    }

    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        if let Ok(text) = std::str::from_utf8(&chunk) {
            for line in text.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                if let Ok(json) = serde_json::from_str::<Value>(line) {
                    if let Some(response_text) = json.get("response").and_then(|r| r.as_str()) {
                        token_count += 1;

                        // Update status less frequently
                        if token_count - last_update >= UPDATE_INTERVAL {
                            let elapsed = start_time.elapsed().as_secs_f64();
                            let tps = token_count as f64 / elapsed;
                            print!("\r⏳ {} tokens | {:.1} tok/s | {:.1}s", token_count, tps, elapsed);
                            io::stdout().flush().ok();
                            last_update = token_count;
                        }
                        full_response.push_str(response_text);

                        // Buffer output
                        output_buffer.push_str(response_text);
                        if output_buffer.len() >= BUFFER_SIZE {
                            print!("{}", output_buffer);
                            io::stdout().flush()?;
                            output_buffer.clear();
                        }
                    }
                    if json.get("done").and_then(|d| d.as_bool()).unwrap_or(false) {
                        if !output_buffer.is_empty() {
                            print!("{}", output_buffer);
                        }
                        println!();
                        break;
                    }
                }
            }
        }
    }

    let elapsed = start_time.elapsed().as_secs_f64();
    let stats = StreamStats::new(token_count, elapsed);
    Ok((full_response, stats))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(UPDATE_INTERVAL, 20);
        assert_eq!(BUFFER_SIZE, 512);
    }

    #[test]
    fn test_stream_stats() {
        let stats = StreamStats::new(100, 10.0);
        assert_eq!(stats.token_count, 100);
        assert_eq!(stats.elapsed_secs, 10.0);
        assert_eq!(stats.tokens_per_sec, 10.0);
    }

    #[test]
    fn test_stream_stats_zero_time() {
        let stats = StreamStats::new(100, 0.0);
        assert_eq!(stats.tokens_per_sec, 0.0);
    }
}
