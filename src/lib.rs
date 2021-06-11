extern crate simple_logger;
extern crate ws;
#[macro_use]
extern crate log;
extern crate url;
#[macro_use]
extern crate json;
extern crate base64;
extern crate image;
extern crate qrcode;
extern crate ring;
extern crate serde;
extern crate untrusted;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate byteorder;
extern crate chrono;
extern crate protobuf;
#[macro_use]
extern crate error_chain;
#[cfg(feature = "media")]
extern crate reqwest;

use crate::errors::*;
use crate::models::Jid;

pub mod connection;
pub mod crypto;
pub mod errors;
mod json_protocol;
#[cfg(feature = "media")]
pub mod media;
pub mod message;
mod message_wire;
pub mod models;
mod node_protocol;
mod node_wire;
mod timeout;
mod websocket_protocol;

#[derive(Debug)]
pub struct Contact {
    ///name used in phonebook, set by user
    pub name: Option<String>,
    ///name used in pushnotification, set by opposite peer
    pub notify: Option<String>,
    pub jid: Jid,
}

#[derive(Debug, Copy, Clone)]
pub enum PresenceStatus {
    Unavailable,
    Available,
    Typing,
    Recording,
}

#[derive(Debug)]
pub struct GroupMetadata {
    pub creation_time: i64,
    pub id: Jid,
    pub owner: Option<Jid>,
    pub participants: Vec<(Jid, bool)>,
    pub subject: String,
    pub subject_owner: Jid,
    pub subject_time: i64,
}

#[derive(Debug, Copy, Clone)]
pub enum GroupParticipantsChange {
    Add,
    Remove,
    Promote,
    Demote,
}

#[derive(Debug, Copy, Clone)]
pub enum ChatAction {
    Add,
    Remove,
    Archive,
    Unarchive,
    Clear,
    Pin(i64),
    Unpin,
    Mute(i64),
    Unmute,
    Read,
    Unread,
}

#[derive(Copy, Clone)]
pub enum MediaType {
    Image,
    Video,
    Audio,
    Document,
}
