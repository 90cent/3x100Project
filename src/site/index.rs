use super::utils::{messageboxtemplate,database};
use crate::site::Template;

use std::io::Result;
use std::{thread};
use std::{io,fs};

use rand::{self, Rng};
use serde_json;

#[derive(serde::Serialize)]
struct TestContext {
    boxes: String,
    title: String,
} 

#[get("/")]
pub fn index() -> Template {
    let mut message_buf = String::new();

    let html_message_boxes = thread::spawn(move || {
        let dir = fs::read_dir("boxes").unwrap();
        for box_file in dir{
            let f = match box_file {
                Ok(entry) => entry,
                Err(_) => panic!("boxfile error"),
            };
            
            let box_file_content = fs::read_to_string(f.path()).expect("couldnt read to string");

            let mut _box: messageboxtemplate::MessageBox = serde_json::from_str(&*box_file_content).unwrap();
            
     

            let old_message_buf = message_buf.to_owned();
            message_buf = _box.to_html();

            message_buf.push_str(&*old_message_buf);
        }

        return message_buf;
    });
    

    let titles = vec!["3x100","100³".into(),"100x3","100+100+100","25*12","1+299"];
    let _title = titles[rand::thread_rng().gen_range(0..=titles.len()-1)];


    let context = TestContext {boxes: html_message_boxes.join().expect("not gud"),title: _title.into()};
    
    database::action(database::Actions::GetBoxes);

    Template::render("index", &context)
}


