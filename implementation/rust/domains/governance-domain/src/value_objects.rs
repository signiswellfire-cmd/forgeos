//! Governance value objects.
//!
//! Value objects are immutable types that describe aspects of the Governance
//! aggregate. They have no identity and are defined by their values.

use std::fmt;

use crate::errors::GovernanceError;

// DecisionId - Unique identifier for decisions

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecisionId(uuid::Uuid);

impl DecisionId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn from_uuid(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }
}

impl Default for DecisionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DecisionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// PolicyId - Unique identifier for policies

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PolicyId(uuid::Uuid);

impl PolicyId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn from_uuid(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }
}

impl Default for PolicyId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PolicyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// AuthorityLevel - Decision authority level (1-5 per RFC-0007)

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthorityLevel {
    Level1Founder,
    Level2Executive,
    Level3Professional,
    Level4Team,
    Level5Mission,
}

impl AuthorityLevel {
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Level1Founder => 1,
            Self::Level2Executive => 2,
            Self::Level3Professional => 3,
            Self::Level4Team => 4,
            Self::Level5Mission => 5,
        }
    }

    pub fn from_u8(level: u8) -> Result<Self, GovernanceError> {
        match level {
            1 => Ok(Self::Level1Founder),
            2 => Ok(Self::Level2Executive),
            3 => Ok(Self::Level3Professional),
            4 => Ok(Self::Level4Team),
            5 => Ok(Self::Level5Mission),
            _ => Err(GovernanceError::InvalidAuthorityLevel(level)),
        }
    }
}

impl fmt::Display for AuthorityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level_str = match self {
            Self::Level1Founder => "Level 1 — Founder",
            Self::Level2Executive => "Level 2 — Executive",
            Self::Level3Professional => "Level 3 — Professional",
            Self::Level4Team => "Level 4 — Team",
            Self::Level5Mission => "Level 5 — Mission",
        };
        write!(f, "{}", level_str)
    }
}

// ApprovalStatus - Approval state

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApprovalStatus {
    Proposed,
    UnderReview,
    Approved,
    Rejected,
    Implemented,
    Archived,
}

impl fmt::Display for ApprovalStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            Self::Proposed => "Proposed",
            Self::UnderReview => "Under Review",
            Self::Approved => "Approved",
            Self::Rejected => "Rejected",
            Self::Implemented => "Implemented",
            Self::Archived => "Archived",
        };
        write!(f, "{}", status_str)
    }
}

// GovernanceScope - Scope of governance authority

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GovernanceScope(String);

impl GovernanceScope {
    pub fn new(scope: impl Into<String>) -> Result<Self, GovernanceError> {
        let scope_str = scope.into();
        if scope_str.is_empty() {
            return Err(GovernanceError::EmptyGovernanceScope);
        }
        if scope_str.len() > 200 {
            return Err(GovernanceError::GovernanceScopeTooLong(scope_str.len()));
        }
        Ok(Self(scope_str))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for GovernanceScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// StandardIdentifier - Unique identifier for standards

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StandardIdentifier(uuid::Uuid);

impl StandardIdentifier {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn from_uuid(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }
}

impl Default for StandardIdentifier {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for StandardIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// GovernanceError and GovernanceResult are defined in errors.rs
// This module only contains value objects
