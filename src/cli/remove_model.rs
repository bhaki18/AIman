use std::fs;
use std::env;
use dialoguer::{Select, theme::ColorfulTheme};
use crate::utils::read_models::read_models;
use crate::libs::uninstaller::delete_model::delete_model;

pub fn remove_model(){

    let home = env::var("HOME").unwrap(); 
    let models_dir = home.clone() + "/.AIman/models";


    let modelli = read_models(models_dir);

    let selezione = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Seleziona il modello AI da rimuovere (usa le frecce ↑/↓ e Invio)")
        .default(0)
        .items(&modelli)
        .items(&["esci"])
        .interact()
        .unwrap();

    if selezione != modelli.len(){// Restituisce l'indice del modello scelto
        delete_model(<std::string::String as Clone>::clone(&modelli[selezione]));
    }else{
        return;
    }

}