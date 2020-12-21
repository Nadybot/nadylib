use dashmap::DashMap;
use dotenv::dotenv;
use log::{debug, info, log_enabled, trace, Level::Trace};
use nadylib::{
    packets::{
        BuddyRemovePacket, BuddyStatusPacket, IncomingPacket, MsgPrivatePacket, OutgoingPacket,
        PacketType, ReceivedPacket,
    },
    AOSocket, Result,
};
use tokio::{
    net::TcpListener,
    spawn,
    sync::{mpsc::unbounded_channel, Notify},
};

use std::{collections::HashMap, convert::TryFrom, sync::Arc};

mod config;
mod worker;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = config::load_config().expect("Invalid configuration");
    let spam_bot_support = config.spam_bot_support;
    let send_tells_over_main = config.send_tells_over_main;
    let account_num = config.accounts.len();
    let tcp_server = TcpListener::bind(format!("0.0.0.0:{}", config.port_number)).await?;

    info!("Listening on port {}", config.port_number);
    info!("Waiting for client to connect...");
    let (client, addr) = tcp_server.accept().await?;
    info!("Client connected from {}", addr);

    // Create the main connection to the chat servers
    let mut sock = AOSocket::from_stream(client);
    let mut real_sock = AOSocket::connect(config.server_address.clone()).await?;

    // List of all buddies
    let buddies: Arc<DashMap<usize, DashMap<u32, ()>>> =
        Arc::new(DashMap::with_capacity(account_num + 1));
    // The main bot buddies
    buddies.insert(0, DashMap::new());
    let task1_buddies = buddies.clone();
    let task2_buddies = buddies.clone();
    // List of communication channels to the workers
    let mut senders = HashMap::with_capacity(account_num + 1);
    let mut receivers = HashMap::with_capacity(account_num);
    senders.insert(0, real_sock.get_sender());
    for i in 0..account_num {
        let (s, r) = unbounded_channel();
        senders.insert(i + 1, s);
        receivers.insert(i, r);
    }

    let sock_sender = sock.get_sender();
    let duplicate_sock_sender = sock_sender.clone();
    let real_sock_sender = real_sock.get_sender();

    let logged_in = Arc::new(Notify::new());
    let logged_in_setter = logged_in.clone();

    let mut tasks = Vec::new();

    // Forward all incoming packets to the client
    tasks.push(spawn(async move {
        loop {
            let packet = real_sock.read_raw_packet().await.unwrap();
            debug!("Received {:?} packet for main", packet.0);

            if log_enabled!(Trace) {
                let loaded = ReceivedPacket::try_from((packet.0, packet.1.as_slice()));
                if let Ok(pack) = loaded {
                    trace!("Packet body: {:?}", pack);
                }
            }

            match packet.0 {
                PacketType::LoginOk => logged_in_setter.notify_one(),
                PacketType::BuddyAdd => {
                    let b = BuddyStatusPacket::load(&packet.1).unwrap();
                    debug!("Buddy {} is online: {}", b.character_id, b.online);
                    task1_buddies.get(&0).unwrap().insert(b.character_id, ());
                }
                PacketType::BuddyRemove => {
                    let b = BuddyRemovePacket::load(&packet.1).unwrap();
                    debug!("Buddy {} removed", b.character_id);
                    task1_buddies.get(&0).unwrap().remove(&b.character_id);
                }
                _ => {}
            }

            let _ = sock_sender.send(packet);
        }
    }));

    // Loop over incoming packets and depending on the type, round robin them
    // If not, we just send them over the normal FC connection
    tasks.push(spawn(async move {
        // For round robin on private msgs
        let start_at = {
            if send_tells_over_main {
                0
            } else {
                1
            }
        };
        let mut current_buddy = start_at;
        loop {
            let packet = sock.read_raw_packet().await.unwrap();
            debug!("Received {:?} packet from main", packet.0);

            if log_enabled!(Trace) {
                let loaded = ReceivedPacket::try_from((packet.0, packet.1.as_slice()));
                if let Ok(pack) = loaded {
                    trace!("Packet body: {:?}", pack);
                }
            }

            match packet.0 {
                PacketType::BuddyAdd => {
                    // Add the buddy on the slave with least buddies
                    let mut least_buddies = 0;
                    let mut buddy_count = task2_buddies.get(&0).unwrap().value().len();

                    for elem in task2_buddies.iter().skip(1) {
                        let val = elem.value().len();
                        if val < buddy_count {
                            buddy_count = val;
                            least_buddies = *elem.key();
                        }
                    }

                    if least_buddies == 0 {
                        debug!("Adding buddy on main ({} current buddies)", buddy_count);
                    } else {
                        debug!(
                            "Adding buddy on worker #{} ({} current buddies)",
                            least_buddies, buddy_count
                        );
                    }
                    let _ = senders.get(&least_buddies).unwrap().send(packet);
                }
                PacketType::BuddyRemove => {
                    let b = BuddyRemovePacket::load(&packet.1).unwrap();
                    // Remove the buddy on the slaves that have it on the buddy list
                    for elem in task2_buddies.iter() {
                        if elem.value().get(&b.character_id).is_some() {
                            let worker_id = elem.key();
                            if worker_id == &0 {
                                debug!("Removing buddy {} on main", b.character_id);
                            } else {
                                debug!(
                                    "Removing buddy {} on worker #{}",
                                    b.character_id,
                                    worker_id + 1
                                );
                            }
                            let _ = senders.get(worker_id).unwrap().send(packet.clone());
                        }
                    }
                }
                PacketType::MsgPrivate => {
                    let mut m = MsgPrivatePacket::load(&packet.1).unwrap();
                    if m.message.send_tag == "spam" {
                        m.message.send_tag = String::from("\u{0}");
                        let serialized = m.serialize();
                        if spam_bot_support && current_buddy != 0 {
                            let _ = senders.get(&current_buddy).unwrap().send(serialized);
                        } else {
                            let _ = real_sock_sender.send(serialized);
                        }
                        if current_buddy == account_num {
                            current_buddy = start_at;
                        } else {
                            current_buddy += 1;
                        }
                    } else {
                        let _ = real_sock_sender.send(packet);
                    }
                }
                _ => {
                    let _ = real_sock_sender.send(packet);
                }
            }
        }
    }));

    // Wait until logged in
    logged_in.notified().await;

    // Create all slaves
    for (idx, acc) in config.accounts.iter().enumerate() {
        info!("Spawning worker for {}", acc.character);
        tasks.push(spawn(worker::worker_main(
            idx + 1,
            config.clone(),
            acc.clone(),
            duplicate_sock_sender.clone(),
            buddies.clone(),
            receivers.remove(&idx).unwrap(),
        )));
    }

    for task in tasks {
        let _ = task.await;
    }

    Ok(())
}
