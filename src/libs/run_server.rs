use std::process::{Command, Stdio};
use std::env;

pub fn start_server(model: String,family: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Avvio del server in background...");
    let home = env::var("HOME")?;
    
    let child = Command::new(format!("{}/.cargo/bin/mistralrs", home))
        .args([
            "serve",
            "-p", "8080",
            "--format", "gguf",
            "-m", &format!("{}/.AIman/models", home),
            "-f", "qwen2.5-coder-3b-instruct-q4_k_m.gguf",
            "--tok-model-id", "Qwen/Qwen2.5-Coder-3B-Instruct",
        ])
        .stdout(Stdio::null()) // <-- Nasconde tutti i log normali (INFO, barre di progresso)
        .stderr(Stdio::null()) // <-- Nasconde anche i log su stderr
        .spawn()?;

    println!("Server avviato con PID: {}", child.id());
    Ok(())
}