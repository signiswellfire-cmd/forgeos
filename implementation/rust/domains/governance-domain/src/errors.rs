//! Governance domain error types.
//!
//! This module defines the error types used throughout the Governance domain.
//! All errors implement the `thiserror::Error` trait for consistent error handling.

use thiserror::Error;

pub type GovernanceResult<T> = Result<T, GovernanceError>;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum GovernanceError {
    #[error("Invalid authority level: {0} (must be 1-5)")]
    InvalidAuthorityLevel(u8),

    #[error("Empty governance scope")]
    EmptyGovernanceScope,

    #[error("Governance scope too long: {0} characters (max 200)")]
    GovernanceScopeTooLong(usize),

    #[error("Decision not found: {0}")]
    DecisionNotFound(String),

    #[error("Policy not found: {0}")]
    PolicyNotFound(String),

    #[error("Standard not found: {0}")]
    StandardNotFound(String),

    #[error("Invalid decision state transition: {0} -> {1}")]
    InvalidStateTransition(String, String),

    #[error("Authority level insufficient: required {0}, got {1}")]
    InsufficientAuthority(u8, u8),
}