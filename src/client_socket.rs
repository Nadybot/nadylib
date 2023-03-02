use std::{convert::TryFrom, time::Duration};

use byteorder::{ByteOrder, NetworkEndian};
use leaky_bucket_lite::LeakyBucket;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream, ToSocketAddrs,
    },
    spawn,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    task::JoinHandle,
    time::timeout,
};

use crate::{
    crypto::generate_key,
    error::Result,
    packets::{
        LoginRequestPacket, OutgoingPacket, PacketType, PingPacket, ReceivedPacket,
        SerializedPacket,
    },
};

/// A send handle for sending packets to an [`AOSocket`].
#[derive(Debug, Clone)]
pub struct SocketSendHandle {
    sender: UnboundedSender<SerializedPacket>,
    ratelimiter: Option<LeakyBucket>,
}

impl SocketSendHandle {
    #[must_use]
    pub fn new(
        sender: UnboundedSender<SerializedPacket>,
        ratelimiter: Option<LeakyBucket>,
    ) -> Self {
        Self {
            sender,
            ratelimiter,
        }
    }

    pub async fn send<O: OutgoingPacket>(&self, packet: O) -> Result<()> {
        let (t, b) = packet.serialize();
        self.send_raw(t, b).await?;
        Ok(())
    }

    pub async fn send_raw(&self, packet_type: PacketType, body: Vec<u8>) -> Result<()> {
        if packet_type == PacketType::MsgPrivate || packet_type == PacketType::GroupMessage {
            if let Some(limiter) = &self.ratelimiter {
                limiter.acquire_one().await;
            }
        }

        self.sender.send((packet_type, body))?;
        Ok(())
    }
}

/// A TCP connection to the Funcom servers.
#[derive(Debug)]
pub struct AOSocket {
    read_half: OwnedReadHalf,
    sender: SocketSendHandle,
    send_task: Option<JoinHandle<(UnboundedReceiver<SerializedPacket>, bool)>>,
    shutdown: Option<oneshot::Sender<()>>,
}

async fn write_packet(sock: &mut OwnedWriteHalf, mut packet: SerializedPacket) -> Result<()> {
    let mut buf = vec![0; 4];
    NetworkEndian::write_u16(&mut buf[..2], packet.0 as u16);
    NetworkEndian::write_u16(&mut buf[2..4], packet.1.len() as u16);
    buf.append(&mut packet.1);

    sock.write_all(&buf).await?;

    Ok(())
}

async fn send_task(
    mut shutdown: oneshot::Receiver<()>,
    mut packet_queue: UnboundedReceiver<SerializedPacket>,
    mut sock: OwnedWriteHalf,
    keep_alive: bool,
) -> (UnboundedReceiver<SerializedPacket>, bool) {
    loop {
        tokio::select! {
            maybe_packet = timeout(Duration::from_secs(55), packet_queue.recv()) => {
                match maybe_packet {
                    Ok(Some(packet)) => {
                        if write_packet(&mut sock, packet).await.is_err() {
                            break;
                        }
                    },
                    Ok(None) => break,
                    Err(_) => {
                        if keep_alive {
                            let packet = PingPacket {
                                client: String::from("nadylib"),
                            };
                            if write_packet(&mut sock, packet.serialize()).await.is_err() {
                                break;
                            }
                        }
                    }
                }
            },
            _ = &mut shutdown => {
                break;
            }
        }
    }

    (packet_queue, keep_alive)
}

#[derive(Debug)]
pub struct SocketConfig {
    keepalive: bool,
    limit_tells: bool,
}

impl Default for SocketConfig {
    fn default() -> Self {
        Self {
            keepalive: true,
            limit_tells: true,
        }
    }
}

impl SocketConfig {
    #[must_use]
    pub fn keepalive(mut self, value: bool) -> Self {
        self.keepalive = value;
        self
    }

    #[must_use]
    pub fn limit_tells(mut self, value: bool) -> Self {
        self.limit_tells = value;
        self
    }
}

impl AOSocket {
    /// Opens a TCP connection to the chat server specified in the address.
    pub async fn connect<A: ToSocketAddrs>(addr: A, config: &SocketConfig) -> Result<Self> {
        let sock = TcpStream::connect(addr).await?;

        Ok(Self::from_stream(sock, config))
    }

    /// Starts the socket from an existing [`TcpStream`].
    pub fn from_stream(sock: TcpStream, config: &SocketConfig) -> Self {
        let (rx, tx) = sock.into_split();

        let (send, recv) = unbounded_channel();
        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        let send_task = spawn(send_task(shutdown_rx, recv, tx, config.keepalive));

        let sender = {
            if config.limit_tells {
                SocketSendHandle::new(
                    send,
                    Some(
                        LeakyBucket::builder()
                            .refill_amount(1)
                            .refill_interval(Duration::from_secs(2))
                            .max(5)
                            .tokens(5)
                            .build(),
                    ),
                )
            } else {
                SocketSendHandle::new(send, None)
            }
        };

        Self {
            read_half: rx,
            sender,
            send_task: Some(send_task),
            shutdown: Some(shutdown_tx),
        }
    }

    pub async fn reconnect(&mut self) -> Result<()> {
        let sock = TcpStream::connect(self.read_half.peer_addr().unwrap()).await?;

        // Cancel the queue -> tcp writing task
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }

        if let Some(send_task_handle) = self.send_task.take() {
            let (recv, keepalive) = send_task_handle.await.unwrap();
            let (rx, tx) = sock.into_split();
            let (shutdown_tx, shutdown_rx) = oneshot::channel();
            let send_task = spawn(send_task(shutdown_rx, recv, tx, keepalive));

            self.read_half = rx;
            self.send_task = Some(send_task);
            self.shutdown = Some(shutdown_tx);
        }

        Ok(())
    }

    /// Wrapper for generating a login key and sending a [`LoginRequestPacket`]
    /// to the server.
    pub async fn login(&self, username: &str, password: &str, login_seed: &str) -> Result<()> {
        let key = generate_key(username, password, login_seed);
        let packet = LoginRequestPacket {
            username: username.to_owned(),
            key,
        };
        self.send(packet).await?;

        Ok(())
    }

    /// Gets a handle to a send end of the internal receiver.
    #[must_use]
    pub fn get_sender(&self) -> SocketSendHandle {
        self.sender.clone()
    }

    /// Queues sending an [`OutgoingPacket`] over the TCP connection.
    pub async fn send<O: OutgoingPacket>(&self, packet: O) -> Result<()> {
        self.sender.send(packet).await?;
        Ok(())
    }

    /// Queues sending a raw packet over the TCP connection.
    pub async fn send_raw(&self, packet_type: PacketType, body: Vec<u8>) -> Result<()> {
        self.sender.send_raw(packet_type, body).await?;
        Ok(())
    }

    /// Attempts to read a raw packet from the underlying connection.
    pub async fn read_raw_packet(&mut self) -> Result<SerializedPacket> {
        let mut header_buf = [0; 4];
        self.read_half.read_exact(&mut header_buf).await?;

        // The header consists of 4 bytes = 2 unsigned 16 bit integers for packet type
        // and length
        let packet_type_int = NetworkEndian::read_u16(&header_buf[0..2]);
        let packet_length = NetworkEndian::read_u16(&header_buf[2..4]);

        let packet_type = PacketType::try_from(packet_type_int)?;

        // Read the rest of the packet
        let mut packet_body = vec![0; packet_length as usize];
        self.read_half.read_exact(&mut packet_body).await?;

        Ok((packet_type, packet_body))
    }

    /// Attempts to read an entire packet from the underlying connection.
    /// Returns a [`ReceivedPacket`] or an [`IoError`] if reading failed.
    ///
    /// [`IoError`]: crate::error::Error
    pub async fn read_packet(&mut self) -> Result<ReceivedPacket> {
        let raw = self.read_raw_packet().await?;

        let packet = ReceivedPacket::try_from((raw.0, raw.1.as_slice()))?;

        Ok(packet)
    }
}

impl Drop for AOSocket {
    fn drop(&mut self) {
        if let Some(send_task) = &self.send_task {
            send_task.abort();
        };
    }
}
