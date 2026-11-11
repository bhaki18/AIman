
// use std::io;
// use std::fs;
use crate::utils::files_manager::look_for_models;

pub fn cli() {
    // let mut input = String::new();

    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Errore nella lettura");

    // println!("Hai scritto: {}", input);
    start();
    look_for_models();
}

fn start(){
    println!("Starting CLI,wait...");
}