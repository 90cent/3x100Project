pub mod messageboxtemplate {

use std::{time,thread,sync::mpsc};
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
    use std::{sync::mpsc,error::Error,time,fmt::Display,env};
    use tokio::{runtime,task};
    use colored::Colorize;
    use mongodb;

    pub enum Actions {
        GetBoxes,
        InsertBox,
        InitDatabaseTime
    }

    struct Database {
        client: mongodb::Client,
        db: mongodb::Database
    }

    impl Display for Database {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"Database Name: {}",&self.db.name())
        }
    }

    pub fn action(database_action_code: Actions) {
        let database = init().expect("Error unwraping database");
        log::info!("{}",database);
        let db = database.db;
        
        
    }

    fn init() -> Option<Database> {
        let rt = runtime::Builder::new_current_thread()
            .max_blocking_threads(4)
            .enable_all()
            .on_thread_start(|| {println!("Created Runtime")})
            .thread_name("Runtime #1")
            .on_thread_park(|| {println!("Runtime Parked")})
            .on_thread_stop(|| {println!("Runtime Stoped")})
            .thread_keep_alive(time::Duration::from_secs(10))
            .worker_threads(3)
            .build().expect("Runtime build failed");

        let (tx,rx) = mpsc::channel();
        
        let _bruh = rt.block_on(async move {
            log::info!("Init Database...");

            let mut client_options = mongodb::options::ClientOptions::parse("mongodb://192.168.178.135:27017/").await.unwrap();

            client_options.credential = Some(init_credentials("project3x100".into()));

            let client = mongodb::Client::with_options(client_options);
            let c = match client {
                Ok(c) => c,
                Err(err) => {
                    panic!("DB Client Err: {}",err.to_string())
                },
            };
            let db = c.database("project3x100");

            let database_tulp = (c,db);
            tx.send(database_tulp).unwrap()
        });

        match rx.recv() {
            Ok(k) => Some(Database { client: k.0, db: k.1 }),
            Err(_) => None,
        }
    }

    fn init_credentials(source: String) -> mongodb::options::Credential {
        let (username,password) = {
            match env::var("AUTH3x100") {
                Ok(k) => {
                    let split: Vec<_> = k.split(':').collect();
                    let censored_pw = {
                        let mut s: String = String::new();
                        for i in split[1].chars() {
                            s += "*";
                        };
                        s
                    };

                    log::info!("AUTH:\n ->  USER:{}\n ->  PASSWORD:{}",split[0],censored_pw);

                    (String::from(split[0]),String::from(split[1]))
                },
                Err(_) => {
                    println!("{}","Please define the 3x100AUTH Enviroment".red());
                    (String::from("ERROR"),String::from("ERROR"))
                },
            }
        };


        mongodb::options::Credential::builder()
            .username(username)
            .password(password)
            .source(source)
            .build()
    }
}

