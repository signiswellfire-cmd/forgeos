//! WorkforcePlanningService stub.
//!
//! This is a minimal stub implementation for the foundation milestone.

use crate::{ProfessionalId, TeamId};

/// WorkforcePlanningService stub.
///
/// Plans workforce capacity (TDS-0002).
#[derive(Debug, Clone, Default)]
pub struct WorkforcePlanningService;

impl WorkforcePlanningService {
    /// Creates a new WorkforcePlanningService stub.
    pub fn new() -> Self {
        Self
    }

    /// Plans workforce capacity (stub implementation).
    pub fn plan_capacity(&self, _required_capabilities: Vec<String>) -> Vec<ProfessionalId> {
        // Stub implementation - returns empty list
        Vec::new()
    }
}