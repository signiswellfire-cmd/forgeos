//! In-memory event publisher implementation (ISP-0005; TDS-0004).
//!
//! This module provides the Infrastructure Layer's implementation of the
//! Domain-owned `EventPublisher` trait. The `InMemoryEventPublisher` stores
//! published events in memory for test verification and in-process dispatch.
//!
//! ## Architecture
//!
//! ```text
//! Application Layer (orchestrates publication)
//!        ↓
//! Domain Layer (EventPublisher trait)
//!        ↓
//! Infrastructure Layer (this module)
//! ```
//!
//! ## Ownership
//!
//! The `EventPublisher` trait is owned by the Organization Domain (TDS-0002).
//! This implementation is owned by the Infrastructure Domain (ARCH-0002).
//!
//! ## Scope
//!
//! This implementation is for in-process event dispatch only. No external
//! message broker, event bus, or messaging infrastructure is introduced
//! (MILESTONE-002.0 Out of Scope).

use forgeos_organization_domain::EventPublisher;
use forgeos_organization_domain::OrganizationDomainEvent;

/// In-memory event publisher for test verification and in-process dispatch (ISP-0005).
///
/// Stores published events in a `Vec` for later inspection. This implementation
/// is intended for testing and in-process event coordination. No external
/// messaging infrastructure is introduced.
///
/// # Architecture
///
/// The `InMemoryEventPublisher` implements the Domain-owned `EventPublisher` trait,
/// preserving the dependency direction: Application → Domain ← Infrastructure.
///
/// # Usage
///
/// ```rust
/// use forgeos_organization_domain::{EventPublisher, OrganizationDomainEvent, OrganizationCreated, OrganizationId, OrganizationName, OrganizationStatus, OrganizationType, OrganizationVersion};
/// use forgeos_organization_infrastructure::InMemoryEventPublisher;
/// use uuid::Uuid;
///
/// // Create a test event
/// let org_id = OrganizationId::from(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap());
/// let name = OrganizationName::new("TestOrg").unwrap();
/// let org_type = OrganizationType::new("test").unwrap();
/// let event = OrganizationDomainEvent::OrganizationCreated(
///     OrganizationCreated::new(org_id, name, org_type, OrganizationStatus::Active, OrganizationVersion::initial())
/// );
///
/// let mut publisher = InMemoryEventPublisher::new();
/// publisher.publish(&event).unwrap();
///
/// // Retrieve published events for verification
/// let events = publisher.drain_events();
/// assert_eq!(events.len(), 1);
/// ```
pub struct InMemoryEventPublisher {
    events: Vec<OrganizationDomainEvent>,
}

impl InMemoryEventPublisher {
    /// Creates a new in-memory event publisher with an empty event buffer.
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }

    /// Retrieves and clears all published events.
    ///
    /// This method is primarily intended for test verification (ISP-0009).
    /// After calling `drain_events`, the publisher's event buffer is empty.
    ///
    /// # Returns
    ///
    /// A vector of all events published since the last drain.
    pub fn drain_events(&mut self) -> Vec<OrganizationDomainEvent> {
        core::mem::take(&mut self.events)
    }

    /// Returns the number of events currently stored without clearing.
    ///
    /// This method is useful for test assertions without consuming the events.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Returns true if no events have been published.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

impl Default for InMemoryEventPublisher {
    fn default() -> Self {
        Self::new()
    }
}

impl EventPublisher for InMemoryEventPublisher {
    /// Publishes a single domain event by storing it in memory.
    ///
    /// # Errors
    ///
    /// Returns an error if the event cannot be stored. In this in-memory
    /// implementation, storage failures are not expected, but the signature
    /// preserves the contract defined by the Domain trait.
    fn publish(&mut self, event: &OrganizationDomainEvent) -> Result<(), String> {
        self.events.push(event.clone());
        Ok(())
    }

    /// Publishes multiple domain events by storing them in memory.
    ///
    /// # Errors
    ///
    /// Returns an error if any event cannot be stored. In this in-memory
    /// implementation, storage failures are not expected, but the signature
    /// preserves the contract defined by the Domain trait.
    fn publish_all(&mut self, events: &[OrganizationDomainEvent]) -> Result<(), String> {
        self.events.extend(events.iter().cloned());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use forgeos_organization_domain::OrganizationCreated;
    use forgeos_organization_domain::{OrganizationId, OrganizationName, OrganizationStatus, OrganizationType, OrganizationVersion};

    /// Creates a test OrganizationCreated event with deterministic values.
    fn test_organization_created_event() -> OrganizationDomainEvent {
        let org_id = OrganizationId::from(uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap());
        let name = OrganizationName::new("TestOrg").unwrap();
        let org_type = OrganizationType::new("test").unwrap();
        let status = OrganizationStatus::Active;
        let version = OrganizationVersion::initial();

        let event = OrganizationCreated::new(org_id, name, org_type, status, version);
        OrganizationDomainEvent::OrganizationCreated(event)
    }

    #[test]
    fn new_publisher_starts_empty() {
        let publisher = InMemoryEventPublisher::new();
        assert!(publisher.is_empty());
        assert_eq!(publisher.len(), 0);
    }

    #[test]
    fn default_publisher_starts_empty() {
        let publisher = InMemoryEventPublisher::default();
        assert!(publisher.is_empty());
        assert_eq!(publisher.len(), 0);
    }

    #[test]
    fn publish_stores_single_event() {
        let mut publisher = InMemoryEventPublisher::new();
        let event = test_organization_created_event();

        let result = publisher.publish(&event);

        assert!(result.is_ok());
        assert_eq!(publisher.len(), 1);
        assert!(!publisher.is_empty());
    }

    #[test]
    fn publish_all_stores_multiple_events() {
        let mut publisher = InMemoryEventPublisher::new();
        let event1 = test_organization_created_event();
        let event2 = test_organization_created_event();

        let result = publisher.publish_all(&[event1, event2]);

        assert!(result.is_ok());
        assert_eq!(publisher.len(), 2);
    }

    #[test]
    fn drain_events_retrieves_and_clears() {
        let mut publisher = InMemoryEventPublisher::new();
        let event = test_organization_created_event();
        publisher.publish(&event).unwrap();

        let drained = publisher.drain_events();

        assert_eq!(drained.len(), 1);
        assert!(publisher.is_empty());
        assert_eq!(publisher.len(), 0);
    }

    #[test]
    fn multiple_publishes_accumulate() {
        let mut publisher = InMemoryEventPublisher::new();
        let event = test_organization_created_event();

        publisher.publish(&event).unwrap();
        publisher.publish(&event).unwrap();

        assert_eq!(publisher.len(), 2);
    }

    #[test]
    fn drain_then_publish_works_correctly() {
        let mut publisher = InMemoryEventPublisher::new();
        let event1 = test_organization_created_event();
        let event2 = test_organization_created_event();

        publisher.publish(&event1).unwrap();
        let first_drain = publisher.drain_events();
        assert_eq!(first_drain.len(), 1);

        publisher.publish(&event2).unwrap();
        assert_eq!(publisher.len(), 1);

        let second_drain = publisher.drain_events();
        assert_eq!(second_drain.len(), 1);
    }
}