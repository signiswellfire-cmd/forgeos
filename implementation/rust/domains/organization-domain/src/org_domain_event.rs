//! Domain events published by the Organization bounded context.
//!
//! The Organization context publishes completed business facts. This
//! milestone publishes only `OrganizationCreated`; future events
//! (`OrganizationUpdated`, `OrganizationArchived`, etc.) require separate
//! authority and are intentionally not invented here.

use crate::organization_created::OrganizationCreated;

/// A domain event published by the Organization bounded context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrganizationDomainEvent {
    /// An Organization was successfully created.
    OrganizationCreated(OrganizationCreated),
}

impl From<OrganizationCreated> for OrganizationDomainEvent {
    fn from(value: OrganizationCreated) -> Self {
        Self::OrganizationCreated(value)
    }
}