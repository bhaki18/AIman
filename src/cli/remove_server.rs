use crate::utils::read_servers::read_servers;
use crate::libs::uninstaller::delete_server::delete_server;
use dialoguer::{Select, theme::ColorfulTheme};

pub fn remove_server(){
    let servers = read_servers();

    let selezione = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Seleziona il modello AI da rimuovere (usa le frecce ↑/↓ e Invio)")
        .default(0)
        .items(&servers)
        .items(&["esci"])
        .interact()
        .unwrap();

    if selezione != servers.len(){// Restituisce l'indice del modello scelto
        delete_server(<std::string::String as Clone>::clone(&servers[selezione]));
    }else{
        return;
    }
}