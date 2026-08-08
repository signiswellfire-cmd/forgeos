//! Standard entity stub.
//!
//! This module provides a minimal Standard entity implementation.
//! Full implementation will be completed in future milestones.

use crate::{AuthorityLevel, StandardIdentifier};

/// Standard - Represents a governance standard
#[derive(Debug, Clone)]
pub struct Standard {
    id: StandardIdentifier,
    name: String,
    description: String,
    version: String,
    enforcement_level: AuthorityLevel,
}

impl Standard {
    pub fn new(
        id: StandardIdentifier,
        name: impl Into<String>,
        description: impl Into<String>,
        version: impl Into<String>,
        enforcement_level: AuthorityLevel,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
            version: version.into(),
            enforcement_level,
        }
    }

    pub fn id(&self) -> StandardIdentifier {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}