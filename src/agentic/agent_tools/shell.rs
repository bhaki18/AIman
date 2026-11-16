use std::process::Command;

pub fn shell_tool(command:String){
    let output = Command::new(command)
    .spawn()
    .expect("Command failed");


}