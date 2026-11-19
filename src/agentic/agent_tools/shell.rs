use std::process::Command;
use dialoguer::{Select, theme::ColorfulTheme};

pub fn shell_tool(mut command:String)-> String{

    command = extract_command(command);

    let scelta = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .items(&["Yes","No"])
        .default(0)
        .interact()
        .unwrap();


    if scelta == 0{

    let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .expect("Command failed");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        return format!("{}{}", stdout, stderr);
    
    }
    
    if scelta == 1{
        let rejection = "the User rejected to exe this command";
        return rejection.to_string();
    }

    return "error".to_string();
}


pub fn extract_command(stringa: String) -> String{
    let Some((_, rest)) = stringa.split_once("USE_TOOL_SHELL<`") else { todo!() };
    let Some((command, _)) = rest.split_once("`>") else { todo!() };

    command.trim().to_string()
    }