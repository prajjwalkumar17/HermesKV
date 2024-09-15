// #[serde(rename_all = "snake_case")]
// pub enum ErrorType {
//     InvalidRequestError,
//     ServerError,
//     ValidationError,
//     ProcessingError,
// }

use std::io;

// #[derive(Debug, thiserror::Error)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ApplicationError {
    // Display's impl can be overridden by the attribute error marco.
    // Don't use Debug here, Debug gives error stack in response.
    // #[error("Application configuration error")]
    ConfigurationError(String),

    // #[error("Invalid configuration value provided: {0}")]
    InvalidConfigurationValueError(String),

    // #[error("Metrics error: {0}")]
    MetricsError(String),

    // #[error("I/O: {0}")]
    #[serde(skip)]
    IoError(io::Error),

    // #[error("Error while constructing api client: {0}")]
    ApiClientError(String),
}

impl From<io::Error> for ApplicationError {
    fn from(err: io::Error) -> Self {
        ApplicationError::IoError(err)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ApiErrorResponse {
    // #[error(error_type = ErrorType::ServerNotAvailable, code = "HE_00", message = "Something went wrong")]
    InternelServerError,
}

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Response<R> {
    Json(R),
    StatusOk,
    TextPlain(String),
}
pub type ApplicationResult<T> = Result<T, ApplicationError>;
pub type ApplicationResponse<T> = ApplicationResult<Response<T>>;
// pub type CustomResult<T, E> = error_stack::Result<T, E>;
pub type RouterResponse<T> = Result<ApplicationResponse<T>, ApiErrorResponse>;
