use std::fs;

pub fn delete_server(server_path:String){
    let _ = fs::remove_file(server_path);
}