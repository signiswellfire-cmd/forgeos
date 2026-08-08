//! Workforce value objects.
//!
//! Value objects are immutable types that describe aspects of the Workforce
//! aggregate. They have no identity and are defined by their values.

use std::fmt;

use crate::errors::WorkforceError;

// ProfessionalId - Unique identifier for professionals

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProfessionalId(uuid::Uuid);

impl ProfessionalId {
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

impl Default for ProfessionalId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ProfessionalId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// TeamId - Unique identifier for teams

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TeamId(uuid::Uuid);

impl TeamId {
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

impl Default for TeamId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TeamId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// CompetencyLevel - Competency proficiency level

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompetencyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

impl fmt::Display for CompetencyLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level_str = match self {
            Self::Beginner => "Beginner",
            Self::Intermediate => "Intermediate",
            Self::Advanced => "Advanced",
            Self::Expert => "Expert",
        };
        write!(f, "{}", level_str)
    }
}

// SkillIdentifier - Unique identifier for skills

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SkillIdentifier(uuid::Uuid);

impl SkillIdentifier {
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

impl Default for SkillIdentifier {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SkillIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// WorkforceStatus - Workforce status

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkforceStatus {
    Active,
    Inactive,
    OnLeave,
    Retired,
}

impl fmt::Display for WorkforceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            Self::Active => "Active",
            Self::Inactive => "Inactive",
            Self::OnLeave => "OnLeave",
            Self::Retired => "Retired",
        };
        write!(f, "{}", status_str)
    }
}

// CapabilityReference - Reference to organizational capability

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapabilityReference(String);

impl CapabilityReference {
    pub fn new(reference: impl Into<String>) -> Result<Self, WorkforceError> {
        let ref_str = reference.into();
        if ref_str.is_empty() {
            return Err(WorkforceError::EmptyCapabilityReference);
        }
        if ref_str.len() > 200 {
            return Err(WorkforceError::CapabilityReferenceTooLong(ref_str.len()));
        }
        Ok(Self(ref_str))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CapabilityReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}