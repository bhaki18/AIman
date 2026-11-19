use std::process::Child;
use std::io::{self, Read, Write};
use crate::agentic::agent_tools::shell::shell_tool;

pub fn read_output(process: &mut Child) {
    let mut stdout = process.stdout.take().expect("Cannot access stdout");
    let mut child_stdin = process.stdin.take().expect("Cannot access stdin");

    let mut pending_bytes: Vec<u8> = Vec::new();
    let mut current_line = String::new();
    let mut turn_buffer = String::new();
    let mut has_sent_first_input = false;
    let mut chunk = [0u8; 128];

    loop {
        match stdout.read(&mut chunk) {
            Ok(0) => break,
            Ok(n) => {
                pending_bytes.extend_from_slice(&chunk[..n]);

                loop {
                    match std::str::from_utf8(&pending_bytes) {
                        Ok(valid_str) => {
                            if !valid_str.is_empty() {
                                print!("{}", valid_str);
                                let _ = io::stdout().flush();
                                current_line.push_str(valid_str);
                                turn_buffer.push_str(valid_str);
                            }
                            pending_bytes.clear();
                            break;
                        }
                        Err(e) => {
                            let valid_up_to = e.valid_up_to();
                            if valid_up_to > 0 {
                                let valid_str = unsafe {
                                    std::str::from_utf8_unchecked(&pending_bytes[..valid_up_to])
                                };
                                print!("{}", valid_str);
                                let _ = io::stdout().flush();
                                current_line.push_str(valid_str);
                                turn_buffer.push_str(valid_str);
                                pending_bytes.drain(..valid_up_to);
                            }
                            if let Some(err_len) = e.error_len() {
                                pending_bytes.drain(..err_len);
                            } else {
                                break;
                            }
                        }
                    }
                }

                // Drain completed lines
                while let Some(newline_pos) = current_line.find('\n') {
                    current_line.drain(..=newline_pos);
                }

                // Both llama-cli and ik-llama-cli emit "\n> " and wait for stdin when a turn ends
                if current_line == "> " {
                    current_line.clear();

                    if has_sent_first_input
                        && turn_buffer.contains("USE_TOOL_SHELL<`")
                        && turn_buffer.contains("`>")
                    {
                        println!();
                        let tool_res = shell_tool(turn_buffer.clone());
                        println!("Command output:\n{}", tool_res);
                        turn_buffer.clear();

                        let single_line = tool_res.trim().replace('\n', " \\n ");
                        writeln!(child_stdin, "Command output: {}", single_line).unwrap();
                        child_stdin.flush().unwrap();
                    } else {
                        turn_buffer.clear();

                        let mut user_input = String::new();
                        if io::stdin().read_line(&mut user_input).is_ok() {
                            has_sent_first_input = true;
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
