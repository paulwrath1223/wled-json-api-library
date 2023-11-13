use thiserror::Error;

#[derive(Error, Debug)]
pub enum WledJsonApiError {
    #[error("serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Error adding port to url, honestly no idea how this happens, should not be possible")]
    UnableToAddPortToURL,
    #[error("Server responded with HTTP code {0}")]
    HttpError(reqwest::StatusCode),
}
