extern crate simple_logger;
#[macro_use]
extern crate log;
extern crate base64;
extern crate bincode;
extern crate image;
extern crate qrcode;
extern crate reqwest;
extern crate whatsappweb;

use std::fs::{File, OpenOptions};

use image::Luma;

use whatsappweb::connection;
use whatsappweb::connection::{
    DisconnectReason, PersistentSession, State, UserData, WhatsappWebConnection, WhatsappWebHandler,
};
use whatsappweb::message::ChatMessage;
use whatsappweb::models::Jid;

const SESSION_FILENAME: &str = "session.bin";

struct Handler {
    subscribed_jid: Jid,
}

impl WhatsappWebHandler for Handler {
    fn on_state_changed(&self, connection: &WhatsappWebConnection<Handler>, state: State) {
        info!("new state: {:?}", state);
        if state == State::Connected {
            connection.subscribe_presence(&self.subscribed_jid);
        }
    }

    fn on_persistent_session_data_changed(&self, persistent_session: PersistentSession) {
        bincode::serialize_into(
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(SESSION_FILENAME)
                .unwrap(),
            &persistent_session,
        )
        .unwrap();
    }
    fn on_user_data_changed(&self, _: &WhatsappWebConnection<Handler>, user_data: UserData) {
        if let UserData::PresenceChange(jid, status, _) = user_data {
            if jid == self.subscribed_jid {
                info!("{} is now {:?}", jid.phonenumber().unwrap(), status);
            }
        }
    }
    fn on_disconnect(&self, _: DisconnectReason) {
        info!("disconnected");
    }
    fn on_message(&self, _: &WhatsappWebConnection<Handler>, _: bool, _: Box<ChatMessage>) {}
}

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let handler = Handler {
        subscribed_jid: Jid::from_phonenumber("+49123456789".to_string()).unwrap(),
    };

    if let Ok(file) = File::open(SESSION_FILENAME) {
        let (_, join_handle) =
            connection::with_persistent_session(bincode::deserialize_from(file).unwrap(), handler);
        join_handle.join().unwrap();
    } else {
        let (_, join_handle) = connection::new(
            |qr| {
                qr.render::<Luma<u8>>()
                    .module_dimensions(10, 10)
                    .build()
                    .save("login_qr.png")
                    .unwrap();
            },
            handler,
        );
        join_handle.join().unwrap();
    }
}
