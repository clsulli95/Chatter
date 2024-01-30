use chatter_types;
use chatter_server::chatter_server;
use chatter_client::chatter_client;
use transport;
use std::thread;

fn main() {
    let (listener_tx, listener_rx) = std::sync::mpsc::channel::<chatter_types::ChatterMessage>();
    let (sender_tx, sender_rx) = std::sync::mpsc::channel::<chatter_types::ChatterMessage>();

    let listener = transport::SocketListener::new("/home/sully/example.sock", listener_tx).unwrap();
    let sender = transport::SocketSender::new(sender_rx).unwrap();
    thread::spawn(move|| listener.serve());
    thread::spawn(move|| sender.serve());

    // We are only passing in the source/dest sock to the client knows what to pack in the
    // source/dest portions of the message header.. i dont really like this.   I also think that we
    // should find a way where we can add multiple channels (one per message type) to the
    // socket listener/sender, this way we can have multiple client types w/ a single socket listener/sender
    let chatter_client = chatter_client::ChatterClient::new("/home/sully/example.sock", "/home/sully/chatter.sock", listener_rx, sender_tx);

    let req = chatter_types::ChatRequestData { id: 1 };
    let reply = chatter_client.chat_request(req).unwrap();

    println!("Got reply! {reply:?}");
}
