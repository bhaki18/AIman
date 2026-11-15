
use std::env;
use dialoguer::{Select, theme::ColorfulTheme};
use crate::utils::read_models::read_models;
use crate::libs::uninstaller::delete_model::delete_model;

pub fn remove_model(){

    let home = env::var("HOME").unwrap(); 
    let models_dir = home.clone() + "/.AIman/models";


    let modelli = read_models(models_dir);

    let selezione = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the AI model to remove (use ↑/↓ arrows and Enter)")
        .default(0)
        .items(&modelli)
        .items(&["Exit"])
        .interact()
        .unwrap();

    if selezione != modelli.len(){// Restituisce l'indice del modello scelto
        delete_model(<std::string::String as Clone>::clone(&modelli[selezione]));
    }else{
        return;
    }

}