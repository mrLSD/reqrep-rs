#![allow(unused_must_use)]

extern crate nanomsg;
extern crate serde;
extern crate log4rs;
extern crate reqrep;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

use std::result;
use serde_json::{Value};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: Option<Value>,
}

fn usage() {
    println!("Usage: reqrep [client|server|device]");
    println!("  Try running several clients and servers");
    println!("  And also try killing and restarting them");
    println!("  Don't forget to start the device !");
}

struct MyServerConfig;

impl reqrep::reqrep::ServerHandler for MyServerConfig {
    fn handler(&self, msg: &Vec<u8>) -> result::Result<Vec<u8>, String> {
        debug!("reqrep.handler");
        serde_json::from_slice(&msg[..])
            .map_err(|_| format!("Failed parse JSON from message"))
            .and_then(|v: Value| {
                serde_json::from_value(v)
                    .map_err(|err| {
                        error!("reqrep.handler.serde_json::from_value: {}", err);
                        err
                    })
                    .and_then(|value: User| {
                        debug!("handler.incoming_json_value: {:?}", value);
                        let mut val: User = value;
                        val.id = 10;
                        debug!("handler.response: {:?}", val);
                        serde_json::to_vec(&val)
                            .map_err(|err| {
                                error!("reqrep.handler.serde_json::to_vec: {}", err);
                                err
                            })
                    })
                    .map_err(|err| format!("{}", err))
            })
    }
}

fn main() {
    log4rs::init_file("config.yml", Default::default()).unwrap();
    info!("Starting REQREP");
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 3 {
        return usage()
    }

    let cfg = reqrep::reqrep::ServerSettings{
        url: &*args[2].to_owned(),
        name: "broker-reqrep-test",
    };
    let handler = MyServerConfig;

    let msg = json!({"id": 123}).to_string();

    match args[1].as_ref() {
        "client" => {reqrep::reqrep::send(&cfg, &msg);},
        "server" => {reqrep::reqrep::serve(&cfg, &handler);},
        _ => usage()
    }
}
