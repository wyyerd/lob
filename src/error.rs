use crate::model::LobError;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn bad_request<S: Into<String>>(msg: S) -> Error {
        Error {
            kind: ErrorKind::BadRequest(msg.into()),
        }
    }
}

#[derive(Debug)]
enum ErrorKind {
    Lob(LobError),
    Http(reqwest::Error),
    Serde(SerdeError),
    BadRequest(String),
}

#[derive(Debug)]
enum SerdeError {
    Json(serde_json::Error),
    Qs(serde_qs::Error),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::Lob(LobError {
                message,
                status_code,
            }) => write!(
                f,
                "Lob error - status_code: {}, message: {}",
                message, status_code
            ),
            ErrorKind::Http(err) => write!(f, "Lob error (reqwest) - {}", err),
            ErrorKind::Serde(SerdeError::Json(err)) => {
                write!(f, "Lob error (serde_json) - {}", err)
            }
            ErrorKind::Serde(SerdeError::Qs(err)) => write!(f, "Lob error (serde_qs) - {}", err),
            ErrorKind::BadRequest(msg) => write!(f, "Lob error (bad request) - {}", msg),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error {
            kind: ErrorKind::Http(err),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error {
            kind: ErrorKind::Serde(SerdeError::Json(err)),
        }
    }
}

impl From<serde_qs::Error> for Error {
    fn from(err: serde_qs::Error) -> Self {
        Error {
            kind: ErrorKind::Serde(SerdeError::Qs(err)),
        }
    }
}

impl From<LobError> for Error {
    fn from(err: LobError) -> Self {
        Error {
            kind: ErrorKind::Lob(err),
        }
    }
}
