use std::process::Child;
use std::io::{self, Read, Write};
use crate::agentic::agent_tools::shell::shell_tool;

pub fn read_output(process: &mut Child) {
    let mut stdout = process.stdout.take().expect("Cannot access stdout");
    let mut child_stdin = process.stdin.take().expect("Cannot access stdin");

    let mut pending_tool_output: Option<String> = None;
    let mut pending_bytes: Vec<u8> = Vec::new();
    let mut current_line = String::new();
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
                                pending_bytes.drain(..valid_up_to);
                            }
                            if let Some(err_len) = e.error_len() {
                                pending_bytes.drain(..err_len);
                            } else {
                                // Incomplete multi-byte UTF-8 sequence at end of chunk; wait for next read
                                break;
                            }
                        }
                    }
                }

                // Process all completed lines in current_line
                while let Some(newline_pos) = current_line.find('\n') {
                    let line: String = current_line.drain(..=newline_pos).collect();
                    let line_trimmed = line.trim_end_matches(&['\r', '\n'][..]);

                    if line_trimmed.contains("USE_TOOL_SHELL<`") && line_trimmed.contains("`>") {
                        let tool_res = shell_tool(line_trimmed.to_string());
                        println!("Command output:\n{}", tool_res);
                        pending_tool_output = Some(tool_res);
                    } else if line_trimmed.contains("/glob <pattern>") || line_trimmed.starts_with("[ Prompt:") {
                        if let Some(tool_res) = pending_tool_output.take() {
                            let single_line = tool_res.trim().replace('\n', " \\n ");
                            writeln!(child_stdin, "Command output: {}", single_line).unwrap();
                            child_stdin.flush().unwrap();
                        } else {
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
            }
            Err(e) => {
                eprintln!("Error reading stdout: {}", e);
                break;
            }
        }
    }
}
