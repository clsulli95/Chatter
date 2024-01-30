pub mod chatter_server;

//use std::sync::mpsc;
//use std::thread;
//use chatter_types::Message;
//use crate::socket_listener::SocketListener;
//
//fn main() -> Result<(), anyhow::Error> {
//    println!("Hello, world!");
//    let (tx, rx) = mpsc::channel::<Message>();
//    let listener = SocketListener::new("/run/chatter.sock", tx)?;
//    thread::spawn(move|| listener.serve());
//
//    while let Ok(msg) = rx.recv() {
//        println!("{msg}");
//    }
//
//    Ok(())
//}
