//! GovernanceRule entity stub.
//!
//! This module provides a minimal GovernanceRule entity implementation.
//! Full implementation will be completed in future milestones.

use crate::{AuthorityLevel, GovernanceScope};

/// GovernanceRule - Represents a governance rule
#[derive(Debug, Clone)]
pub struct GovernanceRule {
    id: String,
    name: String,
    description: String,
    scope: GovernanceScope,
    enforcement_level: AuthorityLevel,
}

impl GovernanceRule {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        scope: GovernanceScope,
        enforcement_level: AuthorityLevel,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            scope,
            enforcement_level,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}