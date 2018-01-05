#![allow(unused_must_use)]

extern crate nanomsg;
extern crate serde;
extern crate serde_json;
extern crate log4rs;
extern crate reqrep;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

use nanomsg::{Socket, Protocol};

use std::thread;
use std::result;
use std::time::Duration;
use std::io::{Read, Write};
use serde_json::{Value};

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: Value,
}

fn usage() {
    println!("Usage: reqrep [client|server|device]");
    println!("  Try running several clients and servers");
    println!("  And also try killing and restarting them");
    println!("  Don't forget to start the device !");
}

struct MyServerConfig;

impl ServerHandler for MyServerConfig {
    fn handler(&self, msg: &Vec<u8>) -> result::Result<Vec<u8>, String> {
        serde_json::from_slice(&msg[..])
            .map_err(|_| format!("Failed parse JSON from message"))
            .and_then(|v: Value| {
                let mut v1: User = serde_json::from_value(v).unwrap();
                println!("ID: {} | Name: {}", v1.id, v1.name);
                v1.id = 10;
                serde_json::to_vec(&v1)
                    .map_err(|_| format!("Failed parse to JSON"))
            })
    }
}

fn main() {
    log4rs::init_file("config.yml", Default::default()).unwrap();
    reqrep::reqrep::test_log();
    info!("booting up");
    info!(target: "app::main", "booting up {}", 100);
    warn!(target: "app::main", "booting up {}", 100);
    warn!(target: "input_events", "booting up {}", 100);
    /*let args: Vec<_> = std::env::args().collect();

    if args.len() < 3 {
        return usage()
    }

    let cfg = ServerSettings{
        url: args[2].to_owned(),
    };
    let handler = MyServerConfig;

    match args[1].as_ref() {
        "client" => client(&cfg),
        "server" => {
            match serve(&cfg, &handler) {
                Err(err) => println!("Error: {}", err),
                _ => ()
            }
        },
        _ => usage()
    }*/
}
