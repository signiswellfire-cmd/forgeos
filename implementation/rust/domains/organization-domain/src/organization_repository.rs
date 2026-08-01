//! Domain-owned `OrganizationRepository` contract (TDS-0002).
//!
//! The Organization Domain owns this contract. Implementations belong to
//! Infrastructure. The contract supports create, retrieve, update, archive,
//! existence verification, and optimistic concurrency.
//!
//! Milestone 1.5 defines the trait only; no infrastructure implementation is
//! created. The contract exposes no persistence technology (no SQLx, database,
//! or serialization types).

use crate::errors::OrganizationError;
use crate::organization::Organization;
use crate::value_objects::OrganizationId;

/// Domain-owned repository contract for the `Organization` aggregate.
pub trait OrganizationRepository {
    /// Persists a new Organization aggregate.
    ///
    /// Per MILESTONE-001-DOMAIN-DECISIONS, the persistence implementation must
    /// also enforce the singleton constraint inside the same transaction so
    /// concurrent requests cannot create two Organizations.
    ///
    /// # Errors
    ///
    /// Returns [`OrganizationError::OrganizationAlreadyExists`] when an
    /// Organization already exists for the ForgeOS instance. The call site
    /// may translate infrastructure failures through the structured
    /// [`OrganizationError::Unexpected`] boundary; the repository trait itself
    /// uses only Domain-owned error types.
    fn create(&self, organization: &Organization) -> Result<(), OrganizationError>;

    /// Retrieves an Organization by its immutable identity.
    fn retrieve(&self, id: OrganizationId) -> Result<Option<Organization>, OrganizationError>;

    /// Persists changes to an existing Organization aggregate.
    ///
    /// The implementation applies optimistic concurrency using the
    /// `OrganizationVersion` value.
    fn update(&self, organization: &Organization) -> Result<(), OrganizationError>;

    /// Archives an Organization aggregate.
    fn archive(&self, organization: &Organization) -> Result<(), OrganizationError>;

    /// Verifies whether an Organization exists for the ForgeOS instance.
    fn exists(&self) -> Result<bool, OrganizationError>;
}