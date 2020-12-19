use crate::crypto::generate_key;
use crate::error::Result;
use crate::packets::{LoginRequestPacket, OutgoingPacket, Packet, PacketType};

use byteorder::{ByteOrder, NetworkEndian};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs};

use std::convert::TryFrom;

pub struct AOSocket {
    socket: TcpStream,
}

impl AOSocket {
    /// Opens a TCP connection to the chat server specified in the address.
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let sock = TcpStream::connect(addr).await?;
        Ok(Self { socket: sock })
    }

    /// Sends a login packet to the server.
    pub async fn login(&mut self, username: &str, password: &str, login_seed: &str) -> Result<()> {
        let key = generate_key(username, password, login_seed);
        let packet = LoginRequestPacket {
            username: username.to_owned(),
            key,
        };
        self.send(packet).await?;

        Ok(())
    }

    /// Sends a `Packet` over the TCP connection.
    pub async fn send<O: OutgoingPacket>(&mut self, packet: O) -> Result<()> {
        let (packet_type, mut packet_body) = packet.serialize();
        let mut buf = Vec::with_capacity(4 + packet_body.len());
        let mut pkt_type_buf = vec![0; 2];
        let mut pkt_len_buf = vec![0; 2];
        NetworkEndian::write_u16(&mut pkt_type_buf, packet_type as u16);
        NetworkEndian::write_u16(&mut pkt_len_buf, packet_body.len() as u16);
        buf.append(&mut pkt_type_buf);
        buf.append(&mut pkt_len_buf);
        buf.append(&mut packet_body);

        self.socket.write_all(&buf).await?;
        Ok(())
    }

    /// Attempts to read an entire packet from the underlying connection.
    /// Returns a `Packet` or an `Error` if reading failed.
    pub async fn read_packet(&mut self) -> Result<Packet> {
        let mut header_buf = [0; 4];
        self.socket.read_exact(&mut header_buf).await?;

        // The header consists of 4 bytes = 2 unsigned 16 bit integers for packet type and length
        let packet_type_int = NetworkEndian::read_u16(&header_buf[0..2]);
        let packet_length = NetworkEndian::read_u16(&header_buf[2..4]);

        let packet_type = PacketType::try_from(packet_type_int)?;

        // Read the rest of the packet
        let mut packet_body = vec![0; packet_length as usize];
        self.socket.read_exact(&mut packet_body).await?;

        let packet = Packet::try_from((packet_type, packet_body.as_slice()))?;

        Ok(packet)
    }
}
