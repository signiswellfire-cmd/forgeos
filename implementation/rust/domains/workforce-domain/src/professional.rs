//! Professional entity stub.
//!
//! This is a minimal stub implementation for the foundation milestone.
//! Full implementation will be completed in a future milestone.

use crate::{ProfessionalId, WorkforceError, WorkforceStatus};

/// Professional entity stub.
///
/// Represents a permanent organizational responsibility that may be fulfilled
/// by a human, local AI, cloud AI, or hybrid execution model (RFC-0015).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Professional {
    id: ProfessionalId,
    name: String,
    status: WorkforceStatus,
}

impl Professional {
    /// Creates a new Professional entity stub.
    pub fn new(id: ProfessionalId, name: impl Into<String>) -> Result<Self, WorkforceError> {
        let name: String = name.into();
        if name.is_empty() {
            return Err(WorkforceError::InvalidProfessionalName(
                "name cannot be empty".to_string(),
            ));
        }

        Ok(Self {
            id,
            name,
            status: WorkforceStatus::Active,
        })
    }

    pub fn id(&self) -> ProfessionalId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn status(&self) -> WorkforceStatus {
        self.status
    }
}