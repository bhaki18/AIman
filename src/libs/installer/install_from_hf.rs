use std::process::Command;
use std::env;
use std::io;

pub fn install_from_hf(){

   
    let mut model_from_hf = String::new();
    let mut quantizzazione = String::new();

    let stdin = io::stdin();

     println!("Enter the name of the model to install!\nExample: ornith-ai/Ornith-1.5-9B-GGUF");
    let _ = stdin.read_line(&mut model_from_hf);
     println!("Enter the quantization of the model to install!\nExample: Q4_K_M");
    let _ = stdin.read_line(&mut quantizzazione);

    let mut quant = "*".to_owned() +  &quantizzazione + ".gguf";
    quant = quant.replace("\n", "");


    let destionation = env::var("HOME").unwrap() + "/.AIman/models";
    let output = Command::new("hf")
    .arg("download")
    .arg(model_from_hf.trim())
    .arg("--local-dir")
    .arg(&destionation)
    .arg("--include")
    .arg(&quant)
    .status()
    .expect("Something went wrong with the download!\nPlease check the model name and make sure hf is installed");

    if !output.success() {
        eprintln!("Download failed!");
    } else {
        println!("Model installed successfully!");


    }
    
}