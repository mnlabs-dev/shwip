use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum ShwipError {
    Io(String),
    Permission(String),
    Timeout(String),
    Trash(String),
    Config(String),
}

impl fmt::Display for ShwipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(msg) => write!(f, "IO error: {msg}"),
            Self::Permission(msg) => write!(f, "Permission error: {msg}"),
            Self::Timeout(msg) => write!(f, "Timeout: {msg}"),
            Self::Trash(msg) => write!(f, "Trash error: {msg}"),
            Self::Config(msg) => write!(f, "Config error: {msg}"),
        }
    }
}

impl std::error::Error for ShwipError {}

impl From<std::io::Error> for ShwipError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::PermissionDenied => {
                Self::Permission(err.to_string())
            }
            _ => Self::Io(err.to_string()),
        }
    }
}

impl From<serde_json::Error> for ShwipError {
    fn from(err: serde_json::Error) -> Self {
        Self::Config(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_io_error_permission() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
        let err: ShwipError = io_err.into();
        assert!(matches!(err, ShwipError::Permission(_)));
    }

    #[test]
    fn test_from_io_error_other() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
        let err: ShwipError = io_err.into();
        assert!(matches!(err, ShwipError::Io(_)));
    }

    #[test]
    fn test_display() {
        let err = ShwipError::Trash("test".into());
        assert_eq!(format!("{err}"), "Trash error: test");
    }
}
