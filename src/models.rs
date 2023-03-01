use std::convert::TryFrom;

use crate::error::Error;

/// Represents a Character in AO.
#[derive(Debug, Clone)]
pub struct Character {
    pub name: String,
    pub level: u8,
    pub online: bool,
    pub id: u32,
}

/// Type of group channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChannelType {
    Org = 3,
    #[cfg(not(feature = "test-server"))]
    FactionLeaders = 4,
    #[cfg(feature = "test-server")]
    Global = 4,
    OrgMsg = 10,
    #[cfg(feature = "test-server")]
    Playfield = 11,
    Announcements = 12,
    Shopping = 134,
    Faction = 135,
}

impl TryFrom<u8> for ChannelType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(Self::Org),
            #[cfg(not(feature = "test-server"))]
            4 => Ok(Self::FactionLeaders),
            #[cfg(feature = "test-server")]
            4 => Ok(Self::Global),
            10 => Ok(Self::OrgMsg),
            #[cfg(feature = "test-server")]
            11 => Ok(Self::Playfield),
            12 => Ok(Self::Announcements),
            134 => Ok(Self::Shopping),
            135 => Ok(Self::Faction),
            _ => Err(Error::UnknownChannelType(value)),
        }
    }
}

/// A group channel in the game.
#[derive(Debug, Clone)]
pub struct Group {
    pub name: Option<String>,
    pub id: u32,
    pub r#type: ChannelType,
    pub status: Option<u32>,
}

/// A channel that messages can be sent to or received from.
#[derive(Debug, Clone)]
pub enum Channel {
    Group(Group),
    PrivateChannel(u32),
    Tell(u32),
    Vicinity,
}

/// Represents a message in AO.
#[derive(Debug, Clone)]
pub struct Message {
    pub sender: Option<u32>,
    pub channel: Channel,
    pub text: String,
    pub send_tag: String,
}

/// Represents a chat notice.
#[derive(Debug)]
pub struct ChatNotice {
    pub sender: u32,
    pub text: String,
}
