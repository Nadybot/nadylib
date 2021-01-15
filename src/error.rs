use leaky_bucket_lite::Error as BucketError;
use tokio::{
    sync::{mpsc::error::SendError as MpscError, watch::error::SendError as WatchError},
    time::Instant,
};

use std::{io::Error as IoError, result::Result as OrigResult, string::FromUtf8Error};

use crate::packets::SerializedPacket;

/// An error in the library.
#[derive(Debug)]
pub enum Error {
    /// Failed to read from underlying connections.
    IoError(IoError),
    /// Unknown packet encountered.
    UnknownPacket(u16),
    /// Payload malformed or incomplete.
    PayloadError,
    /// Could not queue packet for sending.
    QueueError,
    /// Unable to update last ping time.
    PingError,
    /// The ratelimiter failed to acquire.
    RatelimitError,
    /// Failed to parse channel type.
    UnknownChannelType(u8),
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::IoError(e)
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

impl From<BucketError> for Error {
    fn from(_: BucketError) -> Self {
        Self::RatelimitError
    }
}

/// Wrapper for [`Result`].
///
/// [`Result`]: OrigResult
pub type Result<T> = OrigResult<T, Error>;
