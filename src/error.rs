// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use quinn::crypto::rustls;
use std::fmt::{self, Display};
use std::io;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite;

use crate::commands::{
    AuthToListenerCmd, DispatcherToMetricsCmd, ListenerToAuthCmd, ListenerToDispatcherCmd,
    ListenerToSessionCmd, MetricsToDispatcherCmd, SessionToListenerCmd,
};
use crate::types::SessionId;

/// Represent the types of errors.
#[derive(Clone, Debug)]
pub enum ErrorKind {
    /// Error occurred while performing I/O.
    IoError,

    /// Packet encode error.
    EncodeError,

    /// Packet decode error.
    DecodeError,

    /// Send packet error.
    SendError,

    /// Socket stream error.
    SocketError,

    /// Cert files error.
    CertError,

    /// Invalid pid.
    PidError,

    /// Session with id not found in pipelines.
    SessionNotFound,

    /// mpsc channel error.
    ChannelError,

    /// Invalid config file.
    ConfigError,

    LoggerError,

    SSLError,

    /// Command line parameter error.
    ParameterError,

    /// File format error.
    FormatError,

    RedisError,

    MySQLError,

    PgSQLError,

    MongoError,
}

#[derive(Clone, Debug)]
pub struct Error {
    /// Type of current error.
    kind: ErrorKind,

    /// Detail message about this error.
    message: String,
}

impl Error {
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        Error {
            kind,
            message: message.to_owned(),
        }
    }

    pub fn from_string(kind: ErrorKind, message: String) -> Self {
        Error { kind, message }
    }
}

impl Error {
    pub fn session_error(session_id: SessionId) -> Self {
        Error::from_string(
            ErrorKind::SessionNotFound,
            format!("Session with id {} not found", session_id),
        )
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::from_string(ErrorKind::IoError, format!("IoError {}", err))
    }
}

impl From<tungstenite::Error> for Error {
    fn from(err: tungstenite::Error) -> Self {
        Error::from_string(ErrorKind::SocketError, format!("Websocket error: {}", err))
    }
}

impl From<quinn::EndpointError> for Error {
    fn from(err: quinn::EndpointError) -> Self {
        Error::from_string(
            ErrorKind::SocketError,
            format!("Quic endpoint error: {}", err),
        )
    }
}

impl From<quinn::ConnectionError> for Error {
    fn from(err: quinn::ConnectionError) -> Self {
        Error::from_string(
            ErrorKind::SocketError,
            format!("Quic connection error: {}", err),
        )
    }
}

impl From<quinn::ParseError> for Error {
    fn from(err: quinn::ParseError) -> Self {
        Error::from_string(
            ErrorKind::CertError,
            format!("Quic parse cert failed: {}", err),
        )
    }
}

impl From<rustls::TLSError> for Error {
    fn from(err: rustls::TLSError) -> Self {
        Error::from_string(
            ErrorKind::CertError,
            format!("Rustls parse cert failed: {}", err),
        )
    }
}

impl From<quinn::WriteError> for Error {
    fn from(err: quinn::WriteError) -> Self {
        Error::from_string(ErrorKind::SocketError, format!("Quic write error: {}", err))
    }
}

impl From<openssl::error::ErrorStack> for Error {
    fn from(err: openssl::error::ErrorStack) -> Self {
        Error::from_string(ErrorKind::SSLError, format!("{:?}", err))
    }
}

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Self {
        Error::from_string(ErrorKind::FormatError, format!("{:?}", err))
    }
}

impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        Error::from_string(ErrorKind::RedisError, format!("{:?}", err))
    }
}

impl From<mysql_async::Error> for Error {
    fn from(err: mysql_async::Error) -> Self {
        Error::from_string(ErrorKind::MySQLError, format!("{:?}", err))
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(err: tokio_postgres::Error) -> Self {
        Error::from_string(ErrorKind::PgSQLError, format!("{:?}", err))
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(err: mongodb::error::Error) -> Self {
        Error::from_string(ErrorKind::MongoError, format!("{:?}", err))
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::from_string(ErrorKind::ConfigError, format!("{:?}", err))
    }
}

// Internal error convertions.
impl From<codec::EncodeError> for Error {
    fn from(err: codec::EncodeError) -> Self {
        Error::from_string(ErrorKind::EncodeError, format!("{:?}", err))
    }
}

impl From<codec::DecodeError> for Error {
    fn from(err: codec::DecodeError) -> Self {
        Error::from_string(ErrorKind::DecodeError, format!("{:?}", err))
    }
}

macro_rules! convert_send_error {
    ($cmd_type: ident) => {
        impl From<mpsc::error::SendError<$cmd_type>> for Error {
            fn from(err: mpsc::error::SendError<$cmd_type>) -> Self {
                Error::from_string(
                    ErrorKind::ChannelError,
                    format!("$cmd_type channel error: {}", err),
                )
            }
        }
    };
}

convert_send_error!(AuthToListenerCmd);
convert_send_error!(ListenerToAuthCmd);
convert_send_error!(ListenerToDispatcherCmd);
convert_send_error!(ListenerToSessionCmd);
convert_send_error!(SessionToListenerCmd);
convert_send_error!(MetricsToDispatcherCmd);
convert_send_error!(DispatcherToMetricsCmd);
