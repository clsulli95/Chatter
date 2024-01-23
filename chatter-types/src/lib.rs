use core::fmt;
use std::time::SystemTime;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageHeader {
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
pub struct EchoData {
    pub to_echo: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Start(MessageHeader),
    Echo(MessageHeader, EchoData),
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(format!("{self:?}").as_str())?;
        Ok(())
    }
}
