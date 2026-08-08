//! Workforce Repository Implementation (Milestone 4.0).
//!
//! Stub implementation of WorkforceRepository.

use forgeos_workforce_domain::{
    ProfessionalId, Workforce, WorkforceError, WorkforceRepository, WorkforceResult,
};

/// In-memory stub implementation of WorkforceRepository.
///
/// This is a foundation milestone stub. A SQLx/SQLite implementation will
/// replace this in a future milestone per TDR-0003.
pub struct InMemoryWorkforceRepository {
    _storage: std::collections::HashMap<ProfessionalId, Workforce>,
}

impl InMemoryWorkforceRepository {
    pub fn new() -> Self {
        Self {
            _storage: std::collections::HashMap::new(),
        }
    }
}

impl WorkforceRepository for InMemoryWorkforceRepository {
    fn save(&mut self, _workforce: &Workforce) -> WorkforceResult<()> {
        Ok(())
    }

    fn find_by_id(&self, _id: ProfessionalId) -> WorkforceResult<Option<Workforce>> {
        Ok(None)
    }

    fn exists(&self, _id: ProfessionalId) -> WorkforceResult<bool> {
        Ok(false)
    }

    fn delete(&mut self, _id: ProfessionalId) -> WorkforceResult<()> {
        Ok(())
    }
}

impl Default for InMemoryWorkforceRepository {
    fn default() -> Self {
        Self::new()
    }
}