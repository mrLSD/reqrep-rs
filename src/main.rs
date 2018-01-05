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

trait ServerHandler {
    fn handler(&self, msg: &Vec<u8>) -> result::Result<Vec<u8>, String>;
}

struct ServerSettings {
    url: String,
}

pub type ServerResult = result::Result<(), nanomsg::Error>;

fn client(config: &ServerSettings) {
    let mut socket = Socket::new(Protocol::Req).unwrap();
    let mut endpoint = socket.connect(&config.url[..]).unwrap();
    let mut count = 1u32;

    let mut reply = String::new();

    loop {
        let request = format!("{{ \"id\": {}, \"name\": \"Test\" }}", count);

        match socket.write_all(request.as_bytes()) {
            Ok(..) => println!("CLIENT SEND '{}'.", request),
            Err(err) => {
                log_error("client.socket.write_all", err);
                break
            }
        }

        match socket.read_to_string(&mut reply) {
            Ok(_) => {
                println!("CLIENT RECV '{}'.", reply);
                reply.clear()
            },
            Err(err) => {
                log_error("client.socket.read_to_string", err);
                break
            }
        }
        thread::sleep(Duration::from_millis(3000));
        count += 1;
    }

    endpoint.shutdown();
}

fn serve<T: ServerHandler>(config: &ServerSettings, h: &T) -> ServerResult {
    let mut socket = Socket::new(Protocol::Rep)?;
    socket.bind(&config.url[..])?;
    loop {
        let mut msg = Vec::new();
        socket.read_to_end(&mut msg)
            .map_err(|err| log_error("serve.socket.read_to_end", err))
            .and_then(|_| {
                h.handler(&msg)
                    .map_err(|err| log_error("serve.handler", err))
            })
            .map(|msg| {
                socket.nb_write(&msg[..])
                    .map_err(|err| log_error("serve.socket.nb_write", err));
            });
    }
}

fn log_error<T:std::fmt::Display>(event_name: &'static str, err: T) {
    println!("[ERROR] {}: {}", event_name, err);
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
    reqrep::ltst::test_log();
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
