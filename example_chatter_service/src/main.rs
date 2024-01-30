use chatter_types;
use chatter_server::chatter_server;
use chatter_client;
use transport;
use std::thread;

fn main() {
    let (listener_tx, listener_rx) = std::sync::mpsc::channel::<chatter_types::ChatterMessage>();
    let (sender_tx, sender_rx) = std::sync::mpsc::channel::<chatter_types::ChatterMessage>();

    let listener = transport::SocketListener::new("/home/sully/chatter.sock", listener_tx).unwrap();
    let sender = transport::SocketSender::new(sender_rx).unwrap();
    thread::spawn(move|| listener.serve());
    thread::spawn(move|| sender.serve());

    println!("Starting Chatter RPC Server!");
    let rpc_server = chatter_server::ChatterServer::new(listener_rx, sender_tx);
    thread::spawn(move|| rpc_server.serve());

    loop {

    }
}
