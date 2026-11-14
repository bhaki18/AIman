use crate::utils::files_manager::start_fsman;
use super::scegli_modello::scegli_modello;
use crate::libs::find_model::find_model;
use std::process::Command;
use std::env;
use std::process::Stdio;
use dialoguer::{Select, theme::ColorfulTheme};
use owo_colors::OwoColorize;
use crate::utils::read_servers::read_servers;

pub fn avvia_modello(){

    let models = start_fsman();
    let index = scegli_modello(&models);
    let model_infos = find_model(<std::string::String as Clone>::clone(&models[index]));
    start_server(<std::string::String as Clone>::clone(&model_infos[2]),<std::string::String as Clone>::clone(&model_infos[1]));



}

pub fn start_server(mut server_path: String ,mut model_path:String){
    if server_path == ""{
        let servers = read_servers();

        if !servers.is_empty(){

        let selezione = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Seleziona il modello AI da avviare (usa le frecce ↑/↓ e Invio)")
        .default(0)
        .items(&servers)
        .interact()
        .unwrap();

        server_path = <std::string::String as Clone>::clone(&servers[selezione]);
        }else{
            println!("{}","nessun server installato,installa un server per avviare un modello.".yellow());
        }
    }
        
    let home = env::var("HOME").unwrap();

    server_path = server_path.replace("~", &home);
    model_path = model_path.replace("~", &home);


    if server_path.ends_with("mistralrs"){

    let output = Command::new(server_path)
    .env("RUST_LOG", "warn")
    .arg("run")
    .arg("-f")
    .arg(model_path)
    .status()
    .expect("Errore nell'avvio di mistralrs");
    
    }
        
}