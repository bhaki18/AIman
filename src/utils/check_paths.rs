use std::fs;
use owo_colors::OwoColorize;

// controllo la path per i config dei modelli AI
pub fn check_config(path:String) -> bool{
    if fs::read_dir(&path).is_ok() {
        println!("{}","cartella config dei modelli AI trovata con successo!".green());

        return true;
    } else{
        println!("{}{}{}","cartella config dei modelli AI NON trovata [X]".red(),"\n","creazione in corso...");
        return false;
    }
}

pub fn check_models(path:String)-> bool{
    if fs::read_dir(&path).is_ok() {

        println!("{}", "cartella dei modelli AI trovata con successo!".green());

        return true;
    } else {
        println!("{}{}{}", "cartella dei modelli AI NON trovata [X]".red(), "\n", "creazione in corso...");
        
        return false;
    }
}

pub fn check_servers(path:String)-> bool{
    if fs::read_dir(&path).is_ok() {

        println!("{}", "cartella dei servers trovata con successo!".green());

        return true;
    } else {
        println!("{}{}{}", "cartella dei servers NON trovata [X]".red(), "\n", "creazione in corso...");
        
        return false;
    }
}

