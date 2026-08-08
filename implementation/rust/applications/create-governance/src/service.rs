//! Create Governance Service (Milestone 3.0).
//!
//! This module implements the application service for creating Governance
//! aggregates. It orchestrates the creation workflow without containing
//! business rules (TDS-0002, ISP-0001).

use forgeos_governance_domain::{
    AuthorityLevel, Governance, GovernanceError, GovernanceResult, GovernanceScope,
};

/// CreateGovernanceService - Application service for creating Governance
///
/// This service orchestrates the creation of a Governance aggregate
/// by coordinating domain operations and infrastructure concerns.
pub struct CreateGovernanceService;

impl CreateGovernanceService {
    pub fn new() -> Self {
        Self
    }

    /// Execute the create governance workflow
    ///
    /// # Arguments
    /// * `authority_level` - The authority level for this governance
    /// * `scope` - The scope of governance authority
    ///
    /// # Returns
    /// * `Ok(Governance)` - Successfully created
    /// * `Err(GovernanceError)` - Creation failed
    pub fn execute(
        &self,
        authority_level: u8,
        scope: impl Into<String>,
    ) -> GovernanceResult<Governance> {
        // Validate authority level
        let authority_level = AuthorityLevel::from_u8(authority_level)?;

        // Validate scope
        let scope = GovernanceScope::new(scope)?;

        // Create the aggregate
        Governance::new(
            forgeos_governance_domain::DecisionId::new(),
            authority_level,
            scope,
        )
    }
}

impl Default for CreateGovernanceService {
    fn default() -> Self {
        Self::new()
    }
}