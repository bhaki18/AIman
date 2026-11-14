use std::process::Command;
use std::env;
use std::io;

pub fn install_from_hf(){

   
    let mut model_from_hf = String::new();
    let mut quantizzazione = String::new();

    let stdin = io::stdin();

     println!("inserisci il nome del modello da installare!\nexemple: ornith-ai/Ornith-1.5-9B-GGUF");
    let _ = stdin.read_line(&mut model_from_hf);
     println!("inserisci la quantizzazione del modello da installare!\nexemple: ornith-ai/Ornith-1.5-9B-GGUF");
    let _ = stdin.read_line(&mut quantizzazione);
    let nq : u32 = quantizzazione
        .trim()
        .parse()
        .expect("Wanted a number");

    let quant = "*Q".to_owned()+ &nq.to_string() +"_K_M*";


    let destionation = env::var("HOME").unwrap() + "/.AIman/models";
    let output = Command::new("hf")
    .arg("download")
    .arg(model_from_hf.trim())
    .arg("--local-dir")
    .arg(&destionation)
    .arg("--include")
    .arg(&quant)
    .status()
    .expect("qualcosa è andato storto con il download!\nverifica il nome del modello e verifica di avere hf installato");

    if !output.success() {
        eprintln!("Download fallito!");
    } else {
        println!("Modello installato correttamente!");

        
    }
    
}