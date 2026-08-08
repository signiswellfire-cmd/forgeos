//! Workforce domain errors.
//!
//! Defines the error types for the Workforce bounded context.

use thiserror::Error;

/// Workforce domain error types.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum WorkforceError {
    /// Invalid professional name.
    #[error("invalid professional name: {0}")]
    InvalidProfessionalName(String),

    /// Invalid team name.
    #[error("invalid team name: {0}")]
    InvalidTeamName(String),

    /// Invalid competency level.
    #[error("invalid competency level")]
    InvalidCompetencyLevel,

    /// Invalid skill name.
    #[error("invalid skill name: {0}")]
    InvalidSkillName(String),

    /// Empty capability reference.
    #[error("empty capability reference")]
    EmptyCapabilityReference,

    /// Capability reference too long.
    #[error("capability reference too long: {0} characters (max 200)")]
    CapabilityReferenceTooLong(usize),

    /// Professional not found.
    #[error("professional not found: {0}")]
    ProfessionalNotFound(String),

    /// Team not found.
    #[error("team not found: {0}")]
    TeamNotFound(String),

    /// Competency not found.
    #[error("competency not found: {0}")]
    CompetencyNotFound(String),

    /// Skill not found.
    #[error("skill not found: {0}")]
    SkillNotFound(String),

    /// Capability assignment not found.
    #[error("capability assignment not found")]
    CapabilityAssignmentNotFound,

    /// Team membership not found.
    #[error("team membership not found")]
    TeamMembershipNotFound,

    /// Professional already exists.
    #[error("professional already exists: {0}")]
    ProfessionalAlreadyExists(String),

    /// Team already exists.
    #[error("team already exists: {0}")]
    TeamAlreadyExists(String),

    /// Invalid workforce status transition.
    #[error("invalid workforce status transition from {0} to {1}")]
    InvalidStatusTransition(String, String),

    /// Workforce operation failed.
    #[error("workforce operation failed: {0}")]
    OperationFailed(String),
}