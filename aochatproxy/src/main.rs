use nadylib::client_socket::AOSocket;
use nadylib::error::Result;
use nadylib::models::Channel;
use nadylib::packets::{
    BuddyAddPacket, BuddyRemovePacket, ClientLookupPacket, LoginSelectPacket,
    OutPrivgrpInvitePacket, OutPrivgrpKickPacket, Packet, PrivgrpJoinPacket, PrivgrpKickallPacket,
    PrivgrpPartPacket,
};

use std::env::var;

#[tokio::main]
async fn main() -> Result<()> {
    // This is not actually the aochatproxy
    // but until nadylib is finished
    // only a playground for the library
    let mut sock = AOSocket::connect("chat.d1.funcom.com:7105").await?;
    let mut my_id: u32 = 0;

    loop {
        let packet = sock.read_packet().await?;

        match packet {
            Packet::LoginSeed(l) => {
                sock.login(
                    &var("AO_ACCOUNT").unwrap(),
                    &var("AO_PASSWORD").unwrap(),
                    &l.login_seed,
                )
                .await?;
            }
            Packet::LoginError(e) => println!("Error: {}", e.message),
            Packet::LoginCharlist(c) => {
                let char_name = var("AO_CHAR").unwrap();
                let char_id = c
                    .characters
                    .iter()
                    .find(|i| i.name == char_name)
                    .unwrap()
                    .id;
                let pack = LoginSelectPacket {
                    character_id: char_id,
                };
                sock.send(pack).await?;
            }
            Packet::LoginOk => println!("Successfully logged in"),
            Packet::ClientName(c) => {
                if my_id == 0 {
                    my_id = c.character_id;
                }
                println!("Character {} has ID {}", c.character_name, c.character_id)
            }
            Packet::MsgVicinitya(m) => {
                println!("{:?}", m.message);
            }
            Packet::BuddyStatus(b) => {
                println!("Buddy {} online? {}", b.character_id, b.online)
            }
            Packet::GroupAnnounce(c) => {
                if let Channel::Group(g) = c.channel {
                    println!("Am in channel {:?} ({}) {:?}", g.name, g.id, g.r#type)
                }
            }
            Packet::MsgPrivate(m) => {
                println!(
                    "Got a msg from {:?} in {:?} with text {}",
                    m.message.sender, m.message.channel, m.message.text
                );
                if m.message.text.starts_with("!lookup") {
                    let arg = m.message.text.split_whitespace().nth(1).unwrap_or("");
                    let pack = ClientLookupPacket {
                        character_name: arg.to_owned(),
                    };
                    sock.send(pack).await?;
                } else if m.message.text.starts_with("!rembuddy") {
                    let arg = m.message.text.split_whitespace().nth(1).unwrap_or("");
                    let pack = BuddyRemovePacket {
                        character_id: arg.parse().unwrap(),
                    };
                    sock.send(pack).await?;
                } else if m.message.text.starts_with("!addbuddy") {
                    let arg = m.message.text.split_whitespace().nth(1).unwrap_or("");
                    let pack = BuddyAddPacket {
                        character_id: arg.parse().unwrap(),
                    };
                    sock.send(pack).await?;
                } else if m.message.text.starts_with("!join") {
                    let pack = OutPrivgrpInvitePacket {
                        character_id: m.message.sender.unwrap(),
                    };
                    sock.send(pack).await?;
                } else if m.message.text.starts_with("!kickall") {
                    let pack = PrivgrpKickallPacket {};
                    sock.send(pack).await?;
                } else if m.message.text.starts_with("!kick") {
                    let arg = m.message.text.split_whitespace().nth(1).unwrap_or("");
                    let pack = OutPrivgrpKickPacket {
                        character_id: arg.parse().unwrap(),
                    };
                    sock.send(pack).await?;
                } else if m.message.text.starts_with("!leave") {
                    let arg = m.message.text.split_whitespace().nth(1).unwrap_or("");
                    let pack = PrivgrpPartPacket {
                        channel: Channel::PrivateChannel(arg.parse().unwrap()),
                    };
                    sock.send(pack).await?;
                }
            }
            Packet::GroupMessage(m) => {
                println!(
                    "Got a msg from {:?} in {:?} with text {}",
                    m.message.sender, m.message.channel, m.message.text
                )
            }
            Packet::PrivgrpMessage(m) => {
                println!(
                    "Got a msg from {:?} in {:?} with text {}",
                    m.message.sender, m.message.channel, m.message.text
                )
            }
            Packet::ChatNotice(c) => {
                println!(
                    "Got a chat notice from {}: {}",
                    c.notice.sender, c.notice.text
                )
            }
            Packet::ClientLookup(c) => {
                println!("User {} has ID {}", c.character_name, c.character_id)
            }
            Packet::PrivgrpInvite(p) => {
                println!("Got invited to private group {:?}", p.channel);
                let pack = PrivgrpJoinPacket { channel: p.channel };
                sock.send(pack).await?;
            }
            Packet::PrivgrpClijoin(p) => {
                println!(
                    "{} joined the private channel {:?}",
                    p.character_id, p.channel
                )
            }
            Packet::PrivgrpClipart(p) => {
                println!("{} left the private channel", p.character_id)
            }
            Packet::PrivgrpKick(k) => {
                println!("Got kicked from {:?}", k.channel)
            }
            Packet::BuddyRemove(b) => {
                println!("Buddy {} successfully removed", b.character_id)
            }
            Packet::MsgSystem(m) => {
                println!("Got a system message: {}", m.text)
            }
            _ => {}
        }
    }
}
