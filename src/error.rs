// src/error.rs
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("GitLab API error: {0}")]
    GitLabApi(#[from] reqwest::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid color format: {0}")]
    InvalidColor(String),
}

pub type AppResult<T> = Result<T, AppError>;
