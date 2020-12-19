use num_enum::TryFromPrimitiveError;

use std::result::Result as OrigResult;
use std::{io::Error as IoError, string::FromUtf8Error};

use crate::packets::PacketType;

#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    UnknownPacket(Option<PacketType>),
    PayloadError,
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

pub type Result<T> = OrigResult<T, Error>;
