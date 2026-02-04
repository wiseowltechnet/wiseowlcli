use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

pub struct SyntaxHighlighter {
    ps: SyntaxSet,
    ts: ThemeSet,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            ps: SyntaxSet::load_defaults_newlines(),
            ts: ThemeSet::load_defaults(),
        }
    }
    
    pub fn highlight(&self, code: &str, lang: &str) -> String {
        let syntax = self.ps.find_syntax_by_extension(lang)
            .unwrap_or_else(|| self.ps.find_syntax_plain_text());
        
        let mut h = HighlightLines::new(syntax, &self.ts.themes["base16-ocean.dark"]);
        
        let mut output = String::new();
        for line in code.lines() {
            let ranges = h.highlight_line(line, &self.ps).unwrap();
            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
            output.push_str(&escaped);
            output.push('\n');
        }
        output
    }
}
