use crate::utils::files_manager::start_fsman;
use super::scegli_modello::scegli_modello;
use crate::libs::find_model::find_model;

pub fn avvia_modello(){

    let  models = start_fsman();
    let mut index = scegli_modello(&models);
    let model_infos = find_model(<std::string::String as Clone>::clone(&models[index]));



}