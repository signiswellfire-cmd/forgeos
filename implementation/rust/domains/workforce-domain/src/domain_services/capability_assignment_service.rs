//! CapabilityAssignmentService stub.
//!
//! This is a minimal stub implementation for the foundation milestone.

use crate::{CapabilityAssignment, CapabilityReference, ProfessionalId};

/// CapabilityAssignmentService stub.
///
/// Assigns capabilities to professionals or teams (TDS-0002).
#[derive(Debug, Clone, Default)]
pub struct CapabilityAssignmentService;

impl CapabilityAssignmentService {
    /// Creates a new CapabilityAssignmentService stub.
    pub fn new() -> Self {
        Self
    }

    /// Assigns a capability to a professional (stub implementation).
    pub fn assign_capability(
        &self,
        professional_id: ProfessionalId,
        capability_reference: CapabilityReference,
    ) -> CapabilityAssignment {
        // Stub implementation - returns a placeholder assignment
        CapabilityAssignment::new(professional_id, capability_reference)
    }
}