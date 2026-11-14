use crate::libs::installer::select_method::select_install_method;
use crate::libs::installer::install_from_hf::install_from_hf;

pub fn start_install(){
    let method = select_install_method();
    match method{
        0 => install_from_hf(),
        1 => return,
        _ => return,
    }
}