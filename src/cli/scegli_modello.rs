use dialoguer::{Select, theme::ColorfulTheme};

pub fn scegli_modello(modelli: &[String]) -> usize {
    let selezione = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the AI model to start (use ↑/↓ arrows and Enter)")
        .default(0)
        .items(modelli)
        .items(&["Exit"])
        .interact()
        .unwrap();

    selezione // Restituisce l'indice del modello scelto
}