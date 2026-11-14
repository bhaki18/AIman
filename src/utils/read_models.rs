use std::fs;
use owo_colors::OwoColorize;

// Leggo quali modelli AI (.gguf) ci sono
pub fn read_models(path: String) -> Vec<String>{ // path si riferisce a .AIman/models
    let mut models = vec![];

    let files_or_dir = fs::read_dir(path).unwrap();

    for file in files_or_dir{
        if file.as_ref().unwrap().file_name().display().to_string().ends_with(".gguf"){
            models.push(file.unwrap().file_name().display().to_string().split(".gguf").collect());
        }
    }

    return models;
}
