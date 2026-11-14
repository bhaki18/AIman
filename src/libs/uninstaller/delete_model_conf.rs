use std::fs;
use std::env;
use owo_colors::OwoColorize;

pub fn delete_model_conf(model_name: String){
    let home = env::var("HOME").unwrap();
    let model_dir = home.clone() + "/.AIman/models_config/" + &model_name + ".json";
    println!("rimozione di {} in corso...",model_name);
    if fs::remove_file(model_dir).is_ok() {
        println!("{}","modello rimosso con successo!".green());
    }else{
        println!("{}","c'è stato un errore durante la rimozione del modello.".red());
    }

}