// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use quinn::crypto::rustls;
use std::fmt::{self, Display};
use std::io;
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::tungstenite;

use crate::commands::{
    AuthToListenerCmd, DispatcherToMetricsCmd, ListenerToAclCmd, ListenerToAuthCmd,
    ListenerToDispatcherCmd, ListenerToSessionCmd, MetricsToDispatcherCmd,
    ServerContextToMetricsCmd, SessionToListenerCmd,
};
use crate::types::SessionId;

/// Represent the types of errors.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub enum ErrorKind {
    /// Got error while processing system calls.
    KernelError,

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

    /// Invalid session/client status.
    StatusError,

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
    #[must_use]
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        Self {
            kind,
            message: message.to_owned(),
        }
    }

    #[must_use]
    pub const fn from_string(kind: ErrorKind, message: String) -> Self {
        Self { kind, message }
    }
}

impl Error {
    #[must_use]
    pub fn session_error(session_id: SessionId) -> Self {
        Self::from_string(
            ErrorKind::SessionNotFound,
            format!("Session with id {session_id} not found"),
        )
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for Error {}

impl From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Self {
        Self::from_string(ErrorKind::ConfigError, format!("Invalid ip address, {err}"))
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::from_string(ErrorKind::IoError, format!("IoError {err}"))
    }
}

impl From<tungstenite::Error> for Error {
    fn from(err: tungstenite::Error) -> Self {
        Self::from_string(ErrorKind::SocketError, format!("Websocket error: {err}"))
    }
}

impl From<quinn::ReadError> for Error {
    fn from(err: quinn::ReadError) -> Self {
        Self::from_string(ErrorKind::SocketError, format!("Quic read error: {err:?}"))
    }
}

impl From<quinn::WriteError> for Error {
    fn from(err: quinn::WriteError) -> Self {
        Self::from_string(ErrorKind::SocketError, format!("Quic write error: {err:?}"))
    }
}

impl From<quinn::ConnectionError> for Error {
    fn from(err: quinn::ConnectionError) -> Self {
        Self::from_string(
            ErrorKind::SocketError,
            format!("Quic connection error: {err}"),
        )
    }
}

//impl From<quinn::ParseError> for Error {
//    fn from(err: quinn::ParseError) -> Self {
//        Error::from_string(
//            ErrorKind::CertError,
//            format!("Quic parse cert failed: {err}"),
//        )
//    }
//}

impl From<rustls::Error> for Error {
    fn from(err: rustls::Error) -> Self {
        Self::from_string(ErrorKind::CertError, format!("Rustls error: {err:?}"))
    }
}

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Self {
        Self::from_string(ErrorKind::FormatError, format!("{err:?}"))
    }
}

#[cfg(feature = "redis_conn")]
impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        Self::from_string(ErrorKind::RedisError, format!("{err:?}"))
    }
}

#[cfg(feature = "mysql_conn")]
impl From<mysql_async::Error> for Error {
    fn from(err: mysql_async::Error) -> Self {
        Self::from_string(ErrorKind::MySQLError, format!("{err:?}"))
    }
}

#[cfg(feature = "pgsql_conn")]
impl From<tokio_postgres::Error> for Error {
    fn from(err: tokio_postgres::Error) -> Self {
        Self::from_string(ErrorKind::PgSQLError, format!("{err:?}"))
    }
}

#[cfg(feature = "mongodb_conn")]
impl From<mongodb::error::Error> for Error {
    fn from(err: mongodb::error::Error) -> Self {
        Self::from_string(ErrorKind::MongoError, format!("{err:?}"))
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Self::from_string(ErrorKind::ConfigError, format!("{err:?}"))
    }
}

// Internal error convertions.
impl From<codec::EncodeError> for Error {
    fn from(err: codec::EncodeError) -> Self {
        Self::from_string(ErrorKind::EncodeError, format!("{err:?}"))
    }
}

impl From<codec::DecodeError> for Error {
    fn from(err: codec::DecodeError) -> Self {
        Self::from_string(ErrorKind::DecodeError, format!("{err:?}"))
    }
}

impl From<oneshot::error::RecvError> for Error {
    fn from(err: oneshot::error::RecvError) -> Self {
        Self::from_string(
            ErrorKind::ChannelError,
            format!("$cmd_type channel error: {err}"),
        )
    }
}

impl From<quinn::ClosedStream> for Error {
    fn from(err: quinn::ClosedStream) -> Self {
        Self::from_string(
            ErrorKind::IoError,
            format!("Quic closed stream err: {err:?}"),
        )
    }
}

impl From<rustls_pki_types::pem::Error> for Error {
    fn from(err: rustls_pki_types::pem::Error) -> Self {
        Self::from_string(ErrorKind::CertError, format!("cert error: {err:?}"))
    }
}

macro_rules! convert_send_error {
    ($cmd_type: ident) => {
        impl From<mpsc::error::SendError<$cmd_type>> for Error {
            fn from(err: mpsc::error::SendError<$cmd_type>) -> Self {
                Error::from_string(
                    ErrorKind::ChannelError,
                    format!("$cmd_type channel error: {err}"),
                )
            }
        }
    };
}

convert_send_error!(AuthToListenerCmd);
convert_send_error!(DispatcherToMetricsCmd);
convert_send_error!(ListenerToAclCmd);
convert_send_error!(ListenerToAuthCmd);
convert_send_error!(ListenerToDispatcherCmd);
convert_send_error!(ListenerToSessionCmd);
convert_send_error!(MetricsToDispatcherCmd);
convert_send_error!(ServerContextToMetricsCmd);
convert_send_error!(SessionToListenerCmd);
