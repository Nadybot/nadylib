use dashmap::DashMap;
use dotenv::dotenv;
use log::{debug, info};
use nadylib::{
    packets::{BuddyRemovePacket, IncomingPacket, MsgPrivatePacket, OutgoingPacket, PacketType},
    AOSocket, Result,
};
use tokio::{
    net::TcpListener,
    spawn,
    sync::{mpsc::unbounded_channel, Notify},
};

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

mod config;
mod worker;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = config::load_config().expect("Invalid configuration");
    let spam_bot_support = config.spam_bot_support;
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
    let buddies: Arc<DashMap<usize, HashSet<u32>>> = Arc::new(DashMap::with_capacity(account_num));
    let local_buddies = buddies.clone();
    // List of communication channels to the workers
    let mut senders = HashMap::with_capacity(account_num);
    let mut receivers = HashMap::with_capacity(account_num);
    for i in 0..account_num {
        let (s, r) = unbounded_channel();
        senders.insert(i, s);
        receivers.insert(i, r);
    }

    let sock_sender = sock.get_sender();
    let duplicate_sock_sender = sock_sender.clone();
    let real_sock_sender = real_sock.get_sender();

    let logged_in = Arc::new(Notify::new());
    let logged_in_setter = logged_in.clone();

    // Forward all incoming packets to the client
    spawn(async move {
        loop {
            let packet = real_sock.read_raw_packet().await.unwrap();

            if let PacketType::LoginOk = packet.0 {
                logged_in_setter.notify_one();
            }

            let _ = sock_sender.send(packet);
        }
    });

    // Loop over incoming packets and depending on the type, round robin them
    // If not, we just send them over the normal FC connection
    spawn(async move {
        // For round robin on private msgs
        let mut current_buddy = 0;
        loop {
            let packet = sock.read_raw_packet().await.unwrap();

            match packet.0 {
                PacketType::BuddyAdd => {
                    // Add the buddy on the slave with least buddies
                    let mut least_buddies = 0;
                    let mut buddy_count = local_buddies.get(&0).unwrap().value().len();

                    for elem in local_buddies.iter().skip(1) {
                        let val = elem.value().len();
                        if val < buddy_count {
                            buddy_count = val;
                            least_buddies = *elem.key();
                        }
                    }

                    debug!(
                        "Adding buddy on worker #{} ({} current buddies)",
                        least_buddies + 1,
                        buddy_count
                    );
                    let _ = senders.get(&least_buddies).unwrap().send(packet);
                }
                PacketType::BuddyRemove => {
                    let b = BuddyRemovePacket::load(&packet.1).unwrap();
                    // Remove the buddy on the slaves that have it on the buddy list
                    for elem in local_buddies.iter() {
                        if elem.value().contains(&b.character_id) {
                            let worker_id = elem.key();
                            debug!(
                                "Removing buddy {} on worker #{}",
                                b.character_id,
                                worker_id + 1
                            );
                            let _ = senders.get(worker_id).unwrap().send(packet.clone());
                        }
                    }
                }
                PacketType::MsgPrivate => {
                    let mut m = MsgPrivatePacket::load(&packet.1).unwrap();
                    if m.message.send_tag == "spam" {
                        m.message.send_tag = String::from("\u{0}");
                        let serialized = m.serialize();
                        if spam_bot_support {
                            debug!("Sending spam tell: {:?}", m.message);
                            let _ = senders.get(&current_buddy).unwrap().send(serialized);
                        } else {
                            debug!("Sending regular tell: {:?}", m.message);
                            let _ = real_sock_sender.send(serialized);
                        }
                        if current_buddy == account_num - 1 {
                            current_buddy = 0;
                        } else {
                            current_buddy += 1;
                        }
                    } else {
                        debug!("Sending regular tell");
                        let _ = real_sock_sender.send(packet);
                    }
                }
                _ => {
                    let _ = real_sock_sender.send(packet);
                }
            }
        }
    });

    // Wait until logged in
    logged_in.notified().await;

    // Create all slaves
    let mut tasks = Vec::new();
    for (idx, acc) in config.accounts.iter().enumerate() {
        info!("Spawning worker for {}", acc.character);
        tasks.push(spawn(worker::worker_main(
            idx,
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
