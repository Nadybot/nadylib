use nadylib::client_socket::AOSocket;
use nadylib::error::Result;
use nadylib::models::Channel;
use nadylib::packets::{ClientLookupPacket, LoginSelectPacket, Packet};

use std::env::var;

#[tokio::main]
async fn main() -> Result<()> {
    // This is not actually the aochatproxy
    // but until nadylib is finished
    // only a playground for the library
    let mut sock = AOSocket::connect("chat.d1.funcom.com:7105").await?;

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
                println!("Character {} has ID {}", c.character_name, c.character_id)
            }
            Packet::MsgVicinitya(m) => {
                println!("{:?}", m.message);
            }
            Packet::BuddyAdd(b) => {
                println!("Buddy {} online? {}", b.character_id, b.online)
            }
            Packet::GroupAnnounce(c) => match c.channel {
                Channel::Group(g) => {
                    println!("Am in channel {:?} ({}) {:?}", g.name, g.id, g.r#type)
                }
                _ => {}
            },
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
                }
            }
            Packet::GroupMessage(m) => {
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
            _ => {}
        }
    }
}
