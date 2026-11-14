use super::start::start;
use super::action_selector::select_action;
use super::avvia_modello::avvia_modello;
use owo_colors::OwoColorize;
use crate::libs::installer::install_model::start_install;

pub fn cli() {
    start();
    let scelta = select_action();
    match scelta{
        0 =>avvia_modello(), // avvia un modello AI
        1 =>start_install(), // installa un modello AI
        2 =>println!("3"), // disinstalla un modello AI
        3 =>println!("4"), // installa un server
        4 =>println!("{}","exting cli...\ncya!".green()), // esci
        _ =>println!("{}","exting cli...\ncya!".green()), // esci
    }
}
