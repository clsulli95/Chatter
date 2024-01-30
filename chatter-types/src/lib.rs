use core::fmt;
use std::time::SystemTime;
use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageHeader {
    pub timestamp: SystemTime,
    pub destination_socket: String,
    pub source_socket: String,
}

#[allow(unused)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatRequestData {
    pub id: usize,
}

#[allow(unused)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatReplyData {
    pub id: usize,
}

#[allow(unused)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ChatterMessage {
    ChatRequest(MessageHeader, ChatRequestData),
    ChatReply(MessageHeader, ChatReplyData),
}

impl fmt::Display for ChatterMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(format!("{self:?}").as_str())?;
        Ok(())
    }
}
