// commands.rs

/// Extract SESSION:NAME command
pub fn extract_session_command(response_text: &str) -> Option<(String, String)> {
    for line in response_text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("SESSION:") {
            let rest = rest.trim();
            if let Some((name, cmd)) = rest.split_once(' ') {
                return Some((name.trim().to_string(), cmd.trim().to_string()));
            } else if !rest.is_empty() {
                return Some((rest.to_string(), String::new()));
            }
        }
    }
    None
}

/// Extract COMMAND:
pub fn extract_command(response_text: &str) -> Option<String> {
    for line in response_text.lines() {
        let line = line.trim();
        if let Some(cmd) = line.strip_prefix("COMMAND:") {
            return Some(cmd.trim().to_string());
        }
    }
    None
}

/// Extract END_SESSION:
pub fn extract_end_command(response_text: &str) -> Option<String> {
    for line in response_text.lines() {
        let line = line.trim();
        if let Some(name) = line.strip_prefix("END_SESSION:") {
            return Some(name.trim().to_string());
        }
    }
    None
}

/// Extract RUN command (if you still want to support this format)
pub fn extract_run_command(response_text: &str) -> Option<(String, String)> {
    for line in response_text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("TOOL_NAME: RUN") {
            let rest = rest.trim();
            if let Some((name, cmd)) = rest.split_once(' ') {
                return Some((name.to_string(), format!("run {}", cmd.trim())));
            }
        }
    }
    None
}
