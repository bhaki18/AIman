use std::process::Command;
use std::env;
use url::Url;

// #[cfg(target_os = "linux")]

pub fn install_server(server_url:String){

    let home = env::var("HOME").unwrap(); 
    let servers_dir = home.clone() + "/.AIman/servers/";
    println!("{}",server_url);

    let url = Url::parse(&server_url).unwrap();

    let filename = url.path_segments()
        .unwrap()
        .last()
        .unwrap();

    let output = Command::new("curl")
    .arg("-L")
    .arg(&server_url)
    .arg("-o")
    .arg(servers_dir + filename)
    .status()
    .expect("qualcosa è andato storto durante il curl!");
}



