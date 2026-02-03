use rustyline::error::ReadlineError;
use rustyline::{Editor, Config, CompletionType, history::DefaultHistory};
use rustyline::completion::{Completer, Pair};
use rustyline::hint::Hinter;
use rustyline::highlight::Highlighter;
use rustyline::validate::Validator;
use rustyline::Helper;

pub struct OcliHelper {
    commands: Vec<String>,
}

impl OcliHelper {
    pub fn new() -> Self {
        Self {
            commands: vec![
                "/help", "/plan", "/next", "/show-plan",
                "/todo", "/done", "/rule", "/context",
                "/read", "/write", "/preview", "/apply", "/rollback",
                "/mcp", "/config", "/export", "/stats", "/monitor",
                "/git", "/version", "/history", "/alias", "/perf",
                "/clear", "/exit",
            ].iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl Completer for OcliHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        _pos: usize,
        _ctx: &rustyline::Context,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let mut candidates = Vec::new();
        
        if line.starts_with("/") {
            for cmd in &self.commands {
                if cmd.starts_with(line) {
                    candidates.push(Pair {
                        display: cmd.clone(),
                        replacement: cmd.clone(),
                    });
                }
            }
        }
        
        Ok((0, candidates))
    }
}

impl Hinter for OcliHelper {
    type Hint = String;

    fn hint(&self, line: &str, _pos: usize, _ctx: &rustyline::Context) -> Option<String> {
        if line.starts_with("/") && line.len() > 1 {
            for cmd in &self.commands {
                if cmd.starts_with(line) && cmd != line {
                    return Some(cmd[line.len()..].to_string());
                }
            }
        }
        None
    }
}

impl Highlighter for OcliHelper {}
impl Validator for OcliHelper {}
impl Helper for OcliHelper {}

pub fn create_editor() -> Editor<OcliHelper, DefaultHistory> {
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .auto_add_history(true)
        .build();
    
    let mut rl = Editor::with_config(config).unwrap();
    rl.set_helper(Some(OcliHelper::new()));
    rl
}
