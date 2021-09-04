use nadylib::{
    packets::LoginSelectPacket,
    sync_client_socket::{AOSocket, SocketConfig},
    ReceivedPacket, Result,
};

fn main() -> Result<()> {
    let char_name = std::env::var("CHAR_NAME").unwrap();
    let username = std::env::var("USERNAME").unwrap();
    let password = std::env::var("PASSWORD").unwrap();
    let mut sock = AOSocket::connect("chat.d1.funcom.com:7105", SocketConfig::default())?;

    while let Ok(packet) = sock.read_packet() {
        match packet {
            ReceivedPacket::LoginSeed(s) => {
                sock.login(&username, &password, &s.login_seed)?;
            }
            ReceivedPacket::LoginCharlist(c) => {
                let character = c.characters.iter().find(|i| i.name == char_name).unwrap();
                let pack = LoginSelectPacket {
                    character_id: character.id,
                };
                sock.send(pack)?;
            }
            _ => println!("{:?}", packet),
        }
    }

    Ok(())
}
