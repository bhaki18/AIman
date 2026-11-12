use reqwest;
use owo_colors::OwoColorize;

pub fn check_url_model(url: &String) -> bool{

    if url.starts_with("https://huggingface.co"){
        let response = reqwest::blocking::get(
            url
        ).unwrap();

        if response.status() == reqwest::StatusCode::OK{
            return true;
        }else{
            return false;
        }
    }else{
        println!("{}","url non riconosciuto! assicurati che l'url inizi con \"https://huggingface.co\"".red());
        return false;
    }

    
}

pub fn install_model(url: String){
    
    if check_url_model(&url){
        let response = reqwest::blocking::get(
        url
    ).unwrap();

    let body = response.text().unwrap();


    }

}

pub fn install_server(){

}