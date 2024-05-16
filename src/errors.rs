use cfg_if::cfg_if;
use http::status::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq, Serialize, Deserialize)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Missing or Invalid Frontmatter")]
    MissingOrInvalidFrontmatter,
    #[error("TomlError: {0}")]
    OctocrabError(String),
    #[error("TomlError: {0}")]
    TomlError(String),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::MissingOrInvalidFrontmatter => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::OctocrabError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::TomlError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
         impl From<toml::de::Error> for AppError {
            fn from(value: toml::de::Error) -> Self {
                Self::TomlError(value.to_string())
            }
        }
         impl From<octocrab::Error> for AppError {
            fn from(value: octocrab::Error) -> Self {
                Self::OctocrabError(value.to_string())
            }
        }
    }
}
