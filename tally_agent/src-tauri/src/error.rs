use thiserror::error;

#[error(transparent)]
pub struct TallyClientError(pub String);

#[error(transparent)]
pub struct AppError(pub String);

impl From<reqwest::Error> for TallyClientError {
    fn from(e: reqwest::Error) -> Self {
        TallyClientError(e.to_string())
    }
}

impl From<std::net::AddrParseError> for AppError {
    fn from(e: std::net::AddrParseError) -> Self {
        AppError(e.to_string())
    }
}
