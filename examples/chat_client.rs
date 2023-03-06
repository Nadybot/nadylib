use std::time::Duration;

use nadylib::{
    account::{AccountManager, AccountManagerHttpClient},
    packets::{LoginSelectPacket, PrivgrpJoinPacket},
    AOSocket, ReceivedPacket, Result, SocketConfig,
};
use tokio::time::sleep;

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
            ReceivedPacket::LoginError(e) => {
                eprintln!("Failed to log in due to {}", e.message);

                if e.message.contains("Account system denies login") {
                    if let Ok(unfreeze_result) =
                        AccountManager::from_client(AccountManagerHttpClient::new(true))
                            .username(username.clone())
                            .password(password.clone())
                            .reactivate()
                            .await
                    {
                        if unfreeze_result.should_continue() {
                            println!("Unfroze account, waiting 5 seconds before reconnecting");
                            sleep(Duration::from_secs(5)).await;
                            sock.reconnect().await?;
                            continue;
                        }
                    }
                }

                break;
            }
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
