use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Network(String),
    Config(String),
    Provider(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Network(msg) => write!(f, "network error: {msg}"),
            AppError::Config(msg) => write!(f, "config error: {msg}"),
            AppError::Provider(msg) => write!(f, "provider error: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        let err = AppError::Network("timeout".into());
        assert_eq!(err.to_string(), "network error: timeout");
    }
}
