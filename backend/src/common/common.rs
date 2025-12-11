// Error handling utils 
use chrono::{DateTime, Utc};
use axum::http::StatusCode;
use crate::errors::ServiceError;
use std::fmt::Debug;
use std::str::FromStr;
use serde::{
    Deserialize, Serialize,
    de::{DeserializeOwned, Deserializer},
};
use validator::Validate;

// Standard API Response Wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationMeta>,
    /// Request timestamp
    pub timestamp: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub current_page: u32,
    pub per_page: u32,
    pub total_items: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_page: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedData<T> {
    /// List of items for current page
    pub items: Vec<T>,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub error_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<FieldError>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PaginationFilter {
    /// Page number (1-indexed)
    #[validate(range(min = 1))]
    pub page: Option<u32>,
    #[validate(range(min = 1, max = 100))]
    pub per_page: Option<u32>,
}

/// Complete filter combining pagination and filtering options
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct FilterRequest<T>
where
    T: Debug + Clone + Serialize + DeserializeOwned + FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    /// Page number (1-indexed)
    #[validate(range(min = 1))]
    pub page: Option<u32>,

    /// Number of items per page
    #[validate(range(min = 1, max = 100))]
    pub per_page: Option<u32>,

    /// The comparison operator
    pub operator: Option<NumericOperator>,

    /// The value to compare against
    pub value: Option<i64>,

    /// Start date (inclusive)
    pub from: Option<DateTime<Utc>>,

    /// End date (inclusive)
    pub to: Option<DateTime<Utc>>,

    #[serde(default, deserialize_with = "deserialize_states")]
    pub states: Option<Vec<T>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum NumericOperator {
    /// Greater than or equal to
    Gte,
    /// Less than or equal to
    Lte,
    /// Equal to
    Eq,
    /// Greater than
    Gt,
    /// Less than
    Lt,
}


pub fn deserialize_states<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned + FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    use serde::de::Error;

    // First try to deserialize as an optional string
    let opt_string: Option<String> = Option::deserialize(deserializer)?;

    match opt_string {
        Some(s) if s.trim().is_empty() => Ok(None),
        Some(s) => {
            // Split by comma and parse each state
            let states = s
                .split(',')
                .map(|state| state.trim())
                .filter(|state| !state.is_empty())
                .map(|state| {
                    T::from_str(state)
                        .map_err(|e| Error::custom(format!("Invalid state '{state}': {e}")))
                })
                .collect::<Result<Vec<T>, _>>()?;

            if states.is_empty() {
                Ok(None)
            } else {
                Ok(Some(states))
            }
        }
        None => Ok(None),
    }
}

impl PaginationMeta {
    /// Create pagination metadata from page parameters and total count
    pub fn new(current_page: u32, per_page: u32, total_items: u64) -> Self {
        let total_pages = if total_items == 0 {
            1
        } else {
            ((total_items - 1) / per_page as u64 + 1) as u32
        };

        let has_next = current_page < total_pages;
        let has_prev = current_page > 1;

        Self {
            current_page,
            per_page,
            total_items,
            total_pages,
            has_next,
            has_prev,
            next_page: if has_next {
                Some(current_page + 1)
            } else {
                None
            },
            prev_page: if has_prev {
                Some(current_page - 1)
            } else {
                None
            },
        }
    }

    pub fn from_filter(filter: &PaginationFilter, total_items: u64) -> Self {
        Self::new(filter.page(), filter.per_page(), total_items)
    }
}

impl<T> PaginatedData<T> {
    /// Create a new paginated data wrapper
    pub fn new(items: Vec<T>, total: u64) -> Self {
        Self { items, total }
    }
}

impl<T> ApiResponse<T> {
    /// Create a successful response
    pub fn success(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: message.into(),
            error: None,
            pagination: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a successful paginated response
    pub fn paginated(data: T, pagination: PaginationMeta, message: impl Into<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: message.into(),
            error: None,
            pagination: Some(pagination),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a successful paginated response with default message
    pub fn ok_paginated(data: T, pagination: PaginationMeta) -> Self {
        Self::paginated(data, pagination, "Request successful")
    }

    /// Create an error response
    pub fn error(
        message: impl Into<String>,
        error_type: impl Into<String>,
        details: Option<Vec<FieldError>>,
    ) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            message: message.into(),
            error: Some(ErrorDetails {
                error_type: error_type.into(),
                details,
            }),
            pagination: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl PaginationFilter {
    /// Get page number with default
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1).max(1)
    }

    /// Get per_page with default
    pub fn per_page(&self) -> u32 {
        self.per_page.unwrap_or(20)
    }

    /// Calculate offset for database queries
    pub fn offset(&self) -> i64 {
        ((self.page() - 1) * self.per_page()) as i64
    }

    /// Get limit for database queries
    pub fn limit(&self) -> i64 {
        self.per_page() as i64
    }
}

impl Default for PaginationFilter {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(20),
        }
    }
}

pub fn service_error_to_http(error: ServiceError) -> (StatusCode, String) {
    let (status, error_type, message) = match error {
        ServiceError::Validation { message } => {
            (StatusCode::BAD_REQUEST, "validation_error", message)
        }
        ServiceError::NotFound { entity, identifier } => (
            StatusCode::NOT_FOUND,
            "not_found",
            format!("{entity} '{identifier}' not found"),
        ),
        ServiceError::AlreadyExists { entity, identifier } => (
            StatusCode::CONFLICT,
            "already_exists",
            format!("{entity} '{identifier}' already exists"),
        ),
        ServiceError::InvalidOperation { message } => {
            (StatusCode::BAD_REQUEST, "invalid_operation", message)
        }
        ServiceError::Database { source } => {
            tracing::error!("Database error: {}", source);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database_error",
                "Internal server error".to_string(),
            )
        }
        ServiceError::ExternalService { message } => {
            (StatusCode::BAD_GATEWAY, "external_service_error", message)
        }
        ServiceError::InternalError { message } => {
            tracing::error!("Internal error: {}", message);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "Internal server error".to_string(),
            )
        }
    };

    let error_response = ApiResponse::<()>::error(message, error_type, None);
    (status, serde_json::to_string(&error_response).unwrap())
}


/// Formats validator::ValidationErrors into field-specific error details
pub fn validation_errors_to_field_errors(errors: validator::ValidationErrors) -> Vec<FieldError> {
    errors
        .field_errors()
        .into_iter()
        .flat_map(|(field, errors)| {
            errors.iter().map(move |error| FieldError {
                field: field.to_string(),
                message: error
                    .message
                    .as_ref()
                    .unwrap_or(&"Invalid value".into())
                    .to_string(),
            })
        })
        .collect()
}

pub fn validation_error_response(errors: validator::ValidationErrors) -> (StatusCode, String) {
    let field_errors = validation_errors_to_field_errors(errors);
    let error_response =
        ApiResponse::<()>::error("Validation failed", "validation_error", Some(field_errors));
    (
        StatusCode::BAD_REQUEST,
        serde_json::to_string(&error_response).unwrap(),
    )
}

/// Apply pagination to a collection
pub fn apply_pagination<T>(items: Vec<T>, pagination: &PaginationFilter) -> Vec<T> {
    let offset = pagination.offset() as usize;
    let limit = pagination.limit() as usize;

    items.into_iter().skip(offset).take(limit).collect()
}

