use anyhow;
use core::fmt;
use std::io::Read;
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::mpsc;
use std::thread;
use std::time::SystemTime;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct MessageHeader {
    timestamp: SystemTime, 
}

impl Default for MessageHeader {
    fn default() -> Self {
        Self {
            timestamp: SystemTime::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct EchoData {
    to_echo: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum Message {
    Start(MessageHeader),
    Echo(MessageHeader, EchoData),
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(format!("{self:?}").as_str());
        Ok(())
    }
}

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
