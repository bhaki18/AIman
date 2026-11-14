use std::fs;
use std::env;

pub fn create_model_conf(model_name:String){
    let home = env::var("HOME").unwrap(); 
    let model_config_dir = home.clone() + "/.AIman/models_config/" + &model_name + ".json";
    let models_dir = home.clone() + "/.AIman/models";
    
    let mut model_path = String::new();
    let file_in_models = fs::read_dir(models_dir).unwrap();
    for file in file_in_models{
        if file.as_ref().unwrap().file_name().display().to_string().ends_with(".gguf") && file.as_ref().unwrap().file_name().display().to_string() == model_name.clone() + ".gguf"{
            model_path = file.as_ref().unwrap().path().display().to_string();
        }
    }

let body = format!(
    "{{\n\"nome_modello\":\"{}\",\n\"path_modello\":\"{}\",\n\"famiglia_modello\":\"\",\n\"tipo_di_modello\":\"\",\n\"repo_hf\":\"\",\n\"server_utilizzato\":\"\",\n\"quantizzazione\":\"\"\n}}",
    model_name,
    model_path
);    let _ = fs::write(model_config_dir,body);
    
}