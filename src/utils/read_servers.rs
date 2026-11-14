use std::fs;
use std::env;

pub fn read_servers() -> Vec<String>{
    let home = env::var("HOME").unwrap();
    let servers_dir = home.clone() + "/.AIman/servers";

    let mut servers = Vec::new();

    for file in fs::read_dir(servers_dir).unwrap(){
        servers.push(file.as_ref().unwrap().path().display().to_string());
    }
    return servers;
}