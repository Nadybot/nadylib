use crate::config::{AccountData, Config};

use dashmap::DashMap;
use log::{debug, error, info, log_enabled, trace, Level::Trace};
use nadylib::{
    packets::{LoginSelectPacket, OutgoingPacket, PacketType, SerializedPacket},
    AOSocket, ReceivedPacket, Result,
};
use tokio::{
    spawn,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};

use std::{convert::TryFrom, sync::Arc};

// The main helper bot task
pub async fn worker_main(
    id: usize,
    config: Config,
    account: AccountData,
    sender: UnboundedSender<SerializedPacket>,
    buddies: Arc<DashMap<usize, DashMap<u32, ()>>>,
    mut packet_reader: UnboundedReceiver<SerializedPacket>,
) -> Result<()> {
    let mut socket = AOSocket::connect(config.server_address).await?;
    let socket_sender = socket.get_sender();
    buddies.insert(id, DashMap::new());

    // Forward all incoming packets from the master bot to this slave connection
    spawn(async move {
        loop {
            let packet = packet_reader.recv().await.unwrap();
            debug!("Sending {:?} packet from worker #{}", packet.0, id);

            if log_enabled!(Trace) {
                let loaded = ReceivedPacket::try_from((packet.0, packet.1.as_slice()));
                if let Ok(pack) = loaded {
                    trace!("Packet body: {:?}", pack);
                }
            }
            let _ = socket_sender.send(packet);
        }
    });

    loop {
        // Read a packet and handle it if interested
        let (packet_type, body) = socket.read_raw_packet().await?;
        debug!("Received {:?} packet for worker #{}", packet_type, id);

        if log_enabled!(Trace) {
            let loaded = ReceivedPacket::try_from((packet_type, body.as_slice()));
            if let Ok(pack) = loaded {
                trace!("Packet body: {:?}", pack);
            }
        }

        match packet_type {
            PacketType::LoginSeed
            | PacketType::LoginCharlist
            | PacketType::LoginOk
            | PacketType::LoginError
            | PacketType::BuddyAdd
            | PacketType::BuddyRemove
            | PacketType::ClientName
            | PacketType::MsgPrivate => {
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
                        debug!("Sending LoginOk packet from worker #{} to main", id);
                        sender.send((packet_type, body))?;
                    }
                    ReceivedPacket::LoginError(e) => {
                        error!("{} failed to log in: {}", account.character, e.message);
                        break;
                    }
                    ReceivedPacket::ClientName(_) => {
                        debug!("Sending ClientName packet from worker #{} to main", id);
                        sender.send((packet_type, body))?;
                    }
                    ReceivedPacket::BuddyStatus(b) => {
                        debug!(
                            "Worker #{}: Buddy {} is online: {}",
                            id, b.character_id, b.online
                        );
                        debug!("Sending BuddyAdd packet from worker #{} to main", id);
                        buddies.get(&id).unwrap().insert(b.character_id, ());
                        sender.send((packet_type, body))?;
                    }
                    ReceivedPacket::BuddyRemove(b) => {
                        debug!("Worker #{}: Buddy {} removed", id, b.character_id);
                        debug!("Sending BuddyRemove packet from worker #{} to main", id);
                        buddies.get(&id).unwrap().remove(&b.character_id);
                        sender.send((packet_type, body))?;
                    }
                    ReceivedPacket::MsgPrivate(mut m) => {
                        if config.relay_slave_tells {
                            debug!("Relaying tell message from worker #{} to main", id);
                            m.message.send_tag = id.to_string();
                            sender.send(m.serialize())?;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok(())
}
