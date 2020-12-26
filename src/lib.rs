//! Low-level library for the Anarchy Online Chat Protocol and interacting with the chat servers based on tokio.

/// User-friendly bot wrapper class (WIP).
#[allow(dead_code)]
mod bot;
/// Networking structs and methods for connecting to the chat servers.
pub mod client_socket;
/// Cryptographic helpers for generating login keys.
pub mod crypto;
/// Error and Result types for the library.
pub mod error;
#[cfg(feature = "mmdb")]
/// Lightweight version of a parser for the in-game message database in JSON format.
pub mod mmdb;
#[cfg(feature = "mmdb-parser")]
/// Parser for the in-game message database.
pub mod mmdb_parser;
/// Datatypes used in the game.
pub mod models;
/// Packets for receiving and sending data to the servers.
pub mod packets;

// pub use bot::Bot;
pub use client_socket::{AOSocket, SocketConfig};
pub use error::Result;
pub use packets::ReceivedPacket;
