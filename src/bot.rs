use crate::{
    client_socket::{AOSocket, SocketConfig},
    error::Result,
    models::Channel,
    packets::{BuddyAddPacket, LoginSelectPacket, ReceivedPacket},
};

use log::{error, info};
use tokio::time::{sleep, Duration};

use std::collections::HashMap;

pub struct Bot {
    socket: AOSocket,
    channels: Vec<Channel>,
    users_by_id: HashMap<u32, String>,
    users_by_name: HashMap<String, u32>,
    buddies: HashMap<u32, bool>,

    username: String,
    password: String,
    character: String,
}

impl Bot {
    /// Creates a new RK5 bot.
    pub async fn new(username: String, password: String, character: String) -> Result<Self> {
        let socket = AOSocket::connect("chat.d1.funcom.com:7105", SocketConfig::default()).await?;

        Ok(Self {
            socket,
            channels: Vec::new(),
            users_by_id: HashMap::new(),
            users_by_name: HashMap::new(),
            buddies: HashMap::new(),
            username,
            password,
            character,
        })
    }

    /// Gets the name of a user by ID.
    pub fn get_character_name(&self, character_id: u32) -> Option<String> {
        self.users_by_id.get(&character_id).cloned()
    }

    /// Gets the ID of a user by name.
    pub fn get_character_id(&self, character_name: String) -> Option<u32> {
        self.users_by_name.get(&character_name).cloned()
    }

    /// Gets the online status of a character.
    pub async fn is_online(&self, character_id: u32) -> Option<bool> {
        match self.buddies.get(&character_id) {
            Some(online) => Some(*online),
            None => {
                let packet = BuddyAddPacket { character_id };
                self.socket.send(packet).ok()?;
                sleep(Duration::from_millis(500)).await;
                self.buddies.get(&character_id).cloned()
            }
        }
    }

    /// Runs the main bot loop.
    pub async fn main(&mut self) -> Result<()> {
        loop {
            let packet = self.socket.read_packet().await?;

            match packet {
                ReceivedPacket::BuddyRemove(_)
                | ReceivedPacket::MsgVicinitya(_)
                | ReceivedPacket::PrivgrpInvite(_)
                | ReceivedPacket::PrivgrpClijoin(_)
                | ReceivedPacket::PrivgrpClipart(_)
                | ReceivedPacket::MsgPrivate(_)
                | ReceivedPacket::MsgSystem(_)
                | ReceivedPacket::GroupMessage(_)
                | ReceivedPacket::PrivgrpMessage(_)
                | ReceivedPacket::Ping(_)
                | ReceivedPacket::PrivgrpKick(_) => {}
                ReceivedPacket::BuddyStatus(b) => {
                    self.buddies.insert(b.character_id, b.online);
                }
                ReceivedPacket::ChatNotice(_) => {}
                ReceivedPacket::ClientLookup(c) => {
                    self.users_by_id
                        .insert(c.character_id, c.character_name.clone());
                    self.users_by_name.insert(c.character_name, c.character_id);
                }
                ReceivedPacket::ClientName(c) => {
                    self.users_by_id
                        .insert(c.character_id, c.character_name.clone());
                    self.users_by_name.insert(c.character_name, c.character_id);
                }
                ReceivedPacket::LoginSeed(l) => {
                    self.socket
                        .login(&self.username, &self.password, &l.login_seed)?;
                }
                ReceivedPacket::LoginCharlist(c) => {
                    let character_id = c
                        .characters
                        .iter()
                        .find(|i| i.name == self.character)
                        .unwrap()
                        .id;
                    let pack = LoginSelectPacket { character_id };
                    self.socket.send(pack)?;
                }
                ReceivedPacket::LoginOk => info!("Successfully logged in"),
                ReceivedPacket::LoginError(e) => error!("{}", e.message),
                ReceivedPacket::GroupAnnounce(g) => {
                    self.channels.push(g.channel);
                }
            }
        }
    }
}
