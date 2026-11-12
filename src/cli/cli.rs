use super::start::start;
use super::action_selector::select_action;

pub fn cli() {
    start();
    let mut scelta = select_action();
    match scelta{
        0 =>println!("1"), // avvia un modello AI
        1 =>println!("2"), // installa un modello AI
        2 =>println!("3"), // disinstalla un modello AI
        3 =>println!("4"), // installa un server
        4 =>println!("5"), // esci
        _ =>println!("6"), // esci
    }
}
