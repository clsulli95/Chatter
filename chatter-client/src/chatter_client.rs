use anyhow::Result;
use std::{sync::mpsc, time::SystemTime};
use chatter_types::{ChatterMessage, ChatRequestData, ChatReplyData, MessageHeader};

pub struct ChatterClient {
    incoming_messages: mpsc::Receiver<ChatterMessage>,
    outgoing_messages: mpsc::Sender<ChatterMessage>,
    source_socket: String,
    dest_socket: String,
}

impl ChatterClient {
    pub fn new(source_socket: &str, dest_socket: &str, incoming_messages: mpsc::Receiver<ChatterMessage>, outgoing_messages: mpsc::Sender<ChatterMessage>) -> Self {
        Self {
            incoming_messages,
            outgoing_messages, 
            source_socket: source_socket.to_string(),
            dest_socket: dest_socket.to_string(),
        }
    }

    // Here im assuming a sync interface, the reply that is received should
    // match the request that went out.  may need to investigate some
    // request/response uuid logic here.
    pub fn chat_request(&self, req: ChatRequestData) -> Result<ChatReplyData> {
        // probably populate a uuid in the header here
        let hdr = MessageHeader { timestamp: SystemTime::now(), destination_socket: self.dest_socket.clone(), source_socket: self.source_socket.clone() };
        let msg = ChatterMessage::ChatRequest(hdr, req);

        // Send out the request
        self.outgoing_messages.send(msg)?;

        // Wait for reply to come in
        // probably check uuid in header here to ensure request == reply
        // note that there is also a recv_timeout call we can use
        if let Ok(response) = self.incoming_messages.recv() {
            match response {
                ChatterMessage::ChatReply(_, reply) => return Ok(reply),
                _ => return Err(anyhow::anyhow!("Unexpected response")),
            }
        } else {
            return Err(anyhow::anyhow!("Channel error"));
        }
    }
}
