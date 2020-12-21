use num_enum::TryFromPrimitive;

/// Represents a Character in AO.
#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub level: u8,
    pub online: bool,
    pub id: u32,
}

/// Type of group channel.
#[derive(Debug, TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum ChannelType {
    Org = 3,
    FactionLeaders = 4,
    OrgMsg = 10,
    Announcements = 12,
    Shopping = 134,
    Faction = 135,
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
#[derive(Debug)]
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
