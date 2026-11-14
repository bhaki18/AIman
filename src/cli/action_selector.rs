use dialoguer::{Select, theme::ColorfulTheme};

pub fn select_action() -> u32{
    let options = &[
        "Avvia un modello Ai",
        "Installa un modello Ai",
        "Disinstalla un modello Ai",
        "Installa un server",
        "disinstalla un server",
        "esci"
    ];

    let scelta = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("seleziona una scelta")
        .items(options)
        .default(0)
        .interact()
        .unwrap();

    return scelta.try_into().unwrap();
    
}