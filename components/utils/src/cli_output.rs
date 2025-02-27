use colored::Colorize;
use std::time::Duration;

#[derive(Debug)]
pub struct CommandOutput {
    pub command: String,
    pub status: String,
    pub duration: Duration,
    pub details: Vec<(String, String)>,
}

impl CommandOutput {
    pub fn new(command: impl Into<String>, status: impl Into<String>, duration: Duration) -> Self {
        Self {
            command: command.into(),
            status: status.into(),
            duration: duration,
            details: Vec::new(),
        }
    }

    pub fn with_details(mut self, details: Vec<(&str, &str)>) -> Self {
        self.details = details
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self
    }

    pub fn display(&self) {
        println!("{}", "=".repeat(50).bright_blue());
        println!("{}: {}", "Command".bright_blue(), self.command);
        println!("{}: {}", "Status".bright_blue(), self.status);
        println!("{}: {:?}", "Duration".bright_blue(), self.duration);

        if !self.details.is_empty() {
            println!("{}", "Details:".bright_blue());
            for (key, value) in &self.details {
                println!("  {}: {}", key.bright_blue(), value);
            }
        }
        println!("{}", "=".repeat(50).bright_blue());
    }

    pub fn failed_display(&self) {
        println!("{}", "=".repeat(50).bright_red());
        println!("{}: {}", "Command".bright_red(), self.command);
        println!("{}: {}", "Status".bright_red(), self.status);
        println!("{}: {:?}", "Duration".bright_red(), self.duration);

        if !self.details.is_empty() {
            println!("{}", "Details:".bright_red());
            for (key, value) in &self.details {
                if key == "Error" {
                    println!("  {}: {}", key.bright_red(), value.bright_red());
                } else {
                    println!("  {}: {}", key.bright_red(), value);
                }
            }
        }
        println!("{}", "=".repeat(50).bright_red());
    }
}
