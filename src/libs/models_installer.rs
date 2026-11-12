use reqwest::blocking::Client;
use std::fs::File;
use std::env;
use std::io::copy;
use indicatif::{ProgressBar, ProgressStyle};

pub fn install_model(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut response = client.get(&url).send()?;

    // Verifica che l'URL risponda con successo (200 OK) e non con 404
    if !response.status().is_success() {
        return Err(format!("Errore HTTP {}: {}", response.status(), url).into());
    }

    let total_size = response.content_length().unwrap_or(0);

    // Configura la barra di avanzamento
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"),
    );

    let home = env::var("HOME").unwrap(); 
    let file_name = url.rsplit('/').next().unwrap_or("modello.gguf");
    let model_path = format!("{}/.AIman/models/{}", home, file_name);
    let mut file = File::create(model_path)?;

    // wrap_read avanza la barra automaticamente durante la copia
    let mut reader = pb.wrap_read(&mut response);
    copy(&mut reader, &mut file)?;

    pb.finish_with_message("Download completato!");
    Ok(())
}