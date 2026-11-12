
// importo i moduli fs e env
use std::fs;
use std::env;
use owo_colors::OwoColorize;


pub fn look_for_models(){

    // dichiaro la home e le path per i modelli AI
    let home = env::var("HOME").unwrap(); 
    let models_dir = home.clone() + "/.AIman/models";
    let models_config_dir = home.clone() + "/.AIman/models_config";
    let models;

    // controllo l'esistenza delle cartelle altrimenti le creo
    
    if fs::read_dir(&models_dir).is_ok()  {

        check_config(models_config_dir);

        println!("{}","cartella dei modelli AI trovata con successo!".green());
        models = read_models(models_dir);
        // stampo a schermo i modelli AI trovati
        if models[0] != "Nessun modello AI trovato".to_string(){
            println!("modelli AI trovati:");
        }
        for modello in models{
            println!("{}",modello);
        }
    } else {
        println!("{}{}{}","cartella dei modelli AI NON trovata [X]".red(),"\n","creazione in corso...");
        create_model_path(models_dir);
        check_config(models_config_dir);
    }

    
}


// funzioni per creare le path in caso non esistano
pub fn create_model_path(path: String){
    let _ = fs::create_dir_all(path);
}

pub fn create_models_config(path: String){
    let _ = fs::create_dir_all(path);

}

// Leggo quali modelli AI (.gguf) ci sono
pub fn read_models(path: String) -> Vec<String>{
    let mut models = vec![];

    let files_or_dir = fs::read_dir(path).unwrap();

    for file in files_or_dir{
        if file.as_ref().unwrap().file_name().display().to_string().ends_with(".gguf"){
            models.push(file.unwrap().file_name().display().blue().to_string().split(".gguf").collect());
        }
    }

    

    if models.len() == 0{
        models.push("Nessun modello AI trovato".red().to_string());
    }

    return models;
}

// controllo la path per i config dei modelli AI
fn check_config(path:String){
    if fs::read_dir(&path).is_ok() {
        println!("{}","cartella config dei modelli AI trovata con successo!".green());

    } else{
        println!("{}{}{}","cartella config dei modelli AI NON trovata [X]".red(),"\n","creazione in corso...");
        create_models_config(path);
    }
}