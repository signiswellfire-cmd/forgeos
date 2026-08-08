//! Team entity stub.
//!
//! This is a minimal stub implementation for the foundation milestone.

use crate::{TeamId, WorkforceError};

/// Team entity stub.
///
/// Represents a temporary collection of Professionals assembled around Missions
/// (RFC-0015, TDS-0002).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Team {
    id: TeamId,
    name: String,
}

impl Team {
    /// Creates a new Team entity stub.
    pub fn new(id: TeamId, name: impl Into<String>) -> Result<Self, WorkforceError> {
        let name: String = name.into();
        if name.is_empty() {
            return Err(WorkforceError::InvalidTeamName(
                "name cannot be empty".to_string(),
            ));
        }

        Ok(Self { id, name })
    }

    pub fn id(&self) -> TeamId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}