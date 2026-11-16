use dialoguer::{Select, theme::ColorfulTheme};

pub fn select_action() -> u32{
    let options = &[
        "Start an AI model",
        "Install an AI model",
        "Uninstall an AI model",
        "Install a server",
        "Uninstall a server",
        "Start an Angent AI",
        "Exit"
    ];

    let scelta = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .items(options)
        .default(0)
        .interact()
        .unwrap();

    return scelta.try_into().unwrap();
    
}