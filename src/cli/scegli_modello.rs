use dialoguer::{Select, theme::ColorfulTheme};

pub fn scegli_modello(modelli: &[String]) -> usize {
    let selezione = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Seleziona il modello AI da avviare (usa le frecce ↑/↓ e Invio)")
        .default(0)
        .items(modelli)
        .interact()
        .unwrap();

    selezione // Restituisce l'indice del modello scelto
}