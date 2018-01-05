pub trait ServerHandler {
    fn handler(&self, msg: &Vec<u8>) -> result::Result<Vec<u8>, String>;
}

pub struct ServerSettings {
    url: String,
    name: String,
}

pub type ServerResult = result::Result<(), nanomsg::Error>;

fn send(config: &ServerSettings, message: &String) {
    let mut socket = Socket::new(Protocol::Req).unwrap();
    let mut endpoint = socket.connect(&config.url[..]).unwrap();

    let mut reply = String::new();

        match socket.write_all(message.as_bytes()) {
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
