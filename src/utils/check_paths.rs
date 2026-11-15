use std::fs;
use owo_colors::OwoColorize;

// controllo la path per i config dei modelli AI
pub fn check_config(path:String) -> bool{
    if fs::read_dir(&path).is_ok() {
        println!("{}","AI models config folder found successfully!".green());

        return true;
    } else{
        println!("{}{}{}","AI models config folder NOT found [X]".red(),"\n","Creating folder...");
        return false;
    }
}

pub fn check_models(path:String)-> bool{
    if fs::read_dir(&path).is_ok() {

        println!("{}", "AI models folder found successfully!".green());

        return true;
    } else {
        println!("{}{}{}", "AI models folder NOT found [X]".red(), "\n", "Creating folder...");
        
        return false;
    }
}

pub fn check_servers(path:String)-> bool{
    if fs::read_dir(&path).is_ok() {

        println!("{}", "Servers folder found successfully!".green());

        return true;
    } else {
        println!("{}{}{}", "Servers folder NOT found [X]".red(), "\n", "Creating folder...");
        
        return false;
    }
}

