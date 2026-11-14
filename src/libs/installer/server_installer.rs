use std::process::Command;
use std::env;

// #[cfg(target_os = "linux")]

pub fn install_server(server_url:String){

    let home = env::var("HOME").unwrap(); 
    let servers_dir = home.clone() + "/.AIman/servers";

    let output = Command::new("git")
    .arg("clone")
    .arg(server_url)
    .arg(servers_dir)
    .status()
    .expect("qualcosa è andato storto durante il git clone! controlla di avere git installato");
}



