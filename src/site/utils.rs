pub mod messageboxtemplate {

use std::{time,thread};
use std::hash::{Hash,Hasher};
use std::collections::hash_map::DefaultHasher;


use rand::{thread_rng, Rng};
use log;

    #[derive(Debug)]
    #[derive(Clone)]
    pub struct MessageBox {
        pub content: String,
        pub id: u32,
        pub instant: std::time::Instant,
        pub state: BoxState
    }

    #[derive(Debug)]
    #[derive(Clone)]
    pub enum BoxState {
        NEW = 0,
        OLD = 1,
        NEW_SIGNED = 2,
        OLD_SIGNED = 3
    }

    impl MessageBox {
        pub fn new(_content: String,_id: u32) -> MessageBox {
            log::info!("Creating new MessageBox");


            MessageBox {
                content: _content,
                id: _id,
                instant: std::time::Instant::now(),
                state: BoxState::NEW
            }
        }

        pub fn to_json(&self) -> String {
            todo!()
        }

        pub fn to_html(&self) -> String {
            let element_start = String::from("<li><div>");
            let element_end = String::from("</div></li>");

            let element_infobar = format!("<div class=\"infobar\"><h3 id=\"status\">{:#?}</h2><small id=\"id\">{}</small><button id=\"sign-button\">SIGN</button></div>",&self.state,&self.id);

            format!("{}{}<p id=\"content\">{}</p>{}",element_start,element_infobar,&self.content,element_end)
        }
    }
}



pub mod chatverse {
    use std::{sync::Mutex,sync::mpsc};
    use log;

    pub fn init(_message_box: super::messageboxtemplate::MessageBox) {
        //let (tx,rx) = mpsc::channel();

        std::thread::spawn(move || {
            loop {
                let t_handle = std::thread::spawn(move || {
                    //let message_box = &_message_box;
        
                    
                }).join().expect("Failed to start thread"); 
            }   
        });

       
        todo!()
    }

    enum ThreadLogLevel {
        LOG = 0,
        WARNING = 1,
        ERROR = 2
    }

    ///Only use this in the chatverse
    fn t_log(message: String,log_level: ThreadLogLevel) {
        match log_level {
            ThreadLogLevel::LOG => log::info!("[MESSAGEBOX METAVERSE] {}",message),
            ThreadLogLevel::WARNING => log::warn!("[MESSAGEBOX METAVERSE] {}",message),
            ThreadLogLevel::ERROR => log::error!("[MESSAGEBOX METAVERSE] {}",message),
        };
    }


    // t_ means thread bruh
}