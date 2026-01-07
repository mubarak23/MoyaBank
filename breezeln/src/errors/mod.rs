// ! Global application error types and handlers.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LightningError {
    #[error("Node connection error: {0}")]
    ConnectionError(String),
    #[error("Get info error: {0}")]
    GetInfoError(String),
    #[error("Error while retrieving payments: {0}")]
    PaymentError(String),
    #[error("Error while retrieving invoices: {0}")]
    InvoiceError(String),
    #[error("Config validation failed: {0}")]
    ValidationError(String),
    #[error("Get graph error: {0}")]
    GetGraphError(String),
    #[error("Streaming error: {0}")]
    StreamingError(String),
    #[error("Channel error: {0}")]
    ChannelError(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Network error: {0}")]
    /// Network error.
    NetworkError(String),
}

/// Generic service error that can be used across all entities
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("{entity} not found: {identifier}")]
    NotFound { entity: String, identifier: String },

    #[error("{entity} already exists: {identifier}")]
    AlreadyExists { entity: String, identifier: String },

    #[error("Invalid operation: {message}")]
    InvalidOperation { message: String },

    #[error("Database error: {source}")]
    Database {
        #[from]
        source: anyhow::Error,
    },
    #[error("External service error: {message}")]
    ExternalService { message: String },
    #[error("Internal error: {message}")]
    InternalError { message: String },
}

pub type ServiceResult<T> = Result<T, ServiceError>;

impl ServiceError {
    // Helper constructors for common patterns

    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }

    pub fn not_found(entity: impl Into<String>, identifier: impl Into<String>) -> Self {
        Self::NotFound {
            entity: entity.into(),
            identifier: identifier.into(),
        }
    }

    pub fn already_exists(entity: impl Into<String>, identifier: impl Into<String>) -> Self {
        Self::AlreadyExists {
            entity: entity.into(),
            identifier: identifier.into(),
        }
    }

    pub fn invalid_operation(message: impl Into<String>) -> Self {
        Self::InvalidOperation {
            message: message.into(),
        }
    }
}
