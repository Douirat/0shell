
pub fn parse_command(input: &str) -> Command {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let name = parts.get(0).unwrap_or(&"").to_string();
    let args = parts.iter().skip(1).map(|s| s.to_string()).collect();
    Command { name, args }
}