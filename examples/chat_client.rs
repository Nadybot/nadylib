use nadylib::{
    packets::{LoginSelectPacket, PrivgrpJoinPacket},
    AOSocket, ReceivedPacket, Result, SocketConfig,
};

#[tokio::main]
async fn main() -> Result<()> {
    let char_name = std::env::var("CHAR_NAME").unwrap();
    let username = std::env::var("USERNAME").unwrap();
    let password = std::env::var("PASSWORD").unwrap();
    let mut sock = AOSocket::connect("chat.d1.funcom.com:7105", &SocketConfig::default()).await?;

    while let Ok(packet) = sock.read_packet().await {
        match packet {
            ReceivedPacket::LoginSeed(s) => {
                sock.login(&username, &password, &s.login_seed).await?;
            }
            ReceivedPacket::LoginCharlist(c) => {
                let character = c.characters.iter().find(|i| i.name == char_name).unwrap();
                let pack = LoginSelectPacket {
                    character_id: character.id,
                };
                sock.send(pack).await?;
            }
            ReceivedPacket::LoginOk => println!("Logged in successfully"),
            ReceivedPacket::MsgPrivate(m) => {
                println!("Got a private message: {m:?}");
            }
            ReceivedPacket::PrivgrpInvite(p) => {
                println!("Got a private channel invite: {:?}", p.channel);
                println!("Joining it");
                let pack = PrivgrpJoinPacket { channel: p.channel };
                sock.send(pack).await?;
            }
            _ => {}
        }
    }

    Ok(())
}
