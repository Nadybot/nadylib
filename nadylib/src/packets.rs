use crate::error::{Error, Result};
use crate::formatter::{format_string, FormattingArgument};
use crate::mmdb;
use crate::models::{Channel, ChannelType, Character, ChatNotice, Group, Message};

use byteorder::{ByteOrder, NetworkEndian};
use mmdb::get_message;
use num_enum::TryFromPrimitive;

use std::convert::TryFrom;
use std::result::Result as OrigResult;

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

fn parse_ext_params(msg: String) -> Option<Vec<FormattingArgument>> {
    // TODO: Refactor to use bytes because this is abominable
    let mut args = Vec::new();
    let mut characters = msg.chars().peekable();
    while characters.peek().is_some() {
        let data_type = characters.next()?;

        match data_type {
            'S' => {
                let len = (characters.next()? as u32) * 256 + characters.next()? as u32;

                let mut string = String::with_capacity(len as usize);
                for _ in 0..len {
                    string.push(characters.next()?);
                }
                args.push(FormattingArgument::String(string));
            }
            's' => {
                let len = characters.next()? as usize;

                let mut string = String::with_capacity(len as usize);
                for _ in 0..len - 2 {
                    string.push(characters.next()?);
                }
                args.push(FormattingArgument::String(string));
            }
            'I' => {
                let stuff = vec![
                    characters.next()? as u8,
                    characters.next()? as u8,
                    characters.next()? as u8,
                    characters.next()? as u8,
                ];
                let num = NetworkEndian::read_u32(&stuff);
                args.push(FormattingArgument::U32(num));
            }
            'i' => {}
            'u' => {
                let mut n = 0;
                for _ in 0..5 {
                    n = n * 85 + (characters.next()? as u32) - 33;
                }
                args.push(FormattingArgument::U32(n));
            }
            'R' => {
                let mut cat = 0;
                for _ in 0..5 {
                    cat = cat * 85 + (characters.next()? as u32) - 33;
                }
                let mut ins = 0;
                for _ in 0..5 {
                    ins = ins * 85 + (characters.next()? as u32) - 33;
                }
                let string = get_message(cat, ins)?;
                args.push(FormattingArgument::String(string));
            }
            'l' => {
                let stuff = vec![
                    characters.next()? as u8,
                    characters.next()? as u8,
                    characters.next()? as u8,
                    characters.next()? as u8,
                ];
                let ins = NetworkEndian::read_u32(&stuff);
                let cat = 20000;
                let string = get_message(cat, ins)?;
                args.push(FormattingArgument::String(string));
            }
            '~' => return Some(args),
            _ => return None,
        }
    }

    Some(args)
}

#[derive(Debug, TryFromPrimitive)]
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

pub trait OutgoingPacket {
    fn serialize(&self) -> (PacketType, Vec<u8>);
}

pub trait IncomingPacket {
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
// AOCP_CLIENT_UNKNOWN   (Client Unknown)                  I
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
// AOCP_PRIVGRP_KICK     (Privategroup Kicked)             I
// AOCP_PRIVGRP_PART     (Privategroup Part)               I
// AOCP_PRIVGRP_CLIJOIN  (Privategroup Client Join)        II        DONE
// AOCP_PRIVGRP_CLIPART  (Privategroup Client Part)        II        DONE
// AOCP_PRIVGRP_MESSAGE  (Privategroup Message)            IISS      DONE
// AOCP_PRIVGRP_REFUSE   (Privategroup Refuse Invite)      II
// AOCP_GROUP_ANNOUNCE   (Group Announce)                  GSIS      DONE
// AOCP_GROUP_PART       (Group Part)                      G
// AOCP_GROUP_MESSAGE    (Group Message)                   GISS      DONE
// AOCP_PING             (Pong)                            S
// AOCP_FORWARD          (Forward)                         IM
// AOCP_ADM_MUX_INFO     (Adm Mux Info)                    iii
//
// Outgoing:
// AOCP_LOGIN_REQUEST    (Login Response GetCharLst)       ISS       DONE
// AOCP_LOGIN_SELECT     (Login Select Character)          I         DONE
// AOCP_CLIENT_LOOKUP    (Name Lookup)                     S         DONE
// AOCP_MSG_PRIVATE      (Message Private)                 ISS
// AOCP_BUDDY_ADD        (Buddy Add)                       IS        DONE
// AOCP_BUDDY_REMOVE     (Buddy Remove)                    I         DONE
// AOCP_ONLINE_SET       (Onlinestatus Set)                I
// AOCP_PRIVGRP_INVITE   (Privategroup Invite)             I         DONE
// AOCP_PRIVGRP_KICK     (Privategroup Kick)               I
// AOCP_PRIVGRP_JOIN     (Privategroup Join)               I
// AOCP_PRIVGRP_PART     (Privategroup Part)               I
// AOCP_PRIVGRP_KICKALL  (Privategroup Kickall)
// AOCP_PRIVGRP_MESSAGE  (Privategroup Message)            ISS
// AOCP_GROUP_DATA_SET   (Group Data Set)                  GIS
// AOCP_GROUP_MESSAGE    (Group Message)                   GSS
// AOCP_GROUP_CM_SET     (Group Clientmode Set)            GIIII
// AOCP_CLIENTMODE_GET   (Clientmode Get)                  IG
// AOCP_CLIENTMODE_SET   (Clientmode Set)                  IIII
// AOCP_PING             (Ping)                            S
// AOCP_CC               (CC)                              s

#[derive(Debug)]
pub enum Packet {
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
    MsgSystem(MsgSystemPacket),
}

impl TryFrom<(PacketType, &[u8])> for Packet {
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
            PacketType::MsgSystem => Ok(Self::MsgSystem(MsgSystemPacket::load(value.1)?)),
            _ => Err(Error::UnknownPacket(Some(value.0))),
        }
    }
}

/// Packet that contains the login seed for authenticating.
#[derive(Debug)]
pub struct LoginSeedPacket {
    pub login_seed: String,
}

/// Packet for logging in to the server.
#[derive(Debug)]
pub struct LoginRequestPacket {
    pub username: String,
    pub key: String,
}

/// Packet indicating that a login failed.
#[derive(Debug)]
pub struct LoginErrorPacket {
    pub message: String,
}

/// Packet listing the characters on an account.
#[derive(Debug)]
pub struct LoginCharlistPacket {
    pub characters: Vec<Character>,
}

/// Packet used for choosing a character to log in with.
#[derive(Debug)]
pub struct LoginSelectPacket {
    pub character_id: u32,
}

/// Packet indicating the client name.
#[derive(Debug)]
pub struct ClientNamePacket {
    pub character_id: u32,
    pub character_name: String,
}

/// Anonymous vicinity message packet.
#[derive(Debug)]
pub struct MsgVicinityaPacket {
    pub message: Message,
}

/// A buddy went online or offline.
#[derive(Debug)]
pub struct BuddyStatusPacket {
    pub character_id: u32,
    pub online: bool,
}

/// Add a buddy.
#[derive(Debug)]
pub struct BuddyAddPacket {
    pub character_id: u32,
}

/// Remove a buddy or confirmation of success.
#[derive(Debug)]
pub struct BuddyRemovePacket {
    pub character_id: u32,
}

/// A channel becomes available.
#[derive(Debug)]
pub struct GroupAnnouncePacket {
    pub channel: Channel,
}

/// A message from a channel.
#[derive(Debug)]
pub struct GroupMessagePacket {
    pub message: Message,
}

/// A chat notice like AFK.
#[derive(Debug)]
pub struct ChatNoticePacket {
    pub notice: ChatNotice,
}

/// A message from tells.
#[derive(Debug)]
pub struct MsgPrivatePacket {
    pub message: Message,
}

/// Packet for looking up a character by name.
#[derive(Debug)]
pub struct ClientLookupPacket {
    pub character_name: String,
}

/// Packet with a lookup result.
#[derive(Debug)]
pub struct ClientLookupResultPacket {
    pub character_name: String,
    pub character_id: u32,
    pub exists: bool,
}

/// Packet with an invite to a private group.
#[derive(Debug)]
pub struct IncPrivgrpInvitePacket {
    pub channel: Channel,
}

/// Packet to invite someone to the private group.
#[derive(Debug)]
pub struct OutPrivgrpInvitePacket {
    pub character_id: u32,
}

/// Packet indicating someone joined a private group.
#[derive(Debug)]
pub struct PrivgrpClijoinPacket {
    pub channel: Channel,
    pub character_id: u32,
}

/// Packet indicating someone left a private group.
#[derive(Debug)]
pub struct PrivgrpClipartPacket {
    pub channel: Channel,
    pub character_id: u32,
}

/// Packet with a private group message.
#[derive(Debug)]
pub struct PrivgrpMessagePacket {
    pub message: Message,
}

/// Packet with a system message.
#[derive(Debug)]
pub struct MsgSystemPacket {
    pub text: String,
}

impl IncomingPacket for LoginSeedPacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let seed = read_string(&mut data);
        Ok(Self { login_seed: seed })
    }
}

impl OutgoingPacket for LoginRequestPacket {
    fn serialize(&self) -> (PacketType, Vec<u8>) {
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
    fn serialize(&self) -> (PacketType, Vec<u8>) {
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
        // always 0
        // let key = read_string(&mut data);

        let msg = Message {
            sender: None,
            channel: Channel::Vicinity,
            text: content,
        };

        Ok(Self { message: msg })
    }
}

impl IncomingPacket for BuddyStatusPacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let character_id = read_u32(&mut data);
        let online = read_u32(&mut data) == 1;
        // Seems unneeded
        // let c = read_string(&mut data);

        Ok(Self {
            character_id,
            online,
        })
    }
}

impl OutgoingPacket for BuddyAddPacket {
    fn serialize(&self) -> (PacketType, Vec<u8>) {
        let mut buf = Vec::with_capacity(7);
        write_u32(&mut buf, self.character_id);
        write_string(&mut buf, "\u{1}");

        (PacketType::BuddyAdd, buf)
    }
}

impl IncomingPacket for BuddyRemovePacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let character_id = read_u32(&mut data);

        Ok(Self { character_id })
    }
}

impl OutgoingPacket for BuddyRemovePacket {
    fn serialize(&self) -> (PacketType, Vec<u8>) {
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
        // We do not know what this is
        // it seems to depend on the type
        // let _ = read_u32(&mut data);
        // This is always empty
        // let d = read_string(&mut data);

        let channel = Channel::Group(Group {
            name: Some(channel_name),
            id: channel_id,
            r#type: ChannelType::try_from(channel_type).unwrap(),
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
        // This seems to be empty
        // let c = read_string(&mut data);
        let channel = Channel::Group(Group {
            name: None,
            id: channel_id,
            r#type: ChannelType::try_from(channel_type).unwrap(),
        });

        let message = Message {
            sender: Some(sender_id),
            channel,
            text: content,
        };

        Ok(Self { message })
    }
}

impl IncomingPacket for ChatNoticePacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let sender_id = read_u32(&mut data);
        // Seems to be 0 all the time
        let _ = read_u32(&mut data);
        // MMDB instance ID.
        let instance_id = read_u32(&mut data);
        let arguments = read_string(&mut data);

        // This is constant for chat notices.
        let category_id = 20000;

        let message = mmdb::get_message(category_id, instance_id).ok_or(Error::PayloadError)?;
        let params = parse_ext_params(arguments).ok_or(Error::PayloadError)?;

        let text = format_string(&message, params).unwrap_or(message);

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
        // Last part is always \0
        // let c = read_string(&mut data);

        let message = Message {
            sender: Some(sender_id),
            channel: Channel::Tell(sender_id),
            text: content,
        };

        Ok(Self { message })
    }
}

impl OutgoingPacket for ClientLookupPacket {
    fn serialize(&self) -> (PacketType, Vec<u8>) {
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
    fn serialize(&self) -> (PacketType, Vec<u8>) {
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
        // Seems to be empty always
        // let d = read_string(&mut data);

        let message = Message {
            sender: Some(sender_id),
            channel: Channel::PrivateChannel(channel_id),
            text: content,
        };

        Ok(Self { message })
    }
}

impl IncomingPacket for MsgSystemPacket {
    fn load(mut data: &[u8]) -> Result<Self> {
        let text = read_string(&mut data);

        Ok(Self { text })
    }
}
