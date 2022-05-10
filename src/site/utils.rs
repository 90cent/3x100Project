pub mod messageboxtemplate {

use std::{time,thread};
use std::hash::{Hash,Hasher};
use std::collections::hash_map::DefaultHasher;

use rand::{thread_rng, Rng};
use log;

use serde::{Deserialize,Serialize};
use serde_json;

    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Deserialize)]
    #[derive(Serialize)]
    pub struct MessageBox {
        pub content: String,
        pub id: u32,
        pub state: BoxState
    }

    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Deserialize)]
    #[derive(Serialize)]
    pub enum BoxState {
        NEW = 0,
        OLD = 1,
        NEW_SIGNED = 2,
        OLD_SIGNED = 3
    }

    impl MessageBox {
        pub fn new(_content: String,_id: u32) -> MessageBox {
            log::info!("Creating new MessageBox with id {}",_id);


            MessageBox {
                content: _content,
                id: _id,
                state: BoxState::NEW
            }
        }

        pub fn to_json(&self) -> String {
            match serde_json::to_string(self) {
                Ok(k) => k,
                Err(_) => String::from("failed bruh"),
            }
        }

        pub fn to_html(&self) -> String {
            let element_start = String::from("<li><div>");
            let element_end = String::from("</div></li>");

            let element_infobar_sign = format!("<button id=\"sign-button\" onclick='alert(\"{}\");'>SIGN</button","The Sign button is currently W.I.P. So it wont work. A little info about the sign button: The Sign button can be used to buy Message Boxes using Metamask, these will show up at the top");
            let element_infobar = format!("<div class=\"infobar\"><h3 id=\"status\">{:#?}</h2><small id=\"id\">{}</small>{}></div>",&self.state,&self.id,element_infobar_sign);

            format!("{}{}<p id=\"content\">{}</p>{}",element_start,element_infobar,&self.content,element_end)
        }
    }
}

pub mod database {
    use std::error::Error;
    use tokio::{runtime,task};
    use mongodb;

    pub enum Actions {
        GetBoxes,
        InsertBox,
        InitDatabaseTime
    }

    pub fn action(database_action_code: Actions) {
        let rt = runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .on_thread_start(|| {println!("Created Runtime")})
            .thread_name("Runtime #1")
            .build().expect("Runtime build failed");

        rt.spawn_blocking(async move || {
            let mut client_options = mongodb::options::ClientOptions::parse("mongodb://192.168.178.135:27017").await.unwrap();
            client_options.app_name = Some("project3x100".into());

            let client = mongodb::Client::with_options(client_options).expect("Failed");

            let c = client;
                let databases = c.list_database_names(None, None);

                println!("{:#?}",databases.await.unwrap());
        });
        
    }




}