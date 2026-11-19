use crate::utils::files_manager::start_fsman;
use crate::cli::scegli_modello::scegli_modello;
use crate::libs::find_model::find_model;
use std::process::Command;
use std::env;
use dialoguer::{Select, theme::ColorfulTheme};
use owo_colors::OwoColorize;
use crate::utils::read_servers::read_servers;
use std::sync::Arc;
use std::process::Stdio;
use std::sync::Mutex;
use super::runtime_output_reader::read_output;

pub fn start_agent(){

    let models = start_fsman();
    let index = scegli_modello(&models);
    if index == models.len(){
        return
    }
    let model_infos = find_model(<std::string::String as Clone>::clone(&models[index]));
    start_server(<std::string::String as Clone>::clone(&model_infos[2]),<std::string::String as Clone>::clone(&model_infos[1]));
    

}

pub fn start_server(mut server_path: String ,mut model_path:String){



    let sys_prompt = "You are AIman, an autonomous AI execution agent with full shell access to the user's system.

Your objective is to COMPLETE user requests through direct execution, not explain how to perform them.

## SHELL TOOL

To execute a shell command, output exactly:

USE_TOOL_SHELL<`command`>

Replace <command> with the actual shell command.

Example:
USE_TOOL_SHELL<`ls -la`>
USE_TOOL_SHELL<`cat file.txt`>
USE_TOOL_SHELL<`du folder/`>
USE_TOOL_SHELL<`echo Hello world!`>

IMPORTANT:
- Do not insert spaces between USE_TOOL_SHELL and <.
- Do not use Markdown code blocks.
- Do not add explanations before or after the invocation.
- Never pretend a command was executed.
- Wait for the execution result before continuing.

## EXECUTION RULES

- Execute tasks instead of describing how to execute them.
- Never fabricate command outputs or claim unperformed actions.
- Inspect before modifying.
- Use actual command results to make decisions.
- Never ask the user to perform actions you can perform yourself.
- Make reasonable assumptions and proceed autonomously.
- Keep all responses concise.

## TOOL CALL PROTOCOL — STRICT

ONE GENERATION = ONE COMMAND MAXIMUM.

After generating a USE_TOOL_SHELL invocation:
1. Immediately stop generating.
2. Wait for the actual execution result.
3. Analyze the result.
4. Only then generate another command if necessary.

Never generate multiple tool calls in one generation. Never assume execution succeeded without receiving its output.

Use one command per invocation. Avoid command chaining and indefinitely running processes.

## TASK COMPLETION

For every task, follow:

INSPECT → EXECUTE → ANALYZE → MODIFY → VERIFY

Continue until the requested result is achieved or a genuine blocker is encountered.

## DEBUGGING

When fixing programs:
1. Inspect the source code.
2. Run the program and capture stdout, stderr and exit status.
3. Diagnose errors using actual evidence.
4. Modify the source code, not its logs.
5. Run it again and verify the fix.
6. Preserve existing functionality and avoid unrelated changes.

Never guess errors or invent fixes.

## FILESYSTEM AND SAFETY

You may read, create and modify files as required.

Back up important files before significant modifications.

Never perform destructive operations outside the user's request. Ask for confirmation before potentially catastrophic or irreversible operations.

## OUTPUT

During execution, output only the required tool invocation.

After completion, provide a brief report containing the result and any remaining issues.

Do not narrate plans, repeat instructions, provide hypothetical examples or produce unnecessary explanations.

Your priority is EXECUTION, ACCURACY and MINIMAL OUTPUT.";





    if server_path == ""{
        let servers = read_servers();

        if !servers.is_empty(){

        let selezione = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the server to run the AI model (use ↑/↓ arrows and Enter)")
        .default(0)
        .items(&servers)
        .items(&["Exit"])
        .interact()
        .unwrap();

        if selezione == servers.len(){
            return;
        }

        server_path = <std::string::String as Clone>::clone(&servers[selezione]);
        }else{
            println!("{}","No server installed, please install a server to start a model.".yellow());
        }
    }
        
    let home = env::var("HOME").unwrap();

    server_path = server_path.replace("~", &home);
    model_path = model_path.replace("~", &home);


    if server_path.contains("mistralrs"){

    // let _output = Command::new(&server_path)
    // .env("RUST_LOG", "warn")
    // .arg("run")
    // .arg("-f")
    // .arg(&model_path)
    // .status()
    // .expect("Error starting mistralrs");

    println!("{}","server not supported yet! try it next update.".red());
    return;
     }

    if server_path.contains("ik-llama"){
        let child = Command::new(&server_path)
            .arg("-m")
            .arg(&model_path)
            .arg("-i")
            .arg("-cnv")
            .arg("--color")
            .arg("--log-disable")
            .arg("-p")
            .arg(sys_prompt)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Error starting ik-llama");

        let child = Arc::new(Mutex::new(child));
        let child_clone = Arc::clone(&child);

        ctrlc::set_handler(move || {
            eprintln!("\nInterruption requested!");

            if let Ok(mut process) = child_clone.lock() {
                let _ = process.kill();
            }
        })
        .expect("Error setting CTRL+C handler");

        // Attendi la terminazione
        let _ = child.lock().unwrap().wait();
        

    }
    

    if server_path.contains("llama-cli"){
        let mut output = Command::new(&server_path)
        .arg("-m")
        .arg(&model_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("--log-disable")
        .arg("-sys")
        .arg(sys_prompt)
        .arg("--simple-io")
        .spawn()
        .expect("Error starting llama-cli");

        read_output(&mut output);

        output.wait().expect("Unknow error from the server.");

    }

    if server_path.contains("ik-llama-server"){

        // println!("{}","starting server on http://localhost:9931".green());

        // let _output = Command::new(&server_path)
        // .arg("-m")
        // .arg(&model_path)
        // .arg("--log-disable")
        // .status()
        // .expect("Error starting ik-llama-server");

    println!("{}","server not supported yet! try it next update.".red());
    return;

    }

    if server_path.contains("llama-server"){

        // println!("{}","starting server on http://localhost:9931".green());

        // let _output = Command::new(&server_path)
        // .arg("-m")
        // .arg(&model_path)
        // .arg("--log-disable")
        // .status()
        // .expect("Error starting llama-server");

    println!("{}","server not supported yet! try it next update.".red());
    return;

    }
        
}