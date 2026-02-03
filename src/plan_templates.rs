pub fn get_plan_template(goal: &str) -> Option<Vec<String>> {
    let goal_lower = goal.to_lowercase();
    
    if goal_lower.contains("rest api") || goal_lower.contains("api") {
        Some(vec![
            "Set up project structure".to_string(),
            "Add web framework dependencies".to_string(),
            "Create route handlers".to_string(),
            "Add database integration".to_string(),
            "Implement authentication".to_string(),
            "Add error handling".to_string(),
            "Write tests".to_string(),
            "Add documentation".to_string(),
        ])
    } else if goal_lower.contains("cli") || goal_lower.contains("command line") {
        Some(vec![
            "Create new project".to_string(),
            "Add CLI argument parsing library".to_string(),
            "Define commands and subcommands".to_string(),
            "Implement command handlers".to_string(),
            "Add configuration file support".to_string(),
            "Write help documentation".to_string(),
            "Add tests".to_string(),
        ])
    } else if goal_lower.contains("web") && goal_lower.contains("server") {
        Some(vec![
            "Initialize web server project".to_string(),
            "Set up routing".to_string(),
            "Create HTML templates".to_string(),
            "Add static file serving".to_string(),
            "Implement request handlers".to_string(),
            "Add middleware".to_string(),
            "Test endpoints".to_string(),
        ])
    } else {
        None
    }
}
