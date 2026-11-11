
// importo i moduli fs e env
use std::fs;
use std::env;


pub fn look_for_models(){

    // dichiaro la home e le path per i modelli AI
    let home = env::var("HOME").unwrap(); 
    let models_dir = home.clone() + "/.AIman/models";
    let models_config_dir = home.clone() + "/.AIman/models_config";

    // controllo l'esistenza delle cartelle altrimenti le creo
    
    if fs::read_dir(&models_dir).is_ok()  {
        println!("cartella dei modelli AI trovata con successo!");
    } else {
        println!("cartella dei modelli AI NON trovata [X]\ncreazione in corso...");
        create_model_path(models_dir);
    }

    if fs::read_dir(&models_config_dir).is_ok() {
        println!("cartella config dei modelli AI trovata con successo!");

    } else{
        println!("cartella config dei modelli AI NON trovata [X]\ncreazione in corso...");
        create_models_config(models_config_dir);
    }
}


// funzioni per creare le path in caso non esistano
pub fn create_model_path(path: String){
    let _ = fs::create_dir_all(path);
}

pub fn create_models_config(path: String){
    let _ = fs::create_dir_all(path);

}