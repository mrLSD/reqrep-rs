#![allow(unused_must_use)]

extern crate nanomsg;
extern crate serde;
extern crate serde_json;
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

struct ServerSettings {
    url: String,
}

const SERVER_URL: &'static str = "tcp://0.0.0.0:3040";

pub type ServerResult = result::Result<(), nanomsg::Error>;

fn client() {
    let mut socket = Socket::new(Protocol::Req).unwrap();
    let mut endpoint = socket.connect(SERVER_URL).unwrap();
    let mut count = 1u32;

    let mut reply = String::new();

    loop {
        let request = format!("{{ \"id\": {}, \"name\": \"Test\" }}", count);

        match socket.write_all(request.as_bytes()) {
            Ok(..) => println!("CLIENT SEND '{}'.", request),
            Err(err) => {
                println!("Client failed to send request '{}'.", err);
                break
            }
        }

        match socket.read_to_string(&mut reply) {
            Ok(_) => {
                println!("CLIENT RECV '{}'.", reply);
                reply.clear()
            },
            Err(err) => {
                println!("Client failed to receive reply '{}'.", err);
                break
            }
        }
        thread::sleep(Duration::from_millis(3000));
        count += 1;
    }

    endpoint.shutdown();
}

fn server() -> ServerResult {
    let mut socket = Socket::new(Protocol::Rep)?;
    socket.bind(SERVER_URL)?;
    loop {
        let mut msg = Vec::new();
        socket.read_to_end(&mut msg)
            .map_err(|err| format!("Error to read message: {}", err))
            .and_then(|_| {
                serde_json::from_slice(&msg[..])
                    .map_err(|_| format!("Failed parse JSON from message"))
            })
            .and_then(|v: Value| {
                let mut v1: User = serde_json::from_value(v).unwrap();
                println!("ID: {} | Name: {}", v1.id, v1.name);
                v1.id = 10;
                serde_json::to_vec(&v1)
                    .map_err(|_| format!("Failed parse to JSON"))
            })
            .map(|msg| {
                socket.nb_write(&msg[..])
                    .map_err(|err| format!("Error to send message: {}", err));
            })
            .map_err(log_error);
    }
}

fn log_error(err: String) {
    println!("ERROR: {}", err);
}

fn usage() {
    println!("Usage: reqrep [client|server|device]");
    println!("  Try running several clients and servers");
    println!("  And also try killing and restarting them");
    println!("  Don't forget to start the device !");
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 3 {
        return usage()
    }

    let mut ss: ServerSettings;

    match args[1].as_ref() {
        "client" => client(),
        "server" => {
            ss.url = args[2].to_owned();
            match server() {
                Err(err) => println!("Error: {}", err),
                _ => ()
            }
        },
        _ => usage()
    }
}
