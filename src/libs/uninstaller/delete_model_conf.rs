use std::fs;
use std::env;
use owo_colors::OwoColorize;

pub fn delete_model_conf(model_name: String){
    let home = env::var("HOME").unwrap();
    let model_dir = home.clone() + "/.AIman/models_config/" + &model_name + ".json";
    println!("Removing {}...",model_name);
    if fs::remove_file(model_dir).is_ok() {
        println!("{}","Model removed successfully!".green());
    }else{
        println!("{}","An error occurred while removing the model.".red());
    }

}