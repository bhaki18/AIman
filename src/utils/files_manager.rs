
use std::env;
use super::check_paths::check_config;
use super::check_paths::check_models;
use super::check_paths::check_servers;
use super::create_paths::create_path;
use super::read_models::read_models;
use super::check_model_conf::check_model_conf;
use super::create_model_conf::create_model_conf;


pub fn start_fsman() -> Vec<String> {
    let home = env::var("HOME").unwrap(); 
    let models_dir = home.clone() + "/.AIman/models";
    let models_config_dir = home.clone() + "/.AIman/models_config";
    let servers_dir = home.clone() + "/.AIman/servers";
    let mut models = Vec::new(); // vettore vuoto

    if !check_models(models_dir.clone()){
        create_path(models_dir.clone());
    }

    if !check_config(models_config_dir.clone()){
        create_path(models_config_dir);
    }

    if !check_servers(servers_dir.clone()){
        create_path(servers_dir);
    }

    models = read_models(models_dir);

    for model in &models{
        if !check_model_conf(model.clone()){
            create_model_conf(model.to_string());
        }
    }
    
    return models;
}





