mod actions;

extern crate telegram_bot;
//use self::telegram_bot::*;

pub fn resolve (command: &str) -> String {
    let mut acao = command;
    if command.to_lowercase().contains("dara é doida"){
        acao = "/daraedoida";
    }else if command.to_lowercase().contains("alana"){
        acao = "/alanaacha";
    }else if command.to_lowercase().contains("eita"){
        acao = "/valeime";
    }

    match acao {
    "/daraedoida" => actions::berro::run(),
    "/alanaacha" => actions::daraedoida::run(),
    "/valeime" => actions::valeime::run(),
    _ =>  "".to_string(),
    }
}
