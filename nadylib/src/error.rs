use num_enum::TryFromPrimitiveError;
use tokio::{
    sync::{mpsc::error::SendError as MpscError, watch::error::SendError as WatchError},
    time::Instant,
};

use std::{io::Error as IoError, result::Result as OrigResult, string::FromUtf8Error};

use crate::packets::PacketType;

#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    UnknownPacket(Option<PacketType>),
    PayloadError,
    QueueError,
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

impl From<MpscError<(PacketType, Vec<u8>)>> for Error {
    fn from(_: MpscError<(PacketType, Vec<u8>)>) -> Self {
        Self::QueueError
    }
}

impl From<WatchError<Instant>> for Error {
    fn from(_: WatchError<Instant>) -> Self {
        Self::PingError
    }
}

pub type Result<T> = OrigResult<T, Error>;
