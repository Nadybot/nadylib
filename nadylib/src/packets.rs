use crate::error::{Error, Result};
use crate::mmdb;
use crate::models::{Channel, ChannelType, Character, ChatNotice, Group, Message};

use byteorder::{ByteOrder, NetworkEndian};
use mmdb::get_message;
use num_enum::TryFromPrimitive;

use std::convert::TryFrom;
use std::result::Result as OrigResult;

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

fn parse_ext_params(msg: String) -> Option<Vec<String>> {
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
                args.push(string);
            }
            's' => {
                let len = characters.next()? as usize;

                let mut string = String::with_capacity(len as usize);
                for _ in 0..len - 2 {
                    string.push(characters.next()?);
                }
                args.push(string);
            }
            'I' => {
                let stuff = vec![
                    characters.next()? as u8,
                    characters.next()? as u8,
                    characters.next()? as u8,
                    characters.next()? as u8,
                ];
                let num = NetworkEndian::read_u32(&stuff);
                args.push(num.to_string());
            }
            'i' => {}
            'u' => {
                let mut n = 0;
                for _ in 0..5 {
                    n = n * 85 + (characters.next()? as u32) - 33;
                }
                args.push(n.to_string());
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
                args.push(string);
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
                args.push(string);
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

#[derive(Debug)]
pub enum Packet {
    LoginSeed(LoginSeedPacket),
    LoginError(LoginErrorPacket),
    LoginCharlist(LoginCharlistPacket),
    LoginOk,
    ClientName(ClientNamePacket),
    MsgVicinitya(MsgVicinityaPacket),
    BuddyAdd(BuddyAddPacket),
    GroupAnnounce(GroupAnnouncePacket),
    GroupMessage(GroupMessagePacket),
    ChatNotice(ChatNoticePacket),
    MsgPrivate(MsgPrivatePacket),
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
            PacketType::BuddyAdd => Ok(Self::BuddyAdd(BuddyAddPacket::load(value.1)?)),
            PacketType::GroupAnnounce => {
                Ok(Self::GroupAnnounce(GroupAnnouncePacket::load(value.1)?))
            }
            PacketType::GroupMessage => Ok(Self::GroupMessage(GroupMessagePacket::load(value.1)?)),
            PacketType::ChatNotice => Ok(Self::ChatNotice(ChatNoticePacket::load(value.1)?)),
            PacketType::MsgPrivate => Ok(Self::MsgPrivate(MsgPrivatePacket::load(value.1)?)),
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
pub struct BuddyAddPacket {
    pub character_id: u32,
    pub online: bool,
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

impl IncomingPacket for BuddyAddPacket {
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
            channel: channel,
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
        // let arguments = read_string(&mut data);

        // This is constant for chat notices.
        let category_id = 20000;

        let message = mmdb::get_message(category_id, instance_id).ok_or(Error::PayloadError)?;
        // let params = parse_ext_params(arguments).ok_or(Error::PayloadError)?;

        let notice = ChatNotice {
            sender: sender_id,
            text: message,
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
            channel: Channel::Private(sender_id),
            text: content,
        };

        Ok(Self { message })
    }
}
