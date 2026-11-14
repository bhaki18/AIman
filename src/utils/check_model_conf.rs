use std::fs;
use std::env;

pub fn check_model_conf(model_name:String) -> bool{
    let home = env::var("HOME").unwrap(); 
    let model_config_dir = home.clone() + "/.AIman/models_config/" + &model_name + ".json";

    if fs::read_dir(&model_config_dir).is_ok(){
        return true;
    }else{
        return false;
    }
}