use crate::config::{AccountData, Config};

use dashmap::DashMap;
use log::{debug, error, info};
use nadylib::{
    packets::{LoginSelectPacket, PacketType, SerializedPacket},
    AOSocket, ReceivedPacket, Result,
};
use tokio::{
    spawn,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};

use std::{collections::HashSet, convert::TryFrom, sync::Arc};

// The main helper bot task
pub async fn worker_main(
    id: usize,
    config: Config,
    account: AccountData,
    sender: UnboundedSender<SerializedPacket>,
    buddies: Arc<DashMap<usize, HashSet<u32>>>,
    mut packet_reader: UnboundedReceiver<SerializedPacket>,
) -> Result<()> {
    let mut socket = AOSocket::connect(config.server_address).await?;
    let socket_sender = socket.get_sender();
    buddies.insert(id, HashSet::new());

    // Forward all incoming packets from the master bot to this slave connection
    spawn(async move {
        loop {
            let packet = packet_reader.recv().await.unwrap();
            let _ = socket_sender.send(packet);
        }
    });

    loop {
        // Read a packet and handle it if interested
        let (packet_type, body) = socket.read_raw_packet().await?;

        match packet_type {
            PacketType::LoginSeed
            | PacketType::LoginCharlist
            | PacketType::LoginOk
            | PacketType::LoginError
            | PacketType::BuddyAdd
            | PacketType::BuddyRemove => {
                let packet = ReceivedPacket::try_from((packet_type, body.as_slice()))?;

                match packet {
                    ReceivedPacket::LoginSeed(l) => {
                        socket.login(&account.username, &account.password, &l.login_seed)?;
                    }
                    ReceivedPacket::LoginCharlist(c) => {
                        let character_id = c
                            .characters
                            .iter()
                            .find(|i| i.name == account.character)
                            .unwrap()
                            .id;
                        let pack = LoginSelectPacket { character_id };
                        socket.send(pack)?;
                    }
                    ReceivedPacket::LoginOk => {
                        info!("{} logged in", account.character);
                    }
                    ReceivedPacket::LoginError(e) => {
                        error!("{} failed to log in: {}", account.character, e.message);
                        break;
                    }
                    ReceivedPacket::BuddyStatus(b) => {
                        debug!("Buddy {} is online: {}", b.character_id, b.online);
                        buddies.update(&id, |_, v| {
                            let mut w = v.clone();
                            w.insert(b.character_id);
                            w
                        });
                        sender.send((packet_type, body))?;
                    }
                    ReceivedPacket::BuddyRemove(b) => {
                        debug!("Buddy {} removed", b.character_id);
                        buddies.update(&id, |_, v| {
                            let mut w = v.clone();
                            w.remove(&b.character_id);
                            w
                        });
                        sender.send((packet_type, body))?;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok(())
}
