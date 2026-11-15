use crate::utils::read_servers::read_servers;
use crate::libs::uninstaller::delete_server::delete_server;
use dialoguer::{Select, theme::ColorfulTheme};

pub fn remove_server(){
    let servers = read_servers();

    let selezione = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the server to remove (use ↑/↓ arrows and Enter)")
        .default(0)
        .items(&servers)
        .items(&["Exit"])
        .interact()
        .unwrap();

    if selezione != servers.len(){// Restituisce l'indice del modello scelto
        delete_server(<std::string::String as Clone>::clone(&servers[selezione]));
    }else{
        return;
    }
}