use super::Jid;

#[derive(Debug, Serialize)]
pub struct Chat {
    pub name: Option<String>,
    pub jid: Jid,
    pub last_activity: i64,
    pub pin_time: Option<i64>,
    pub mute_until: Option<i64>,
    pub spam: Option<bool>,
    pub read_only: bool,
}
