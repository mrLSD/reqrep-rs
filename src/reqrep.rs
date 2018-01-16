use nanomsg;
use nanomsg::{Socket, Protocol};
use std::io::{Read, Write};
use std::result;
use std::str;

pub struct ServerSettings<'a> {
    pub url: &'a str,
    pub name: &'a str,
}

// Result type for Serve
pub type ServeResult = result::Result<(), nanomsg::Error>;
// Result type for Send
pub type SendResult = result::Result<String, nanomsg::Error>;

pub type HandlerResult = result::Result<Vec<u8>, String>;

// Handler for server response
pub trait ServerHandler {
    fn handler(&self, msg: &Vec<u8>) -> HandlerResult;
}

// Send event message
pub fn send(config: &ServerSettings, message: &String) -> SendResult {
    let mut socket = Socket::new(Protocol::Req).unwrap();
    let mut endpoint = socket.connect(&config.url[..]).unwrap();
    let mut reply = String::new();

    debug!("send.socket.write_all: {}", message);

    socket.write_all(message.as_bytes())
        .map_err(|err| { error!("send.socket.write_all: {}", err); err })
        .and_then(|_| {
            socket.read_to_string(&mut reply)
                .map_err(|err| { error!("send.socket.read_to_string: {}", err); err })
        })
        .map_err(|err| {
            let _ = endpoint.shutdown()
                .map_err(|err| error!("send.endpoint.shutdown: {}", err) );
            err
        })?;

    debug!("send.reply: {}", reply);

    let _ = endpoint.shutdown()
        .map_err(|err| error!("send.endpoint.shutdown: {}", err) );
    return Ok(reply);
}

// Serve event messages
pub fn serve<T: ServerHandler>(config: &ServerSettings, h: &T) -> ServeResult {
    let mut socket = Socket::new(Protocol::Rep)
        .map_err(|err| {error!("serve.Socket::new: {}", err); err })?;
    socket.bind(&config.url[..])
        .map_err(|err| {error!("serve.socket.bind: {}", err); err })?;

    loop {
        let mut msg = Vec::new();
        let _ = socket.read_to_end(&mut msg)
            .map_err(|err| error!("serve.socket.read_to_end: {}", err))
            .and_then(|_| {
                debug!("serve.socket.read_to_end: {}", str::from_utf8(&msg).unwrap());
                h.handler(&msg)
                    .map_err(|err| error!("serve.handler: {}", err))
            })
            .map(|msg| {
                debug!("serve.socket.nb_write: {}", str::from_utf8(&msg).unwrap());
                socket.nb_write(&msg[..])
                    .map_err(|err| error!("serve.socket.nb_write: {}", err))
            });
    }
}
