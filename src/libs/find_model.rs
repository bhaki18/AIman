use std::fs;
use std::env; 
extern crate json;

pub fn find_model(nome: String)-> Vec<String>{
    let home = env::var("HOME").unwrap(); 
    let models_config_dir = home.clone() + "/.AIman/models_config/";
    let servers_dir = home.clone() + "/.AIman/servers";
    let mut model_infos = Vec::new(); // [0] nome del modello.[1] path del modello in models
                                  // [2] path del server del modello.[3] famiglia del modello 
    model_infos.push(nome.clone());       // [4] tipo di modello

    let model_conf = models_config_dir + &nome + ".json";
    let json = fs::read_to_string(model_conf).unwrap(); // stringa json 
    let obj_conf_model = json::parse(&json).unwrap(); // oggetto json

    model_infos.push(obj_conf_model["path_modello"].to_string());
    model_infos.push(obj_conf_model["server_utilizzato"].to_string());
    model_infos.push(obj_conf_model["famiglia_modello"].to_string());
    model_infos.push(obj_conf_model["tipo_di_modello"].to_string());


    return model_infos;
    

// ## struttura di un config.json

//     {
//         "nome_modello":"ai",
//         "path_modello":".AIman/models/ai.gguf",
//         "famiglia_modello":"qwen",
//         "tipo_di_modello":"llm",
//         "repo_hf":"https://huggingface.co/company/ai.gguf",
//         "server_utilizzato":".AIman/server/mistralrs.bin",
//         "quantizzazione":"16bits"
//     }

    

}