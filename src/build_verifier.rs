use std::process::Command;
use std::fs;

pub struct BuildVerifier {
    temp_dir: String,
}

impl BuildVerifier {
    pub fn new() -> Self {
        Self {
            temp_dir: "/tmp/wiseowl_verify".to_string(),
        }
    }
    
    pub fn verify_rust(&self, code: &str) -> bool {
        let dir = format!("{}/rust_test", self.temp_dir);
        let _ = fs::create_dir_all(&dir);
        
        let file = format!("{}/lib.rs", dir);
        if fs::write(&file, code).is_err() {
            return false;
        }
        
        Command::new("/home/drew/.cargo/bin/rustc")
            .arg("--crate-type")
            .arg("lib")
            .arg(&file)
            .current_dir(&dir)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    
    pub fn verify_python(&self, code: &str) -> bool {
        let dir = format!("{}/python_test", self.temp_dir);
        let _ = fs::create_dir_all(&dir);
        
        let file = format!("{}/test.py", dir);
        if fs::write(&file, code).is_err() {
            return false;
        }
        
        Command::new("python3")
            .arg("-m")
            .arg("py_compile")
            .arg(&file)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    
    pub fn verify(&self, code: &str, language: &str) -> bool {
        match language.to_lowercase().as_str() {
            "rust" | "rs" => self.verify_rust(code),
            "python" | "py" => self.verify_python(code),
            _ => false,
        }
    }
    
    pub fn cleanup(&self) {
        let _ = fs::remove_dir_all(&self.temp_dir);
    }
}

impl Drop for BuildVerifier {
    fn drop(&mut self) {
        self.cleanup();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_verifier() {
        let verifier = BuildVerifier::new();
        assert_eq!(verifier.temp_dir, "/tmp/wiseowl_verify");
    }

    #[test]
    fn test_verify_rust_invalid() {
        let verifier = BuildVerifier::new();
        let code = "pub fn test( -> i32 { 42 }";
        assert!(!verifier.verify_rust(code));
    }

    #[test]
    fn test_verify_method_exists() {
        let verifier = BuildVerifier::new();
        let _ = verifier.verify("test", "unknown");
    }
}
