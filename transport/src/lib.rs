use std::os::unix::net::UnixListener;
use chatter_types::{ChatterMessage, MessageHeader, ChatRequestData}; 
use std::io::Read;
use std::os::unix::net::UnixStream;
use std::sync::mpsc;
use std::thread;
use std::path::Path;
use std::io::Write;

pub struct SocketListener {
    listener: UnixListener,
    send_handle: mpsc::Sender<ChatterMessage>,
}

impl SocketListener {
    pub fn new(socket: &str, send_handle: mpsc::Sender<ChatterMessage>) -> Result<Self, anyhow::Error> {
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
                Err(_) => {
                    println!("Error!");
                    break;
                }
            }
        }
    }
}

fn handle_client(send_handle: mpsc::Sender<ChatterMessage>, mut stream: UnixStream) -> Result<(), anyhow::Error> {
    let mut buf = String::new();
    stream.read_to_string(&mut buf)?;

    if let Ok(msg) = serde_json::from_str(buf.as_str()) {
        send_handle.send(msg)?;
        Ok(())
    } else {
        Err(anyhow::anyhow!("Fuck!"))
    }
}

pub struct SocketSender {
    rx: mpsc::Receiver<ChatterMessage>,
}

impl SocketSender {
    pub fn new(rx: mpsc::Receiver<ChatterMessage>) -> Result<Self, anyhow::Error> {
        
        let me = Self {
            rx,
        };

        Ok(me)
    }

    pub fn serve(&self) -> Result<(), anyhow::Error> {
        while let Ok(msg) = self.rx.recv() {
            match msg.clone() {
                ChatterMessage::ChatRequest(hdr, _) => self.send_to(hdr, msg)?,
                ChatterMessage::ChatReply(hdr, _) => self.send_to(hdr, msg)?,
            } 
        }

        Ok(())
    }

    fn send_to(&self, hdr: MessageHeader, msg: ChatterMessage) -> Result<(), anyhow::Error> {
        let socket = Path::new(hdr.destination_socket.as_str());
        let mut stream = UnixStream::connect(&socket)?;

        stream.write(serde_json::to_string(&msg)?.as_bytes())?;

        Ok(())
    }
}


