//! Manage Workforce Service (Milestone 4.0).
//!
//! This module implements the application service for managing Workforce
//! aggregates. It orchestrates the workforce management workflow without
//! containing business rules (TDS-0002, ISP-0001).

use forgeos_workforce_domain::{ProfessionalId, Workforce, WorkforceResult};

/// ManageWorkforceService - Application service for managing Workforce
///
/// This service orchestrates the management of a Workforce aggregate
/// by coordinating domain operations and infrastructure concerns.
pub struct ManageWorkforceService;

impl ManageWorkforceService {
    pub fn new() -> Self {
        Self
    }

    /// Execute the manage workforce workflow
    ///
    /// # Arguments
    /// * `id` - The professional identifier for this workforce
    ///
    /// # Returns
    /// * `Ok(Workforce)` - Successfully created
    /// * `Err(WorkforceError)` - Creation failed
    pub fn execute(&self, id: ProfessionalId) -> WorkforceResult<Workforce> {
        // Create the aggregate
        Workforce::new(id)
    }
}

impl Default for ManageWorkforceService {
    fn default() -> Self {
        Self::new()
    }
}