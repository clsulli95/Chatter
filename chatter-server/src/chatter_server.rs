use anyhow::Result;
use std::{sync::mpsc, time::SystemTime};
use chatter_types::{ MessageHeader, ChatterMessage, ChatReplyData, ChatRequestData };

#[allow(unused)]
pub struct ChatterServer {
    incoming_messages: mpsc::Receiver<ChatterMessage>,
    outgoing_messages: mpsc::Sender<ChatterMessage>,
}

impl ChatterServer {
    #[allow(unused)]
    pub fn new(incoming_messages: mpsc::Receiver<ChatterMessage>, outgoing_messages: mpsc::Sender<ChatterMessage>) -> Self {
        Self {
            incoming_messages,
            outgoing_messages
        }
    }

    #[allow(unused)]
    pub fn serve(&self) {
        println!("Beginning to serve incoming rpcs..");
        while let Ok(msg) = self.incoming_messages.recv() {
            match msg {
                ChatterMessage::ChatRequest(hdr, req) => self.chat_request(hdr, req),
                _ => println!("Bad!"),
            };
        }
    }

    #[allow(unused)]
    fn chat_request(&self, hdr: MessageHeader, request: ChatRequestData) {
        println!("Chat request received!\nHeader: {hdr:?}\nRequest: {request:?}");
        let reply = ChatReplyData { id: request.id + 1 };
        let hdr = MessageHeader { timestamp: SystemTime::now(), destination_socket: hdr.source_socket, source_socket: hdr.destination_socket };
        println!("Sending Chat reply!\nHeader: {hdr:?}\nReply: {reply:?}");
        self.outgoing_messages.send(ChatterMessage::ChatReply(hdr, reply));
    }
}
