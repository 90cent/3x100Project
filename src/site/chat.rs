use std::{vec::Vec,io::{self, Write},fs};
use std::thread;
use std::io::Error;

use rocket_contrib::json::Json;
use serde::{Deserialize,Serialize};
use log::{warn,info,trace, log};
use rand::{self, Rng};
use chrono;

use super::utils::messageboxtemplate::MessageBox;

#[derive(Deserialize)]
#[derive(Serialize)]
pub struct ChatMessage {
    content: String
}

#[post("/chat", format="json", data = "<message>")]
pub fn chat(message: Json<ChatMessage>) -> Result<String,Error> {
    warn!("[/CHAT BACKEND] NEW POST REQUEST\nContent:{}",message.content);

    create_message(MessageBox::new(message.content.to_owned(),rand::thread_rng().gen_range(1000..9999)));

    //log::info!("[-- EPIC TESTTING --] {:#?}",rx.recv().unwrap());

    Ok(String::from("ok bruh"))
}

fn create_message(_box: MessageBox) {
    let json = _box.to_json();
    let filename = format!("{:?}-{}",chrono::Utc::now(),rand::thread_rng().gen::<u32>());
    let path = format!("boxes/{}",filename);

    let mut file = match fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(path) {
            Ok(file) => Some(file),
            Err(_) => None,
        };
        
    if file.is_some() {
        let mut f = file.unwrap();

        f.write_fmt(format_args!("{}",_box.to_json())).unwrap();
        f.sync_all().unwrap();

        
    }
}