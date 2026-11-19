use std::process::Child;
use std::io::{self, BufRead, BufReader, Write};
use crate::agentic::agent_tools::shell::shell_tool;

pub fn read_output(process: &mut Child) {
    let stdout = process.stdout.take().expect("Cannot access stdout");
    let mut child_stdin = process.stdin.take().expect("Cannot access stdin");
    let reader = BufReader::new(stdout);

    // Conterrà l'output del tool se il modello ha chiamato USE_TOOL_SHELL in questo turno
    let mut pending_tool_output: Option<String> = None;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                println!("{}", line);

                // 1. Se il modello ha generato un comando tool, eseguilo e salva l'output
                if line.contains("USE_TOOL_SHELL<`") && line.contains("`>") {
                    let tool_res = shell_tool(line);
                    println!("Output comando:\n{}", tool_res);
                    pending_tool_output = Some(tool_res);
                }
                // 2. Quando llama-cli ha finito il banner iniziale o ha finito di generare la risposta:
                else if line.contains("/glob <pattern>") || line.starts_with("[ Prompt:") {
                    if let Some(tool_res) = pending_tool_output.take() {
                        // Trasforma i \n in spazi o \n testuali così llama-cli lo legge come un unico messaggio
                        let single_line = tool_res.trim().replace('\n', " \\n ");
                        writeln!(child_stdin, "Command output: {}", single_line).unwrap();
                        child_stdin.flush().unwrap();
                    } else {
                        // Altrimenti tocca all'utente scrivere
                        print!("\nYou > ");
                        io::stdout().flush().unwrap();

                        let mut user_input = String::new();
                        if io::stdin().read_line(&mut user_input).is_ok() {
                            writeln!(child_stdin, "{}", user_input.trim()).unwrap();
                            child_stdin.flush().unwrap();
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading stdout: {}", e);
                break;
            }
        }
    }
}