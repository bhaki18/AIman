use start::start;
use action_selector::select_action;

pub fn cli() {
    start();
    let mut scelta = action_selector();
    match scelta{
        0 =>, // avvia un modello AI
        1 =>, // installa un modello AI
        2 =>, // disinstalla un modello AI
        3 =>, // installa un server
        4 =>, // esci
        _ =>, // esci
    }
}
