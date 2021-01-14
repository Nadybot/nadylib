use crate::{
    crypto::generate_key,
    error::Result,
    packets::{
        LoginRequestPacket, OutgoingPacket, PacketType, PingPacket, ReceivedPacket,
        SerializedPacket,
    },
};

use byteorder::{ByteOrder, NetworkEndian};
use leaky_bucket::LeakyBucket;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream, ToSocketAddrs,
    },
    spawn,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        watch::{channel, Receiver, Sender},
    },
    task::JoinHandle,
    time::{timeout_at, Instant},
};

use std::{convert::TryFrom, time::Duration};

/// A send handle for sending packets to an [`AOSocket`].
#[derive(Debug, Clone)]
pub struct SocketSendHandle {
    sender: UnboundedSender<SerializedPacket>,
    ratelimiter: Option<LeakyBucket>,
}

impl SocketSendHandle {
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
        if packet_type == PacketType::MsgPrivate {
            if let Some(limiter) = &self.ratelimiter {
                limiter.acquire_one().await?;
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
    tasks: Vec<JoinHandle<Result<()>>>,
}

async fn send_task(
    mut packet_queue: UnboundedReceiver<SerializedPacket>,
    mut sock: OwnedWriteHalf,
    last_packet: Option<Sender<Instant>>,
) -> Result<()> {
    loop {
        let (packet_type, mut packet_body) = packet_queue.recv().await.unwrap();
        let mut buf = Vec::with_capacity(4 + packet_body.len());
        let mut pkt_type_buf = vec![0; 2];
        let mut pkt_len_buf = vec![0; 2];
        NetworkEndian::write_u16(&mut pkt_type_buf, packet_type as u16);
        NetworkEndian::write_u16(&mut pkt_len_buf, packet_body.len() as u16);
        buf.append(&mut pkt_type_buf);
        buf.append(&mut pkt_len_buf);
        buf.append(&mut packet_body);

        sock.write_all(&buf).await?;

        if let Some(sender) = &last_packet {
            sender.send(Instant::now())?;
        }
    }
}

async fn keepalive(
    sender: UnboundedSender<SerializedPacket>,
    mut last_packet: Receiver<Instant>,
) -> Result<()> {
    let mut last_p = *last_packet.borrow();
    loop {
        let res = timeout_at(last_p + Duration::from_secs(55), last_packet.changed()).await;

        match res {
            Ok(_) => {
                last_p = *last_packet.borrow();
            }
            Err(_) => {
                let pack = PingPacket {
                    client: String::from("nadylib"),
                };
                sender.send(pack.serialize())?;
                last_p = Instant::now();
            }
        }
    }
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
    pub fn keepalive(mut self, value: bool) -> Self {
        self.keepalive = value;
        self
    }
    pub fn limit_tells(mut self, value: bool) -> Self {
        self.limit_tells = value;
        self
    }
}

impl AOSocket {
    /// Opens a TCP connection to the chat server specified in the address.
    pub async fn connect<A: ToSocketAddrs>(addr: A, config: SocketConfig) -> Result<Self> {
        let sock = TcpStream::connect(addr).await?;

        Ok(Self::from_stream(sock, config))
    }

    /// Starts the socket from an existing [`TcpStream`].
    pub fn from_stream(sock: TcpStream, config: SocketConfig) -> Self {
        let (rx, tx) = sock.into_split();
        let (send, recv) = unbounded_channel();

        let mut tasks = Vec::with_capacity(2);

        if config.keepalive {
            let (lp_send, lp_recv) = channel(Instant::now());
            tasks.push(spawn(keepalive(send.clone(), lp_recv)));
            tasks.push(spawn(send_task(recv, tx, Some(lp_send))));
        } else {
            tasks.push(spawn(send_task(recv, tx, None)));
        }

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
            tasks,
            sender,
        }
    }

    /// Wrapper for generating a login key and sending a [`LoginRequestPacket`] to the server.
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

        // The header consists of 4 bytes = 2 unsigned 16 bit integers for packet type and length
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
        for task in self.tasks.iter() {
            task.abort();
        }
    }
}
