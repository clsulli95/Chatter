use std::io::Read;
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::mpsc;
use std::thread;
use chatter_types::Message;

struct SocketListener {
    listener: UnixListener,
    send_handle: mpsc::Sender<Message>,
}

impl SocketListener {
    pub fn new(socket: &str, send_handle: mpsc::Sender<Message>) -> Result<Self, anyhow::Error> {
        let _ = std::fs::remove_file(socket);
        let me = Self {
            listener: UnixListener::bind(socket)?,
            send_handle,
        };

        Ok(me)
    }

    pub fn serve(&self) {
        for stream in self.listener.incoming() {

            let handle = self.send_handle.clone();

            match stream {
                Ok(stream) => {
                    thread::spawn(move|| handle_client(handle, stream));
                }
                Err(err) => {
                    println!("Error!");
                    break;
                }
        }
        }
    }
}

fn handle_client(send_handle: mpsc::Sender<Message>, mut stream: UnixStream) -> Result<(), anyhow::Error> {
    let mut buf = String::new();
    stream.read_to_string(&mut buf)?;

    if let Ok(msg) = serde_json::from_str(buf.as_str()) {
        send_handle.send(msg)?;
        Ok(())
    } else {
        Err(anyhow::anyhow!("Fuck!"))
    }
}

fn main() -> Result<(), anyhow::Error> {
    println!("Hello, world!");
    let (tx, rx) = mpsc::channel::<Message>();
    let listener = SocketListener::new("/run/chatter.sock", tx)?;
    thread::spawn(move|| listener.serve());

    while let Ok(msg) = rx.recv() {
        println!("{msg}");
    }

    Ok(())
}
