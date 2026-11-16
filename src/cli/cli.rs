use super::start::start;
use super::action_selector::select_action;
use super::avvia_modello::avvia_modello;
use owo_colors::OwoColorize;
use crate::libs::installer::install_model::start_install;
use super::remove_model::remove_model;
use crate::cli::remove_server::remove_server;
use super::start_install_server::start_install_server;
use crate::utils::files_manager::start_fsman;
use crate::agentic::start_agent::start_agent;

pub fn cli() {
    start();
    let _ = start_fsman();
    loop{
        let scelta = select_action();
        match scelta{
            0 =>avvia_modello(), // avvia un modello AI
            1 =>start_install(), // installa un modello AI
            2 =>remove_model(), // disinstalla un modello AI
            3 =>start_install_server(), // installa un server
            4 =>remove_server(), // disinstalla un server
            5 =>start_agent(), // avvia un agente AI
            6 =>println!("{}","exting cli...\ncya!".green()), // esci
            _ =>println!("{}","exting cli...\ncya!".green()), // esci
        }

        if scelta == 5{
            break;
        }
    }
}
