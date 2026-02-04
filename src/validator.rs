pub struct CodeValidator {
    valid_count: usize,
    invalid_count: usize,
}

impl CodeValidator {
    pub fn new() -> Self {
        Self {
            valid_count: 0,
            invalid_count: 0,
        }
    }
    
    pub fn validate_rust(&mut self, code: &str) -> bool {
        let valid = code.contains("fn ") && code.contains("{") && code.contains("}");
        if valid {
            self.valid_count += 1;
        } else {
            self.invalid_count += 1;
        }
        valid
    }
    
    pub fn validate_python(&mut self, code: &str) -> bool {
        let valid = code.contains("def ") || code.contains("class ") || code.contains("import ");
        if valid {
            self.valid_count += 1;
        } else {
            self.invalid_count += 1;
        }
        valid
    }
    
    pub fn validate_javascript(&mut self, code: &str) -> bool {
        let valid = code.contains("function ") || code.contains("const ") || code.contains("let ");
        if valid {
            self.valid_count += 1;
        } else {
            self.invalid_count += 1;
        }
        valid
    }
    
    pub fn validate(&mut self, code: &str, language: &str) -> bool {
        match language.to_lowercase().as_str() {
            "rust" | "rs" => self.validate_rust(code),
            "python" | "py" => self.validate_python(code),
            "javascript" | "js" => self.validate_javascript(code),
            _ => {
                self.invalid_count += 1;
                false
            }
        }
    }
    
    pub fn stats(&self) -> (usize, usize) {
        (self.valid_count, self.invalid_count)
    }
    
    pub fn success_rate(&self) -> f64 {
        let total = self.valid_count + self.invalid_count;
        if total == 0 {
            0.0
        } else {
            self.valid_count as f64 / total as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_rust_valid() {
        let mut validator = CodeValidator::new();
        assert!(validator.validate_rust("fn main() {}"));
    }

    #[test]
    fn test_validate_rust_invalid() {
        let mut validator = CodeValidator::new();
        assert!(!validator.validate_rust("let x = 5;"));
    }

    #[test]
    fn test_validate_python_valid() {
        let mut validator = CodeValidator::new();
        assert!(validator.validate_python("def main():\n    pass"));
    }

    #[test]
    fn test_validate_by_language() {
        let mut validator = CodeValidator::new();
        assert!(validator.validate("fn main() {}", "rust"));
        assert!(validator.validate("def main(): pass", "python"));
    }

    #[test]
    fn test_stats() {
        let mut validator = CodeValidator::new();
        validator.validate("fn main() {}", "rust");
        validator.validate("invalid", "rust");
        let (valid, invalid) = validator.stats();
        assert_eq!(valid, 1);
        assert_eq!(invalid, 1);
    }

    #[test]
    fn test_success_rate() {
        let mut validator = CodeValidator::new();
        validator.validate("fn main() {}", "rust");
        validator.validate("fn test() {}", "rust");
        validator.validate("invalid", "rust");
        assert_eq!(validator.success_rate(), 2.0 / 3.0);
    }
}
