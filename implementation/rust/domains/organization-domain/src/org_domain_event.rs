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

/// Contract for publishing domain events after successful transaction commit (ISP-0005).
///
/// The Organization Domain owns this trait. Both Application and Infrastructure
/// layers depend on it. The Infrastructure layer provides the concrete implementation.
/// The Application layer orchestrates event publication after commit.
///
/// Events are published only after successful transaction commit per ISP-0005
/// and ISP-0006. Event publication failures do not rollback committed business state.
pub trait EventPublisher {
    /// Publishes a single domain event.
    ///
    /// # Arguments
    ///
    /// * `event` - The domain event to publish
    ///
    /// # Errors
    ///
    /// Returns an error if the event cannot be dispatched. Business state
    /// remains committed even if publication fails (ISP-0005; ISP-0006).
    fn publish(&mut self, event: &OrganizationDomainEvent) -> Result<(), String>;

    /// Publishes multiple domain events.
    ///
    /// # Arguments
    ///
    /// * `events` - The domain events to publish
    ///
    /// # Errors
    ///
    /// Returns an error if any event cannot be dispatched. Business state
    /// remains committed even if publication fails (ISP-0005; ISP-0006).
    fn publish_all(&mut self, events: &[OrganizationDomainEvent]) -> Result<(), String>;
}
