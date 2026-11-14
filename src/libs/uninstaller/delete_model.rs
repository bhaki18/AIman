use std::fs;
use std::env;
use super::delete_model_conf::delete_model_conf;

pub fn delete_model(model_name: String){
    let home = env::var("HOME").unwrap();
    let model_dir = home.clone() + "/.AIman/models/" + &model_name + ".gguf";

    let _ = fs::remove_file(model_dir);
    delete_model_conf(model_name);
    

}