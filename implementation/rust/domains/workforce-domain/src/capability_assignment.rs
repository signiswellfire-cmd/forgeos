//! CapabilityAssignment entity stub.
//!
//! This is a minimal stub implementation for the foundation milestone.

use crate::{CapabilityReference, ProfessionalId, WorkforceError};

/// CapabilityAssignment entity stub.
///
/// Represents the assignment of capabilities to professionals or teams
/// (TDS-0002, TDS-0003).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityAssignment {
    professional_id: ProfessionalId,
    capability_reference: CapabilityReference,
}

impl CapabilityAssignment {
    /// Creates a new CapabilityAssignment entity stub.
    pub fn new(
        professional_id: ProfessionalId,
        capability_reference: CapabilityReference,
    ) -> Self {
        Self {
            professional_id,
            capability_reference,
        }
    }

    pub fn professional_id(&self) -> ProfessionalId {
        self.professional_id
    }

    pub fn capability_reference(&self) -> &CapabilityReference {
        &self.capability_reference
    }
}