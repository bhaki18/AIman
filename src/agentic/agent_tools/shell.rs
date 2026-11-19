use std::io::{self, Write};
use std::process::Command;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::Duration;
use dialoguer::{Select, theme::ColorfulTheme};
use owo_colors::OwoColorize;

pub fn shell_tool(mut command: String) -> String {
    command = extract_command(command);

    let scelta = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Execute `{}`?", command))
        .items(&["Yes", "No"])
        .default(0)
        .interact()
        .unwrap();

    if scelta == 0 {
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = Arc::clone(&running);

        let spinner_thread = thread::spawn(move || {
            let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
            let mut i = 0;
            while running_clone.load(Ordering::SeqCst) {
                print!("\r{} {}", frames[i % frames.len()].cyan(), "working...".yellow());
                let _ = io::stdout().flush();
                thread::sleep(Duration::from_millis(80));
                i += 1;
            }
            print!("\r\x1b[2K");
            let _ = io::stdout().flush();
        });

        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .expect("Command failed");

        running.store(false, Ordering::SeqCst);
        let _ = spinner_thread.join();

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        return format!("{}{}", stdout, stderr);
    }

    if scelta == 1 {
        let rejection = "the User rejected to exe this command";
        return rejection.to_string();
    }

    return "error".to_string();
}

pub fn extract_command(stringa: String) -> String {
    let Some((_, rest)) = stringa.split_once("USE_TOOL_SHELL<`") else {
        return String::new();
    };
    let Some((command, _)) = rest.split_once("`>") else {
        return String::new();
    };

    command.trim().to_string()
}
