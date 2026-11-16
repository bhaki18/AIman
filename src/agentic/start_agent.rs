use std::process::Command;
use owo_colors::OwoColorize;

pub fn start_agent(model_path:String,server_path:String){
    let output = Command::new(server_path)
    .arg("-m")
    .arg(model_path)
    .spawn()
    .expect("Something went wrong during model start".red());

}