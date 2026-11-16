use std::process::Command;
use owo_colors::OwoColorize;

pub fn shell_tool(command:String){
    let output = Command::new(command)
    .spawn()
    .expect("Command failed".red());


}