use dialoguer::{Select, theme::ColorfulTheme};

pub fn select_install_method() -> i32{
    let options = &[
        "Installa da hugging face",
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
        _ => 1,
    }
}