use std::str::FromStr;

use crate::{Error, Result};

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Jid {
    pub id: String,
    pub is_group: bool,
}

/// Jid used to identify either a group or an individual
impl Jid {
    pub fn to_string(&self) -> String {
        self.id.to_string() + if self.is_group { "@g.us" } else { "@c.us" }
    }

    /// If the Jid is from an individual return the international phonenumber, else None
    pub fn phonenumber(&self) -> Option<String> {
        if !self.is_group {
            Some("+".to_string() + &self.id)
        } else {
            None
        }
    }

    pub fn from_phonenumber(mut phonenumber: String) -> Result<Jid> {
        if phonenumber.starts_with('+') {
            phonenumber.remove(0);
        }

        if phonenumber.chars().any(|c| !c.is_digit(10)) {
            return Err("not a valid phonenumber".into());
        }

        Ok(Jid {
            id: phonenumber,
            is_group: false,
        })
    }
}

impl FromStr for Jid {
    type Err = Error;

    fn from_str(jid: &str) -> Result<Jid> {
        let at = jid.find('@').ok_or("jid missing @")?;

        let (id, surfix) = jid.split_at(at);
        Ok(Jid {
            id: id.to_string(),
            is_group: match surfix {
                "@c.us" => false,
                "@g.us" => true,
                "@s.whatsapp.net" => false,
                "@broadcast" => false, //TODO
                _ => return Err("invalid surfix".into()),
            },
        })
    }
}
