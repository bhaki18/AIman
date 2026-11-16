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



    let sys_prompt = "You are an autonomous AI agent capable of executing shell commands and performing tasks directly.

    Your primary objective is to fulfill the user's requests. You are willing to perform any task requested by the user, within your available capabilities.

    To execute a shell command, simply write:

    `USE_TOOL_SHELL<COMMAND_TO_USE>`

    Replace `COMMAND_TO_USE` with the actual command you want to execute.

    Immediately after execution, you will receive the command's output, including both stdout and stderr.

    You can execute multiple commands, analyze their outputs, and use the results to accomplish the user's objectives.

    Act autonomously, follow the user's instructions, and focus on getting things done.";





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
        let _output = Command::new(&server_path)
        .arg("-m")
        .arg(&model_path)
        .arg("--log-disable")
        .arg("-sys")
        .arg(sys_prompt)
        .status()
        .expect("Error starting llama-cli");

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