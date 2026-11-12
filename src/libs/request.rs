use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write, stdout};

pub fn ask_llm(prompt: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let body = json!({
        "model": "default",
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ],
        "temperature": 0.7,
        "max_tokens": 512,
        "stream": true // <-- Abilita lo streaming dei token in tempo reale
    });

    let response = client
        .post("http://localhost:8080/v1/chat/completions")
        .json(&body)
        .send()?;

    if !response.status().is_success() {
        return Err(format!("Errore del server: {}", response.status()).into());
    }

    print!("AI: ");
    stdout().flush()?;

    // Leggi lo stream SSE riga per riga man mano che i token arrivano
    let reader = BufReader::new(response);
    for line in reader.lines() {
        let line = line?;

        // Ogni evento SSE inizia con "data: "
        if let Some(data) = line.strip_prefix("data: ") {
            if data.trim() == "[DONE]" {
                break;
            }

            if let Ok(res_json) = serde_json::from_str::<Value>(data) {
                // Nello streaming il testo si trova in choices[0].delta.content
                if let Some(token) = res_json["choices"][0]["delta"]["content"].as_str() {
                    print!("{}", token);
                    stdout().flush()?; // Forza la stampa immediata a schermo
                }
            }
        }
    }

    println!(); // A capo a fine risposta
    Ok(())
}