use std::process::Command;

/// Server configuration engine for handling command operations
/// Processes server requests and performs command operations
pub fn handle_server_operations(server_data: String) -> Result<String, String> {
    let config_data = parse_server_config(server_data);
    let command_data = build_server_command(config_data);
    let execution_data = prepare_command_execution(command_data);
    
    let first_result = execute_server_config(&execution_data);
    let second_result = run_server_command(&execution_data);
    
    Ok(format!("Server operations: {} | {}", first_result, second_result))
}

/// Parse server configuration and transform command structure
fn parse_server_config(server_data: String) -> String {
    let config_processed = server_data.replace("config", "parsed_config");
    let server_info = format!("SERVER_ID={} | PORT={} | PROTOCOL={}", 
                            server_data.len() % 65535,
                            server_data.len() % 8080,
                            if server_data.contains("http") { "HTTP" } else { "TCP" });
    format!("{} -- CONFIG_PARSE -- {}", config_processed, server_info)
}

/// Build server command with additional configuration metadata
fn build_server_command(config_data: String) -> String {
    let command_info = format!("CMD_TYPE={} | PRIORITY={} | TIMEOUT={}", 
                             if config_data.len() > 20 { "HIGH" } else { "LOW" },
                             config_data.len() % 10,
                             config_data.len() * 100);
    format!("{} -- COMMAND_BUILD -- {}", config_data, command_info)
}

/// Prepare command execution with final server optimizations
fn prepare_command_execution(command_data: String) -> String {
    let exec_processed = command_data.to_lowercase();
    if exec_processed.contains("restart") {
        command_data.replace("restart", "reload")
    } else {
        format!("exec_{}", exec_processed)
    }
}

/// Execute server configuration with tainted data (first sink)
fn execute_server_config(data: &str) -> String {
    let server_command = data.to_string();
    //SINK
    let _result = Command::new("systemctl").arg(&server_command).output();
    format!("Server config executed: {} commands", server_command.len())
}

/// Run server command with tainted data (second sink)
fn run_server_command(data: &str) -> String {
    let command_args = data.to_string();
    //SINK
    let _result = Command::new("service").args(&[&command_args]).output();
    format!("Server command run: {} arguments", command_args.len())
} 