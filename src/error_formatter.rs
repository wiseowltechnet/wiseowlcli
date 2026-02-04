pub struct ErrorFormatter;

impl ErrorFormatter {
    pub fn format(error: &str, context: Option<&str>) -> String {
        let mut output = String::new();
        
        output.push_str("╔═══════════════════════════════════════════════════════════╗\n");
        output.push_str("║ ❌ ERROR                                                  ║\n");
        output.push_str("╠═══════════════════════════════════════════════════════════╣\n");
        
        for line in error.lines() {
            output.push_str(&format!("║ {}{}║\n", line, " ".repeat(57 - line.len().min(57))));
        }
        
        if let Some(ctx) = context {
            output.push_str("╠═══════════════════════════════════════════════════════════╣\n");
            output.push_str("║ 💡 SUGGESTION                                             ║\n");
            output.push_str("╠═══════════════════════════════════════════════════════════╣\n");
            for line in ctx.lines() {
                output.push_str(&format!("║ {}{}║\n", line, " ".repeat(57 - line.len().min(57))));
            }
        }
        
        output.push_str("╚═══════════════════════════════════════════════════════════╝");
        output
    }
    
    pub fn format_validation_error(language: &str, error: &str) -> String {
        Self::format(
            &format!("Invalid {} code: {}", language, error),
            Some("Check syntax and try again")
        )
    }
    
    pub fn format_build_error(language: &str, output: &str) -> String {
        Self::format(
            &format!("{} build failed", language),
            Some(&format!("Compiler output:\n{}", output.lines().take(3).collect::<Vec<_>>().join("\n")))
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_basic() {
        let result = ErrorFormatter::format("Test error", None);
        assert!(result.contains("❌ ERROR"));
        assert!(result.contains("Test error"));
    }

    #[test]
    fn test_format_with_context() {
        let result = ErrorFormatter::format("Test error", Some("Try this"));
        assert!(result.contains("💡 SUGGESTION"));
        assert!(result.contains("Try this"));
    }

    #[test]
    fn test_format_validation_error() {
        let result = ErrorFormatter::format_validation_error("rust", "missing semicolon");
        assert!(result.contains("Invalid rust code"));
        assert!(result.contains("Check syntax"));
    }

    #[test]
    fn test_format_build_error() {
        let result = ErrorFormatter::format_build_error("rust", "error: expected `;`");
        assert!(result.contains("rust build failed"));
        assert!(result.contains("Compiler output"));
    }
}
