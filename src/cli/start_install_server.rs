use crate::libs::installer::select_server::select_server;
use crate::libs::installer::server_installer::install_server;


pub fn start_install_server(){
    let scelta = select_server();


    if scelta != ""{
        install_server(scelta);
    }else{
        return;
    }
}