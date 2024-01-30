pub mod chatter_client;
//use chatter_types::{Message, MessageHeader, EchoData};

//fn main() {
//    println!("Hello, world!");
//
//    let socket = Path::new("/run/chatter.sock");
//
//    let mut stream = match UnixStream::connect(&socket) {
//        Err(_) => panic!("Server is not running"),
//        Ok(stream) => stream,
//    };
//
//    let msg = Message::Echo(MessageHeader::default(), EchoData { to_echo: "Fuck!".to_string() });
//
//    match stream.write(serde_json::to_string(&msg).unwrap().as_bytes()) {
//        Err(_) => panic!("couldn't send message!"),
//        Ok(_) => {}
//    }
//}
