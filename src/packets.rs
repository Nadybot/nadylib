#[cfg(feature = "mmdb")]
use crate::mmdb;
use crate::{
    error::{Error, Result},
    models::{Channel, ChannelType, Character, ChatNotice, Group, Message},
};

use byteorder::{ByteOrder, NetworkEndian};
use num_enum::TryFromPrimitive;

use std::{convert::TryFrom, result::Result as OrigResult};

/// The maximum unsigned 32-bit integer, used to check if character lookup failed.
const MAXINT: u32 = 4294967295;

fn read_u8(data: &mut &[u8]) -> u8 {
    let val = data[0];
    *data = &data[1..];
    val
}

fn read_u32(data: &mut &[u8]) -> u32 {
    let val = NetworkEndian::read_u32(&data[..4]);
    *data = &data[4..];
    val
}

fn read_integer_array(data: &mut &[u8]) -> Vec<u32> {
    let n = NetworkEndian::read_u16(&data) as usize;
    let mut buf = vec![0; n];
    NetworkEndian::read_u32_into(&data[2..2 + 4 * n], &mut buf);
    *data = &data[2 + 4 * n..];
    buf
}

fn read_string(data: &mut &[u8]) -> String {
    let n = NetworkEndian::read_u16(&data) as usize;
    let raw = &data[2..n + 2];
    *data = &data[n + 2..];
    String::from_utf8(raw.to_vec()).unwrap()
}

fn read_string_array(data: &mut &[u8]) -> Vec<String> {
    let n = NetworkEndian::read_u16(&data) as usize;
    *data = &data[2..];
    let mut buf = Vec::with_capacity(n);
    for _ in 0..n {
        let slen = NetworkEndian::read_u16(&data) as usize;
        let decoded = String::from_utf8(data[2..2 + slen].to_vec()).unwrap();
        buf.push(decoded);
        *data = &data[2 + slen..];
    }
    buf
}

fn write_u8(target: &mut Vec<u8>, integer: u8) {
    target.push(integer);
}

fn write_u16(target: &mut Vec<u8>, integer: u16) {
    let mut buf = vec![0; 2];
    NetworkEndian::write_u16(&mut buf, integer);
    target.append(&mut buf);
}

fn write_u32(target: &mut Vec<u8>, integer: u32) {
    let mut buf = vec![0; 4];
    NetworkEndian::write_u32(&mut buf, integer);
    target.append(&mut buf);
}

fn write_string(target: &mut Vec<u8>, string: &str) {
    write_u16(target, string.len() as u16);
    target.extend_from_slice(string.as_bytes());
}

#[cfg(feature = "mmdb")]
fn parse_ext_params(msg: &mut &[u8]) -> Option<Vec<mmdb::FormattingArgument>> {
    let mut args = Vec::new();
    while !msg.is_empty() {
        let data_type = msg[0] as char;
        *msg = &msg[1..];

        match data_type {
            'S' => {
                let len = ((msg[0] as u32) * 256 + msg[1] as u32) as usize;

                let string = String::from_utf8(msg[2..2 + len].to_vec()).ok()?;
                *msg = &msg[2 + len..];
                args.push(mmdb::FormattingArgument::String(string));
            }
            's' => {
                let len = msg[0] as usize;
                let string = String::from_utf8(msg[1..1 + len - 2].to_vec()).ok()?;
                *msg = &msg[1 + len - 2..];
                args.push(mmdb::FormattingArgument::String(string));
            }
            'I' => {
                let num = NetworkEndian::read_u32(&msg[..4]);
                *msg = &msg[4..];
                args.push(mmdb::FormattingArgument::U32(num));
            }
            'i' => {}
            'u' => {
                let mut n = 0;
                for i in 0..5 {
                    n = n * 85 + (msg[i] as u32) - 33;
                }
                *msg = &msg[5..];
                args.push(mmdb::FormattingArgument::U32(n));
            }
            'R' => {
                let mut cat = 0;
                for i in 0..5 {
                    cat = cat * 85 + (msg[i] as u32) - 33;
                }
                let mut ins = 0;
                for i in 5..10 {
                    ins = ins * 85 + (msg[i] as u32) - 33;
                }
                *msg = &msg[10..];
                let string = mmdb::get_message(cat, ins)?;
                args.push(mmdb::FormattingArgument::String(string));
            }
            'l' => {
                let ins = NetworkEndian::read_u32(&msg[..4]);
                let cat = 20000;
                let string = mmdb::get_message(cat, ins)?;
                *msg = &msg[4..];
                args.push(mmdb::FormattingArgument::String(string));
            }
            '~' => return Some(args),
            _ => return None,
        }
    }

    Some(args)
}

/// Represents a kind of packet in the chat protocol.
#[derive(Debug, TryFromPrimitive, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum PacketType {
    LoginSeed = 0,
    LoginRequest = 2,
    LoginSelect = 3,
    LoginOk = 5,
    LoginError = 6,
    LoginCharlist = 7,
    ClientUnknown = 10,
    ClientName = 20,
    ClientLookup = 21,
    MsgPrivate = 30,
    MsgVicinity = 34,
    MsgVicinitya = 35,
    MsgSystem = 36,
    ChatNotice = 37,
    BuddyAdd = 40,
    BuddyRemove = 41,
    OnlineSet = 42,
    PrivgrpInvite = 50,
    PrivgrpKick = 51,
    PrivgrpJoin = 52,
    PrivgrpPart = 53,
    PrivgrpKickall = 54,
    PrivgrpClijoin = 55,
    PrivgrpClipart = 56,
    PrivgrpMessage = 57,
    PrivgrpRefuse = 58,
    GroupAnnounce = 60,
    GroupPart = 61,
    GroupDataSet = 62,
    GroupMessage = 65,
    GroupCmSet = 66,
    ClientmodeGet = 70,
    ClientmodeSet = 71,
    Ping = 100,
    Forward = 110,
    Cc = 120,
    AdmMuxInfo = 1100,
}

/// A trait for packets that can be sent to the server.
pub trait OutgoingPacket {
    /// Serializes a packet in the AOCP format.
    fn serialize(&self) -> SerializedPacket;
}

/// A trait for packets that can be received from the server.
pub trait IncomingPacket {
    /// Loads a packet in AOCP format and parses it to the appropiate packet.
    fn load(data: &[u8]) -> Result<Self>
    where
        Self: Sized;
}

// Current status
//
// Incoming:
// AOCP_LOGIN_SEED       (Login Seed)                      S         DONE
// AOCP_LOGIN_OK         (Login Result OK)                           DONE
// AOCP_LOGIN_ERROR      (Login Result Error)              S         DONE
// AOCP_LOGIN_CHARLIST   (Login CharacterList)             isii      DONE
// AOCP_CLIENT_UNKNOWN   (Client Unknown)                  I         Does not seem to exist
// AOCP_CLIENT_NAME      (Client Name)                     IS        DONE
// AOCP_CLIENT_LOOKUP    (Lookup Result)                   IS        DONE
// AOCP_MSG_PRIVATE      (Message Private)                 ISS       DONE
// AOCP_MSG_VICINITY     (Message Vicinity)                ISS       Can't be added
// AOCP_MSG_VICINITYA    (Message Anon Vicinity)           SSS       DONE
// AOCP_MSG_SYSTEM       (Message System)                  S         DONE
// AOCP_CHAT_NOTICE      (Chat Notice)                     IIIS      DONE
// AOCP_BUDDY_ADD        (Buddy Added)                     IIS       DONE
// AOCP_BUDDY_REMOVE     (Buddy Removed)                   I         DONE
// AOCP_PRIVGRP_INVITE   (Privategroup Invited)            I         DONE
// AOCP_PRIVGRP_KICK     (Privategroup Kicked)             I         DONE
// AOCP_PRIVGRP_CLIJOIN  (Privategroup Client Join)        II        DONE
// AOCP_PRIVGRP_CLIPART  (Privategroup Client Part)        II        DONE
// AOCP_PRIVGRP_MESSAGE  (Privategroup Message)            IISS      DONE
// AOCP_PRIVGRP_REFUSE   (Privategroup Refuse Invite)      II        Does not seem to exist
// AOCP_GROUP_ANNOUNCE   (Group Announce)                  GSIS      DONE
// AOCP_GROUP_PART       (Group Part)                      G         Does not seem to exist
// AOCP_GROUP_MESSAGE    (Group Message)                   GISS      DONE
// AOCP_PING             (Pong)                            S         DONE
// AOCP_FORWARD          (Forward)                         IM        Can't be added
// AOCP_ADM_MUX_INFO     (Adm Mux Info)                    iii       Can't be added
//
// Outgoing:
// AOCP_LOGIN_REQUEST    (Login Response GetCharLst)       ISS       DONE
// AOCP_LOGIN_SELECT     (Login Select Character)          I         DONE
// AOCP_CLIENT_LOOKUP    (Name Lookup)                     S         DONE
// AOCP_MSG_PRIVATE      (Message Private)                 ISS       DONE
// AOCP_BUDDY_ADD        (Buddy Add)                       IS        DONE
// AOCP_BUDDY_REMOVE     (Buddy Remove)                    I         DONE
// AOCP_ONLINE_SET       (Onlinestatus Set)                I         Can't be added
// AOCP_PRIVGRP_INVITE   (Privategroup Invite)             I         DONE
// AOCP_PRIVGRP_KICK     (Privategroup Kick)               I         DONE
// AOCP_PRIVGRP_JOIN     (Privategroup Join)               I         DONE
// AOCP_PRIVGRP_PART     (Privategroup Part)               I         DONE
// AOCP_PRIVGRP_KICKALL  (Privategroup Kickall)                      DONE
// AOCP_PRIVGRP_MESSAGE  (Privategroup Message)            ISS       DONE
// AOCP_GROUP_DATA_SET   (Group Data Set)                  GIS       Does not seem to exist
// AOCP_GROUP_MESSAGE    (Group Message)                   GSS       DONE
// AOCP_GROUP_CM_SET     (Group Clientmode Set)            GIIII     Can't be added
// AOCP_CLIENTMODE_GET   (Clientmode Get)                  IG        Can't be added
// AOCP_CLIENTMODE_SET   (Clientmode Set)                  IIII      Can't be added
// AOCP_PING             (Ping)                            S         DONE
// AOCP_CC               (CC)                              s         Can't be added

/// Enum for all packets possible to be received from the server.
#[derive(Debug)]
pub enum ReceivedPacket {
    LoginSeed(LoginSeedPacket),
    LoginError(LoginErrorPacket),
    LoginCharlist(LoginCharlistPacket),
    LoginOk,
    ClientName(ClientNamePacket),
    MsgVicinitya(MsgVicinityaPacket),
    BuddyStatus(BuddyStatusPacket),
    BuddyRemove(BuddyRemovePacket),
    GroupAnnounce(GroupAnnouncePacket),
    GroupMessage(GroupMessagePacket),
    ChatNotice(ChatNoticePacket),
    MsgPrivate(MsgPrivatePacket),
    ClientLookup(ClientLookupResultPacket),
    PrivgrpInvite(IncPrivgrpInvitePacket),
    PrivgrpClijoin(PrivgrpClijoinPacket),
    PrivgrpClipart(PrivgrpClipartPacket),
    PrivgrpMessage(PrivgrpMessagePacket),
    PrivgrpKick(IncPrivgrpKickPacket),
    MsgSystem(MsgSystemPacket),
    Ping(PingPacket),
}

/// A packet serialized for sending over the TCP connection.
pub type SerializedPacket = (PacketType, Vec<u8>);

impl TryFrom<(PacketType, &[u8])> for ReceivedPacket {
    type Error = Error;

    fn try_from(value: (PacketType, &[u8])) -> OrigResult<Self, Self::Error> {
        match value.0 {
            PacketType::LoginSeed => Ok(Self::LoginSeed(LoginSeedPacket::load(value.1)?)),
            PacketType::LoginError => Ok(Self::LoginError(LoginErrorPacket::load(value.1)?)),
            PacketType::LoginCharlist => {
                Ok(Self::LoginCharlist(LoginCharlistPacket::load(value.1)?))
            }
            PacketType::LoginOk => Ok(Self::LoginOk),
            PacketType::ClientName => Ok(Self::ClientName(ClientNamePacket::load(value.1)?)),
            PacketType::MsgVicinitya => Ok(Self::MsgVicinitya(MsgVicinityaPacket::load(value.1)?)),
            PacketType::BuddyAdd => Ok(Self::BuddyStatus(BuddyStatusPacket::load(value.1)?)),
            PacketType::BuddyRemove => Ok(Self::BuddyRemove(BuddyRemovePacket::load(value.1)?)),
            PacketType::GroupAnnounce => {
                Ok(Self::GroupAnnounce(GroupAnnouncePacket::load(value.1)?))
            }
            PacketType::GroupMessage => Ok(Self::GroupMessage(GroupMessagePacket::load(value.1)?)),
            PacketType::ChatNotice => Ok(Self::ChatNotice(ChatNoticePacket::load(value.1)?)),
            PacketType::MsgPrivate => Ok(Self::MsgPrivate(MsgPrivatePacket::load(value.1)?)),
            PacketType::ClientLookup => {
                Ok(Self::ClientLookup(ClientLookupResultPacket::load(value.1)?))
            }
            PacketType::PrivgrpInvite => {
                Ok(Self::PrivgrpInvite(IncPrivgrpInvitePacket::load(value.1)?))
            }
            PacketType::PrivgrpClijoin => {
                Ok(Self::PrivgrpClijoin(PrivgrpClijoinPacket::load(value.1)?))
            }
            PacketType::PrivgrpClipart => {
                Ok(Self::PrivgrpClipart(PrivgrpClipartPacket::load(value.1)?))
            }
            PacketType::PrivgrpMessage => {
                Ok(Self::PrivgrpMessage(PrivgrpMessagePacket::load(value.1)?))
            }
            PacketType::PrivgrpKick => Ok(Self::PrivgrpKick(IncPrivgrpKickPacket::load(value.1)?)),
            PacketType::MsgSystem => Ok(Self::MsgSystem(MsgSystemPacket::load(value.1)?)),
            PacketType::Ping => Ok(Self::Ping(PingPacket::load(value.1)?)),
            _ => Err(Error::UnknownPacket(Some(value.0))),
        }
    }
}

/// Packet that contains the login seed for authenticating.
#[derive(Debug)]
pub struct LoginSeedPacket {
    /// The seed for generating a login key.
    pub login_seed: String,
}

/// Packet for logging in to the server.
#[derive(Debug)]
pub struct LoginRequestPacket {
    /// The account name to log in on.
    pub username: String,
    /// The login key to log in with.
    pub key: String,
}

/// Packet indicating that a login failed.
#[derive(Debug)]
pub struct LoginErrorPacket {
    /// The error message from the server.
    pub message: String,
}

/// Packet listing the characters on an account.
#[derive(Debug)]
pub struct LoginCharlistPacket {
    /// A list of all `Character`s on the account.
    pub characters: Vec<Character>,
}

/// Packet used for choosing a character to log in with.
#[derive(Debug)]
pub struct LoginSelectPacket {
    /// The character ID to log in on.
    pub character_id: u32,
}

/// Packet with information about a player's name and ID.
#[derive(Debug)]
pub struct ClientNamePacket {
    /// The character ID.
    pub character_id: u32,
    /// The name of the character.
    pub character_name: String,
}

/// Anonymous vicinity message packet.
#[derive(Debug)]
pub struct MsgVicinityaPacket {
    /// The message as received.
    pub message: Message,
}

/// A buddy went online or offline.
#[derive(Debug)]
pub struct BuddyStatusPacket {
    /// Character ID of the buddy.
    pub character_id: u32,
    /// Whether the buddy is now online or not.
    pub online: bool,
    /// Send tag from the server.
    pub send_tag: String,
}

/// Add a buddy.
#[derive(Debug)]
pub struct BuddyAddPacket {
    /// The character to add as a buddy.
    pub character_id: u32,
    /// Send tag, should always be \u{1}
    pub send_tag: String,
}

/// Remove a buddy or confirmation of success.
#[derive(Debug)]
pub struct BuddyRemovePacket {
    /// The buddy to removed or that was removed.
    pub character_id: u32,
}

/// A channel becomes available.
#[derive(Debug)]
pub struct GroupAnnouncePacket {
    /// The channel that is now available.
    pub channel: Channel,
}

/// A message from a channel.
#[derive(Debug)]
pub struct GroupMessagePacket {
    /// The message received or to be sent.
    pub message: Message,
}

/// A chat notice like AFK.
#[derive(Debug)]
pub struct ChatNoticePacket {
    /// The chat noticed received.
    pub notice: ChatNotice,
}

/// A message from tells.
#[derive(Debug)]
pub struct MsgPrivatePacket {
    /// The message received.
    pub message: Message,
}

/// Packet for looking up a character by name.
#[derive(Debug)]
pub struct ClientLookupPacket {
    /// The character name to look up by.
    pub character_name: String,
}

/// Packet with a lookup result.
#[derive(Debug)]
pub struct ClientLookupResultPacket {
    /// Name of the character.
    pub character_name: String,
    /// ID of the character.
    pub character_id: u32,
    /// Whether this character exists or not.
    pub exists: bool,
}

/// Packet with an invite to a private group.
#[derive(Debug)]
pub struct IncPrivgrpInvitePacket {
    /// The channel the bot was invited to.
    pub channel: Channel,
}

/// Packet to invite someone to the private group.
#[derive(Debug)]
pub struct OutPrivgrpInvitePacket {
    /// The character to invite to the private group.
    pub character_id: u32,
}

/// Packet indicating someone joined a private group.
#[derive(Debug)]
pub struct PrivgrpClijoinPacket {
    /// The channel that the character joined.
    pub channel: Channel,
    /// Character that joined a private group.
    pub character_id: u32,
}

/// Packet indicating someone left a private group.
#[derive(Debug)]
pub struct PrivgrpClipartPacket {
    /// The channel that the character left.
    pub channel: Channel,
    /// The character that left a private group.
    pub character_id: u32,
}

/// Packet with a private group message.
#[derive(Debug)]
pub struct PrivgrpMessagePacket {
    /// The message to be sent or received.
    pub message: Message,
}

/// Packet to join a private group.
#[derive(Debug)]
pub struct PrivgrpJoinPacket {
    /// The private group to join.
    pub channel: Channel,
}

/// Packet to leave a private group.
#[derive(Debug)]
pub struct PrivgrpPartPacket {
    /// The private group to leave.
    pub channel: Channel,
}

/// Packet indicating the client got kicked from a private channel.
#[derive(Debug)]
pub struct IncPrivgrpKickPacket {
    /// The private channel the bot was kicked from.
    pub channel: Channel,
}

/// Packet to kick someone from the private group.
#[derive(Debug)]
pub struct OutPrivgrpKickPacket {
    /// The character to kick from the group.
    pub character_id: u32,
}

/// Packet to kick all members from the private group.
#[derive(Debug)]
pub struct PrivgrpKickallPacket {}

/// Packet with a system message.
#[derive(Debug)]
pub struct MsgSystemPacket {
    /// The message as received from the server.
    pub text: String,
}

/// Packet for keeping the connection open.
#[derive(Debug)]
pub struct PingPacket {
    /// The ping message to send or received.
    pub client: String,
}

impl IncomingPacket for LoginSeedPacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let seed = read_string(&mut data);
        Ok(Self { login_seed: seed })
    }
}

impl OutgoingPacket for LoginRequestPacket {
    fn serialize(&self) -> SerializedPacket {
        let mut buf = Vec::with_capacity(4 + 2 + self.username.len() + 2 + self.key.len());
        write_u32(&mut buf, 0);
        write_string(&mut buf, &self.username);
        write_string(&mut buf, &self.key);

        (PacketType::LoginRequest, buf)
    }
}

impl IncomingPacket for LoginErrorPacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let text = read_string(&mut data);
        Ok(Self { message: text })
    }
}

impl IncomingPacket for LoginCharlistPacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let ids = read_integer_array(&mut data);
        let names = read_string_array(&mut data);
        let levels = read_integer_array(&mut data);
        let online = read_integer_array(&mut data);
        let mut characters = Vec::with_capacity(ids.len());

        for i in 0..ids.len() {
            let character = Character {
                name: names[i].clone(),
                level: levels[i] as u8,
                online: online[i] != 0,
                id: ids[i],
            };
            characters.push(character);
        }

        Ok(Self { characters })
    }
}

impl OutgoingPacket for LoginSelectPacket {
    fn serialize(&self) -> SerializedPacket {
        let mut buf = Vec::with_capacity(4);
        write_u32(&mut buf, self.character_id);

        (PacketType::LoginSelect, buf)
    }
}

impl IncomingPacket for ClientNamePacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let character_id = read_u32(&mut data);
        let character_name = read_string(&mut data);

        Ok(Self {
            character_id,
            character_name,
        })
    }
}

impl IncomingPacket for MsgVicinityaPacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        // Sender, not interesting for anon
        let _ = read_string(&mut data);
        let content = read_string(&mut data);
        let send_tag = read_string(&mut data);

        let msg = Message {
            sender: None,
            channel: Channel::Vicinity,
            text: content,
            send_tag,
        };

        Ok(Self { message: msg })
    }
}

impl IncomingPacket for BuddyStatusPacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let character_id = read_u32(&mut data);
        let online = read_u32(&mut data) == 1;
        let send_tag = read_string(&mut data);

        Ok(Self {
            character_id,
            online,
            send_tag,
        })
    }
}

impl OutgoingPacket for BuddyStatusPacket {
    fn serialize(&self) -> SerializedPacket {
        let mut buf = Vec::with_capacity(8 + 2 + self.send_tag.len());
        write_u32(&mut buf, self.character_id);
        write_u32(&mut buf, self.online as u32);
        write_string(&mut buf, &self.send_tag);

        (PacketType::BuddyAdd, buf)
    }
}

impl OutgoingPacket for BuddyAddPacket {
    fn serialize(&self) -> SerializedPacket {
        let mut buf = Vec::with_capacity(7);
        write_u32(&mut buf, self.character_id);
        write_string(&mut buf, &self.send_tag);

        (PacketType::BuddyAdd, buf)
    }
}

impl IncomingPacket for BuddyAddPacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let character_id = read_u32(&mut data);
        let send_tag = read_string(&mut data);

        Ok(Self {
            character_id,
            send_tag,
        })
    }
}

impl IncomingPacket for BuddyRemovePacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let character_id = read_u32(&mut data);

        Ok(Self { character_id })
    }
}

impl OutgoingPacket for BuddyRemovePacket {
    fn serialize(&self) -> SerializedPacket {
        let mut buf = Vec::with_capacity(4);
        write_u32(&mut buf, self.character_id);

        (PacketType::BuddyRemove, buf)
    }
}

impl IncomingPacket for GroupAnnouncePacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let channel_type = read_u8(&mut data);
        let channel_id = read_u32(&mut data);
        let channel_name = read_string(&mut data);
        let status = read_u32(&mut data);
        // This is always empty
        // let d = read_string(&mut data);

        let channel = Channel::Group(Group {
            name: Some(channel_name),
            id: channel_id,
            r#type: ChannelType::try_from(channel_type).unwrap(),
            status: Some(status),
        });

        Ok(Self { channel })
    }
}

impl IncomingPacket for GroupMessagePacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let channel_type = read_u8(&mut data);
        let channel_id = read_u32(&mut data);
        let sender_id = read_u32(&mut data);
        let content = read_string(&mut data);
        let send_tag = read_string(&mut data);
        let channel = Channel::Group(Group {
            name: None,
            id: channel_id,
            r#type: ChannelType::try_from(channel_type).unwrap(),
            status: None,
        });

        let message = Message {
            sender: Some(sender_id),
            channel,
            text: content,
            send_tag,
        };

        Ok(Self { message })
    }
}

impl OutgoingPacket for GroupMessagePacket {
    fn serialize(&self) -> SerializedPacket {
        if let Channel::Group(g) = &self.message.channel {
            let mut buf = Vec::with_capacity(5 + 2 + self.message.text.len() + 1);
            write_u8(&mut buf, g.r#type as u8);
            write_u32(&mut buf, g.id);
            write_string(&mut buf, &self.message.text);
            write_string(&mut buf, "\u{0}");

            (PacketType::GroupMessage, buf)
        } else {
            panic!("Can only send this packet to group channels");
        }
    }
}

impl IncomingPacket for ChatNoticePacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let sender_id = read_u32(&mut data);
        // Seems to be 0 all the time
        let _ = read_u32(&mut data);
        // MMDB instance ID.
        let instance_id = read_u32(&mut data);
        let arguments = read_string(&mut data).into_bytes();

        // This is constant for chat notices.
        let category_id = 20000;

        #[cfg(feature = "mmdb")]
        let text = {
            let message = mmdb::get_message(category_id, instance_id).ok_or(Error::PayloadError)?;
            let params = parse_ext_params(&mut arguments.as_slice()).ok_or(Error::PayloadError)?;
            mmdb::format_string(&message, params).unwrap_or(message)
        };
        #[cfg(not(feature = "mmdb"))]
        let text = String::from("");

        let notice = ChatNotice {
            sender: sender_id,
            text,
        };

        Ok(Self { notice })
    }
}

impl IncomingPacket for MsgPrivatePacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let sender_id = read_u32(&mut data);
        let content = read_string(&mut data);
        let send_tag = read_string(&mut data);

        let message = Message {
            sender: Some(sender_id),
            channel: Channel::Tell(sender_id),
            text: content,
            send_tag,
        };

        Ok(Self { message })
    }
}

impl OutgoingPacket for MsgPrivatePacket {
    fn serialize(&self) -> SerializedPacket {
        if let Channel::Tell(recipient) = self.message.channel {
            let mut buf = Vec::with_capacity(4 + 2 + self.message.text.len() + 3);
            write_u32(&mut buf, recipient);
            write_string(&mut buf, &self.message.text);
            write_string(&mut buf, &self.message.send_tag);

            (PacketType::MsgPrivate, buf)
        } else {
            panic!("Can only send a private message with tell channel");
        }
    }
}

impl OutgoingPacket for ClientLookupPacket {
    fn serialize(&self) -> SerializedPacket {
        let mut buf = Vec::with_capacity(self.character_name.len());
        write_string(&mut buf, &self.character_name);

        (PacketType::ClientLookup, buf)
    }
}

impl IncomingPacket for ClientLookupResultPacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let character_id = read_u32(&mut data);
        let character_name = read_string(&mut data);
        let exists = character_id != MAXINT;

        Ok(Self {
            character_name,
            character_id,
            exists,
        })
    }
}

impl IncomingPacket for IncPrivgrpInvitePacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let channel_id = read_u32(&mut data);
        let channel = Channel::PrivateChannel(channel_id);

        Ok(Self { channel })
    }
}

impl OutgoingPacket for OutPrivgrpInvitePacket {
    fn serialize(&self) -> SerializedPacket {
        let mut buf = Vec::with_capacity(4);
        write_u32(&mut buf, self.character_id);

        (PacketType::PrivgrpInvite, buf)
    }
}

impl IncomingPacket for PrivgrpClijoinPacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let channel_id = read_u32(&mut data);
        let character_id = read_u32(&mut data);

        let channel = Channel::PrivateChannel(channel_id);

        Ok(Self {
            channel,
            character_id,
        })
    }
}

impl IncomingPacket for PrivgrpClipartPacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let channel_id = read_u32(&mut data);
        let character_id = read_u32(&mut data);

        let channel = Channel::PrivateChannel(channel_id);

        Ok(Self {
            channel,
            character_id,
        })
    }
}

impl IncomingPacket for PrivgrpMessagePacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let channel_id = read_u32(&mut data);
        let sender_id = read_u32(&mut data);
        let content = read_string(&mut data);
        let send_tag = read_string(&mut data);

        let message = Message {
            sender: Some(sender_id),
            channel: Channel::PrivateChannel(channel_id),
            text: content,
            send_tag,
        };

        Ok(Self { message })
    }
}

impl OutgoingPacket for PrivgrpMessagePacket {
    fn serialize(&self) -> SerializedPacket {
        if let Channel::PrivateChannel(id) = self.message.channel {
            let mut buf = Vec::with_capacity(4 + 2 + self.message.text.len() + 3);
            write_u32(&mut buf, id);
            write_string(&mut buf, &self.message.text);
            write_string(&mut buf, "\u{0}");

            (PacketType::PrivgrpMessage, buf)
        } else {
            panic!("Can only send to private channels with this packet")
        }
    }
}

impl OutgoingPacket for PrivgrpJoinPacket {
    fn serialize(&self) -> SerializedPacket {
        if let Channel::PrivateChannel(id) = self.channel {
            let mut buf = Vec::with_capacity(4);
            write_u32(&mut buf, id);

            (PacketType::PrivgrpJoin, buf)
        } else {
            panic!("Can only join private channels with this packet")
        }
    }
}

impl OutgoingPacket for PrivgrpPartPacket {
    fn serialize(&self) -> SerializedPacket {
        if let Channel::PrivateChannel(id) = self.channel {
            let mut buf = Vec::with_capacity(4);
            write_u32(&mut buf, id);

            (PacketType::PrivgrpPart, buf)
        } else {
            panic!("Can only leave private channels with this packet")
        }
    }
}

impl IncomingPacket for IncPrivgrpKickPacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let channel_id = read_u32(&mut data);

        let channel = Channel::PrivateChannel(channel_id);

        Ok(Self { channel })
    }
}

impl OutgoingPacket for OutPrivgrpKickPacket {
    fn serialize(&self) -> SerializedPacket {
        let mut buf = Vec::with_capacity(4);
        write_u32(&mut buf, self.character_id);

        (PacketType::PrivgrpKick, buf)
    }
}

impl OutgoingPacket for PrivgrpKickallPacket {
    fn serialize(&self) -> SerializedPacket {
        let buf = Vec::new();

        (PacketType::PrivgrpKickall, buf)
    }
}

impl IncomingPacket for MsgSystemPacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let text = read_string(&mut data);

        Ok(Self { text })
    }
}

impl OutgoingPacket for PingPacket {
    fn serialize(&self) -> SerializedPacket {
        let mut buf = Vec::with_capacity(2 + self.client.len());
        write_string(&mut buf, &self.client);

        (PacketType::Ping, buf)
    }
}

impl IncomingPacket for PingPacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let client = read_string(&mut data);

        Ok(Self { client })
    }
}
