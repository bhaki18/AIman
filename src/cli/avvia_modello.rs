use crate::utils::files_manager::start_fsman;
use super::scegli_modello::scegli_modello;

pub fn avvia_modello(){

    let mut models = start_fsman();
    let mut index = scegli_modello(&models);
    

}