use dialoguer::{Select, theme::ColorfulTheme};

pub fn select_install_method() -> i32{
    let options = &[
        "Install from Hugging Face",
        "Exit"
    ];

    let scelta = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
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