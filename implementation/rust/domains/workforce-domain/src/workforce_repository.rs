//! Workforce repository interface.
//!
//! Domain-owned repository contract for persisting and retrieving Workforce
//! aggregates. The implementation resides in the infrastructure layer (ISP-0004).

use crate::{ProfessionalId, Workforce};

/// Repository contract for Workforce aggregate persistence.
///
/// This trait defines the operations that any Workforce repository
/// implementation must provide. The interface is owned by the domain layer
/// to preserve dependency direction (ARCH-0003).
pub trait WorkforceRepository {
    /// Persists a Workforce aggregate.
    ///
    /// Returns an error if the aggregate already exists or if persistence fails.
    fn save(&mut self, workforce: &Workforce) -> Result<(), crate::WorkforceError>;

    /// Retrieves a Workforce aggregate by its professional identifier.
    ///
    /// Returns `None` if the aggregate does not exist.
    fn find_by_id(&self, id: ProfessionalId) -> Result<Option<Workforce>, crate::WorkforceError>;

    /// Checks whether a Workforce aggregate exists.
    fn exists(&self, id: ProfessionalId) -> Result<bool, crate::WorkforceError>;

    /// Archives (deletes) a Workforce aggregate.
    ///
    /// Returns an error if the aggregate does not exist or if deletion fails.
    fn delete(&mut self, id: ProfessionalId) -> Result<(), crate::WorkforceError>;
}