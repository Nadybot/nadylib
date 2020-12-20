use nadylib::{
    client_socket::AOSocket,
    error::Result,
    models::{Channel, Message},
    packets::{
        BuddyAddPacket, BuddyRemovePacket, ClientLookupPacket, GroupMessagePacket,
        LoginSelectPacket, MsgPrivatePacket, OutPrivgrpInvitePacket, OutPrivgrpKickPacket,
        PrivgrpJoinPacket, PrivgrpKickallPacket, PrivgrpMessagePacket, PrivgrpPartPacket,
        ReceivedPacket,
    },
};

use std::env::var;

#[tokio::main]
async fn main() -> Result<()> {
    // This is not actually the aochatproxy
    // but until nadylib is finished
    // only a playground for the library
    let mut sock = AOSocket::connect("chat.d1.funcom.com:7105").await?;
    let mut my_id: u32 = 0;
    let mut groups = Vec::new();

    loop {
        let packet = sock.read_packet().await?;

        match packet {
            ReceivedPacket::LoginSeed(l) => {
                sock.login(
                    &var("AO_ACCOUNT").unwrap(),
                    &var("AO_PASSWORD").unwrap(),
                    &l.login_seed,
                )?;
            }
            ReceivedPacket::LoginError(e) => println!("Error: {}", e.message),
            ReceivedPacket::LoginCharlist(c) => {
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
                sock.send(pack)?;
            }
            ReceivedPacket::LoginOk => println!("Successfully logged in"),
            ReceivedPacket::ClientName(c) => {
                if my_id == 0 {
                    my_id = c.character_id;
                }
                println!("Character {} has ID {}", c.character_name, c.character_id)
            }
            ReceivedPacket::MsgVicinitya(m) => {
                println!("{:?}", m.message);
            }
            ReceivedPacket::BuddyStatus(b) => {
                println!("Buddy {} online? {}", b.character_id, b.online)
            }
            ReceivedPacket::GroupAnnounce(c) => {
                if let Channel::Group(g) = &c.channel {
                    println!("Am in channel {:?} ({}) {:?}", g.name, g.id, g.r#type)
                }
                groups.push(c.channel);
            }
            ReceivedPacket::MsgPrivate(m) => {
                println!(
                    "Got a msg from {:?} in {:?} with text {}",
                    m.message.sender, m.message.channel, m.message.text
                );
                if m.message.text.starts_with("!lookup") {
                    let arg = m.message.text.split_whitespace().nth(1).unwrap_or("");
                    let pack = ClientLookupPacket {
                        character_name: arg.to_owned(),
                    };
                    sock.send(pack)?;
                } else if m.message.text.starts_with("!rembuddy") {
                    let arg = m.message.text.split_whitespace().nth(1).unwrap_or("");
                    let pack = BuddyRemovePacket {
                        character_id: arg.parse().unwrap(),
                    };
                    sock.send(pack)?;
                } else if m.message.text.starts_with("!addbuddy") {
                    let arg = m.message.text.split_whitespace().nth(1).unwrap_or("");
                    let pack = BuddyAddPacket {
                        character_id: arg.parse().unwrap(),
                    };
                    sock.send(pack)?;
                } else if m.message.text.starts_with("!join") {
                    let pack = OutPrivgrpInvitePacket {
                        character_id: m.message.sender.unwrap(),
                    };
                    sock.send(pack)?;
                } else if m.message.text.starts_with("!kickall") {
                    let pack = PrivgrpKickallPacket {};
                    sock.send(pack)?;
                } else if m.message.text.starts_with("!kick") {
                    let arg = m.message.text.split_whitespace().nth(1).unwrap_or("");
                    let pack = OutPrivgrpKickPacket {
                        character_id: arg.parse().unwrap(),
                    };
                    sock.send(pack)?;
                } else if m.message.text.starts_with("!leave") {
                    let arg = m.message.text.split_whitespace().nth(1).unwrap_or("");
                    let pack = PrivgrpPartPacket {
                        channel: Channel::PrivateChannel(arg.parse().unwrap()),
                    };
                    sock.send(pack)?;
                } else if m.message.text.starts_with("!say") {
                    let arg = m.message.text.split_whitespace().nth(1).unwrap_or("");
                    let pack = MsgPrivatePacket {
                        message: Message {
                            sender: None,
                            channel: Channel::Tell(m.message.sender.unwrap()),
                            text: arg.to_owned(),
                        },
                    };
                    sock.send(pack)?;
                }
            }
            ReceivedPacket::GroupMessage(m) => {
                println!(
                    "Got a msg from {:?} in {:?} with text {}",
                    m.message.sender, m.message.channel, m.message.text
                );
                // Echo it back
                if m.message.sender.unwrap() != my_id {
                    let pack = GroupMessagePacket { message: m.message };
                    sock.send(pack)?;
                }
            }
            ReceivedPacket::PrivgrpMessage(m) => {
                println!(
                    "Got a msg from {:?} in {:?} with text {}",
                    m.message.sender, m.message.channel, m.message.text
                );
                if m.message.text.starts_with("!say") {
                    let arg = m.message.text.split_whitespace().nth(1).unwrap_or("");
                    let pack = PrivgrpMessagePacket {
                        message: Message {
                            sender: None,
                            channel: m.message.channel,
                            text: arg.to_owned(),
                        },
                    };
                    sock.send(pack)?;
                }
            }
            ReceivedPacket::ChatNotice(c) => {
                println!(
                    "Got a chat notice from {}: {}",
                    c.notice.sender, c.notice.text
                )
            }
            ReceivedPacket::ClientLookup(c) => {
                println!("User {} has ID {}", c.character_name, c.character_id)
            }
            ReceivedPacket::PrivgrpInvite(p) => {
                println!("Got invited to private group {:?}", p.channel);
                let pack = PrivgrpJoinPacket { channel: p.channel };
                sock.send(pack)?;
            }
            ReceivedPacket::PrivgrpClijoin(p) => {
                println!(
                    "{} joined the private channel {:?}",
                    p.character_id, p.channel
                )
            }
            ReceivedPacket::PrivgrpClipart(p) => {
                println!("{} left the private channel", p.character_id)
            }
            ReceivedPacket::PrivgrpKick(k) => {
                println!("Got kicked from {:?}", k.channel)
            }
            ReceivedPacket::BuddyRemove(b) => {
                println!("Buddy {} successfully removed", b.character_id)
            }
            ReceivedPacket::MsgSystem(m) => {
                println!("Got a system message: {}", m.text)
            }
            ReceivedPacket::Ping(p) => {
                println!("Got a ping from {}", p.client)
            }
        }
    }
}
