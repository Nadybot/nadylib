use num_enum::TryFromPrimitiveError;
use tokio::{
    sync::{mpsc::error::SendError as MpscError, watch::error::SendError as WatchError},
    time::Instant,
};

use std::{io::Error as IoError, result::Result as OrigResult, string::FromUtf8Error};

use crate::packets::{PacketType, SerializedPacket};

/// An error in the library.
#[derive(Debug)]
pub enum Error {
    /// Failed to read from underlying connections.
    IoError(IoError),
    /// Unknown packet encountered.
    UnknownPacket(Option<PacketType>),
    /// Payload malformed or incomplete.
    PayloadError,
    /// Could not queue packet for sending.
    QueueError,
    /// Unable to update last ping time.
    PingError,
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::IoError(e)
    }
}

impl From<TryFromPrimitiveError<PacketType>> for Error {
    fn from(_: TryFromPrimitiveError<PacketType>) -> Self {
        Self::UnknownPacket(None)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Self {
        Self::PayloadError
    }
}

impl From<MpscError<SerializedPacket>> for Error {
    fn from(_: MpscError<SerializedPacket>) -> Self {
        Self::QueueError
    }
}

impl From<WatchError<Instant>> for Error {
    fn from(_: WatchError<Instant>) -> Self {
        Self::PingError
    }
}

/// Wrapper for [`Result`].
///
/// [`Result`]: OrigResult
pub type Result<T> = OrigResult<T, Error>;
