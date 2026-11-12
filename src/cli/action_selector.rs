use dialoguer::{Select, theme::ColorfulTheme};

pub fn select_action() -> u32{
    let options = &[
        "Avvia un modello Ai",
        "Installa un modello Ai",
        "Disinstalla un modello Ai",
        "Installa un server",
        "esci"
    ];

    let scelta = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("seleziona una scelta")
        .items(options)
        .default(0)
        .interact()
        .unwrap();

    match scelta {
        0 => return 0,
        1 => return 1,
        2 => return 2,
        3 => return 3,
        4 => return 4,
        _ => 4,
    }
    
}