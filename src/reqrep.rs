use nanomsg;
use nanomsg::{Socket, Protocol};
use std::io::{Read, Write};
use std::result;

pub struct ServerSettings<'a> {
    pub url: &'a str,
    pub name: &'a str,
}

pub type ServerResult = result::Result<(), nanomsg::Error>;
pub type ClientResult = result::Result<String, nanomsg::Error>;

pub trait ServerHandler {
    fn handler(&self, msg: &Vec<u8>) -> result::Result<Vec<u8>, String>;
}

pub fn send(config: &ServerSettings, message: &String) -> ClientResult {
    let mut socket = Socket::new(Protocol::Req).unwrap();
    let mut endpoint = socket.connect(&config.url[..]).unwrap();

    let mut reply = String::new();

    socket.write_all(message.as_bytes())
        .map_err(|err| { error!("socket.write_all: {}", err); err })
        .and_then(|_| {
            socket.read_to_string(&mut reply)
                .map_err(|err| { error!("socket.read_to_string: {}", err); err })
        })
        .map_err(|err| { let _ = endpoint.shutdown(); err })?;

    let _ = endpoint.shutdown();
    return Ok(reply);
}

pub fn serve<T: ServerHandler>(config: &ServerSettings, h: &T) -> ServerResult {
    let mut socket = Socket::new(Protocol::Rep)?;
    socket.bind(&config.url[..])?;
    loop {
        let mut msg = Vec::new();
        let _ = socket.read_to_end(&mut msg)
            .map_err(|err| error!("serve.socket.read_to_end: {}", err))
            .and_then(|_| {
                h.handler(&msg)
                    .map_err(|err| error!("serve.handler: {}", err))
            })
            .map(|msg| {
                socket.nb_write(&msg[..])
                    .map_err(|err| error!("serve.socket.nb_write: {}", err))
            });
    }
}
