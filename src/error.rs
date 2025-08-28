//! Top-level error types

#[derive(Debug, thiserror::Error)]
#[error("An error occurred in the application")]
pub struct AppError;

//a sugggestion displayd to the user
pub struct Suggestion(pub &'static str);

#[allow(dead_code)]
pub struct ErrorCode(u16);