use crate::{
    crypto::generate_key,
    error::Result,
    packets::{
        LoginRequestPacket, OutgoingPacket, PacketType, PingPacket, ReceivedPacket,
        SerializedPacket,
    },
};

use byteorder::{ByteOrder, NetworkEndian};
use leaky_bucket_lite::sync_threadsafe::LeakyBucket;

use std::{
    convert::TryFrom,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    sync::mpsc::{channel, Receiver, Sender},
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
};

/// A send handle for sending packets to an [`AOSocket`].
#[derive(Debug, Clone)]
pub struct SocketSendHandle {
    sender: Sender<SerializedPacket>,
    ratelimiter: Option<LeakyBucket>,
}

impl SocketSendHandle {
    pub fn new(sender: Sender<SerializedPacket>, ratelimiter: Option<LeakyBucket>) -> Self {
        Self {
            sender,
            ratelimiter,
        }
    }

    pub fn send<O: OutgoingPacket>(&self, packet: O) -> Result<()> {
        let (t, b) = packet.serialize();
        self.send_raw(t, b)?;
        Ok(())
    }

    pub fn send_raw(&self, packet_type: PacketType, body: Vec<u8>) -> Result<()> {
        if packet_type == PacketType::MsgPrivate || packet_type == PacketType::GroupMessage {
            if let Some(limiter) = &self.ratelimiter {
                limiter.acquire_one();
            }
        }

        self.sender.send((packet_type, body))?;
        Ok(())
    }
}

/// A TCP connection to the Funcom servers.
#[derive(Debug)]
pub struct AOSocket {
    stream: TcpStream,
    sender: SocketSendHandle,
    tasks: Vec<JoinHandle<Result<()>>>,
}

fn send_task(packet_queue: Receiver<SerializedPacket>, mut sock: TcpStream) -> Result<()> {
    loop {
        let (packet_type, mut packet_body) = packet_queue.recv().unwrap();
        let mut buf = Vec::with_capacity(4 + packet_body.len());
        let mut pkt_type_buf = vec![0; 2];
        let mut pkt_len_buf = vec![0; 2];
        NetworkEndian::write_u16(&mut pkt_type_buf, packet_type as u16);
        NetworkEndian::write_u16(&mut pkt_len_buf, packet_body.len() as u16);
        buf.append(&mut pkt_type_buf);
        buf.append(&mut pkt_len_buf);
        buf.append(&mut packet_body);

        sock.write_all(&buf)?;
    }
}

fn keepalive(sender: Sender<SerializedPacket>) -> Result<()> {
    loop {
        sleep(Duration::from_secs(55));

        let pack = PingPacket {
            client: String::from("nadylib"),
        };
        sender.send(pack.serialize())?;
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
    pub fn connect<A: ToSocketAddrs>(addr: A, config: SocketConfig) -> Result<Self> {
        let sock = TcpStream::connect(addr)?;

        Self::from_stream(sock, config)
    }

    /// Starts the socket from an existing [`TcpStream`].
    pub fn from_stream(sock: TcpStream, config: SocketConfig) -> Result<Self> {
        let (send, recv) = channel();

        let mut tasks = Vec::new();

        if config.keepalive {
            let send = send.clone();
            tasks.push(spawn(move || keepalive(send)));
        }

        let sock_clone = sock.try_clone()?;
        tasks.push(spawn(move || send_task(recv, sock_clone)));

        let sender = {
            if config.limit_tells {
                SocketSendHandle::new(
                    send,
                    Some(
                        LeakyBucket::builder()
                            .refill_amount(1.0)
                            .refill_interval(Duration::from_secs(2))
                            .max(5.0)
                            .tokens(5.0)
                            .build(),
                    ),
                )
            } else {
                SocketSendHandle::new(send, None)
            }
        };

        Ok(Self {
            stream: sock,
            sender,
            tasks,
        })
    }

    /// Wrapper for generating a login key and sending a [`LoginRequestPacket`] to the server.
    pub fn login(&self, username: &str, password: &str, login_seed: &str) -> Result<()> {
        let key = generate_key(username, password, login_seed);
        let packet = LoginRequestPacket {
            username: username.to_owned(),
            key,
        };
        self.send(packet)?;

        Ok(())
    }

    /// Gets a handle to a send end of the internal receiver.
    pub fn get_sender(&self) -> SocketSendHandle {
        self.sender.clone()
    }

    /// Queues sending an [`OutgoingPacket`] over the TCP connection.
    pub fn send<O: OutgoingPacket>(&self, packet: O) -> Result<()> {
        self.sender.send(packet)?;
        Ok(())
    }

    /// Queues sending a raw packet over the TCP connection.
    pub fn send_raw(&self, packet_type: PacketType, body: Vec<u8>) -> Result<()> {
        self.sender.send_raw(packet_type, body)?;
        Ok(())
    }

    /// Attempts to read a raw packet from the underlying connection.
    pub fn read_raw_packet(&mut self) -> Result<SerializedPacket> {
        let mut header_buf = [0; 4];
        self.stream.read_exact(&mut header_buf)?;

        // The header consists of 4 bytes = 2 unsigned 16 bit integers for packet type and length
        let packet_type_int = NetworkEndian::read_u16(&header_buf[0..2]);
        let packet_length = NetworkEndian::read_u16(&header_buf[2..4]);

        let packet_type = PacketType::try_from(packet_type_int)?;

        // Read the rest of the packet
        let mut packet_body = vec![0; packet_length as usize];
        self.stream.read_exact(&mut packet_body)?;

        Ok((packet_type, packet_body))
    }

    /// Attempts to read an entire packet from the underlying connection.
    /// Returns a [`ReceivedPacket`] or an [`IoError`] if reading failed.
    ///
    /// [`IoError`]: crate::error::Error
    pub fn read_packet(&mut self) -> Result<ReceivedPacket> {
        let raw = self.read_raw_packet()?;

        let packet = ReceivedPacket::try_from((raw.0, raw.1.as_slice()))?;

        Ok(packet)
    }
}

impl Drop for AOSocket {
    fn drop(&mut self) {
        while self.tasks.get(0).is_some() {
            let task = self.tasks.swap_remove(0);

            #[cfg(windows)]
            unsafe {
                stop_thread::kill_thread_forcibly_exit_code(task, 1)
            }
            #[cfg(unix)]
            unsafe {
                stop_thread::kill_thread_graceful(task)
            }
        }
    }
}
