use std::fs;
use owo_colors::OwoColorize;

pub fn create_path(path: String){
    let _ = fs::create_dir_all(&path);
    println!("creata la path: {}",path.blue());
}
