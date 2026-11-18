use std::process::Command;
use dialoguer::{Select, theme::ColorfulTheme};

pub fn shell_tool(command:String)-> String{

    let scelta = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .items(&["Yes","No"])
        .default(0)
        .interact()
        .unwrap();


    if scelta == 1{

    let output = Command::new(command)
    .output()
    .expect("Command failed");

    return String::from_utf8_lossy(&output.stdout).to_string();
    
    }
    
    if scelta == 0{
        let rejection = "the User rejected to exe this command";
        return rejection.to_string();
    }

    return "error".to_string();
}