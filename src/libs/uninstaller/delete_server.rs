use std::fs;

pub fn delete_server(server_path:String){
    fs::remove_file(server_path);
}