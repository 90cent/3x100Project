use std::{vec::Vec};
use std::io::Error;

use rocket_contrib::json::Json;
use serde::Deserialize;
use log::{warn,info,trace};

#[derive(Deserialize)]
pub struct ChatMessage {
    content: String
}

#[post("/chat", format="json", data = "<message>")]
pub fn chat(message: Json<ChatMessage>) -> Result<String,Error> {
    warn!("[/CHAT BACKEND] NEW POST REQUEST\nContent:{}",message.content);

    
    

    //log::info!("[-- EPIC TESTTING --] {:#?}",rx.recv().unwrap());

    Ok(String::from("ok bruh"))
}