//! Governance repository contract.
//!
//! This module defines the domain-owned repository interface for the Governance
//! aggregate. The repository provides persistence abstraction while maintaining
//! the domain layer's independence from infrastructure concerns.

use crate::{Governance, GovernanceResult};

/// GovernanceRepository - Domain-owned repository contract
///
/// This trait defines the operations available for persisting and retrieving
/// Governance aggregates. It is owned by the Governance domain and implemented
/// by the Infrastructure layer.
///
/// # Architecture Notes
/// - Owned by: Governance Domain (TDS-0002, ARCH-0002)
/// - Implemented by: Infrastructure Domain (ISP-0004)
/// - Dependency direction: Infrastructure → Domain (TDS-0002, ARCH-0003)
pub trait GovernanceRepository: Send + Sync {
    /// Save a Governance aggregate
    ///
    /// # Arguments
    /// * `governance` - The Governance aggregate to persist
    ///
    /// # Returns
    /// * `Ok(())` - Successfully persisted
    /// * `Err(GovernanceError)` - Persistence failed
    fn save(&mut self, governance: &Governance) -> GovernanceResult<()>;

    /// Find a Governance aggregate by its identifier
    ///
    /// # Arguments
    /// * `id` - The Governance identifier
    ///
    /// # Returns
    /// * `Ok(Some(Governance))` - Found
    /// * `Ok(None)` - Not found
    /// * `Err(GovernanceError)` - Query failed
    fn find_by_id(&self, id: crate::DecisionId) -> GovernanceResult<Option<Governance>>;

    /// Check if a Governance aggregate exists
    ///
    /// # Arguments
    /// * `id` - The Governance identifier
    ///
    /// # Returns
    /// * `Ok(true)` - Exists
    /// * `Ok(false)` - Does not exist
    /// * `Err(GovernanceError)` - Query failed
    fn exists(&self, id: crate::DecisionId) -> GovernanceResult<bool>;

    /// Delete a Governance aggregate (archive)
    ///
    /// # Arguments
    /// * `id` - The Governance identifier
    ///
    /// # Returns
    /// * `Ok(())` - Successfully deleted
    /// * `Err(GovernanceError)` - Deletion failed
    fn delete(&mut self, id: crate::DecisionId) -> GovernanceResult<()>;
}
