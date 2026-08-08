//! Skill entity stub.
//!
//! This is a minimal stub implementation for the foundation milestone.

use crate::{SkillIdentifier, WorkforceError};

/// Skill entity stub.
///
/// Represents a specific professional capability (TDS-0002).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skill {
    id: SkillIdentifier,
    name: String,
    description: String,
}

impl Skill {
    /// Creates a new Skill entity stub.
    pub fn new(
        id: SkillIdentifier,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<Self, WorkforceError> {
        let name: String = name.into();
        if name.is_empty() {
            return Err(WorkforceError::InvalidSkillName(
                "name cannot be empty".to_string(),
            ));
        }

        Ok(Self {
            id,
            name,
            description: description.into(),
        })
    }

    pub fn id(&self) -> SkillIdentifier {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}