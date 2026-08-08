//! Governance Repository Implementation (Milestone 3.0).
//!
//! Stub implementation of GovernanceRepository.

use forgeos_governance_domain::{Governance, GovernanceError, GovernanceResult, GovernanceRepository, DecisionId};

pub struct InMemoryGovernanceRepository {
    _storage: std::collections::HashMap<DecisionId, Governance>,
}

impl InMemoryGovernanceRepository {
    pub fn new() -> Self {
        Self { _storage: std::collections::HashMap::new() }
    }
}

impl GovernanceRepository for InMemoryGovernanceRepository {
    fn save(&mut self, _governance: &Governance) -> GovernanceResult<()> {
        Ok(())
    }

    fn find_by_id(&self, _id: DecisionId) -> GovernanceResult<Option<Governance>> {
        Ok(None)
    }

    fn exists(&self, _id: DecisionId) -> GovernanceResult<bool> {
        Ok(false)
    }

    fn delete(&mut self, _id: DecisionId) -> GovernanceResult<()> {
        Ok(())
    }
}

impl Default for InMemoryGovernanceRepository {
    fn default() -> Self {
        Self::new()
    }
}