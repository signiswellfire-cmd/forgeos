//! The `Organization` aggregate root (TDS-0002).
//!
//! The Organization aggregate is the authoritative root of the Organization
//! bounded context and the sole mutation boundary for organizational state.
//! It owns organizational identity, profile, lifecycle, configuration, and
//! hierarchy metadata; this milestone implements only the approved creation
//! behavior.
//!
//! The aggregate exposes behavior rather than mutable internal state. It is
//! independent of persistence, Tauri, IPC, and other infrastructure.

use crate::errors::OrganizationError;
use crate::id_generation::OrganizationIdGenerator;
use crate::org_domain_event::OrganizationDomainEvent;
use crate::organization_created::OrganizationCreated;
use crate::value_objects::{
    OrganizationId, OrganizationName, OrganizationStatus, OrganizationType, OrganizationVersion,
};

/// The `Organization` aggregate root.
///
/// Owns identity, name, type, lifecycle status, and version. Created
/// organizations start with an aggregate-generated `OrganizationId`, the
/// supplied name and type, `Active` status, and version `1`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Organization {
    organization_id: OrganizationId,
    name: OrganizationName,
    organization_type: OrganizationType,
    status: OrganizationStatus,
    version: OrganizationVersion,
    recorded_events: Vec<OrganizationDomainEvent>,
}

impl Organization {
    /// Creates an Organization aggregate from validated domain values.
    ///
    /// The aggregate:
    ///
    /// 1. generates an immutable, globally unique `OrganizationId` through the
    ///    injected generator (TDR-0006);
    /// 2. establishes the supplied name and type;
    /// 3. initializes without capabilities, hierarchy members, or additional
    ///    configuration; and
    /// 4. records the `OrganizationCreated` domain event.
    ///
    /// The aggregate is externally observable as `Active` with version `1`,
    /// implementing rather than altering the lifecycle order defined by
    /// TDS-0002. Callers that hold raw input should use
    /// [`Organization::attempt_create`] to validate through the value-object
    /// constructors first.
    pub fn create(
        name: OrganizationName,
        organization_type: OrganizationType,
        generator: &dyn OrganizationIdGenerator,
    ) -> Self {
        let organization_id = generator.generate();
        let status = OrganizationStatus::Active;
        let version = OrganizationVersion::initial();

        let event = OrganizationCreated::new(
            organization_id,
            name.clone(),
            organization_type.clone(),
            status,
            version,
        );

        Self {
            organization_id,
            name,
            organization_type,
            status,
            version,
            recorded_events: vec![OrganizationDomainEvent::OrganizationCreated(event)],
        }
    }

    /// Creates an Organization aggregate from raw caller input.
    ///
    /// Validates `name` and `organization_type` through the approved value
    /// object constructors, then delegates to [`Organization::create`].
    ///
    /// # Errors
    ///
    /// Returns [`OrganizationError::Validation`] when either value is missing
    /// or contains no non-whitespace character. No aggregate is created.
    pub fn attempt_create(
        name: impl Into<String>,
        organization_type: impl Into<String>,
        generator: &dyn OrganizationIdGenerator,
    ) -> Result<Self, OrganizationError> {
        let name = OrganizationName::new(name)?;
        let organization_type = OrganizationType::new(organization_type)?;
        Ok(Self::create(name, organization_type, generator))
    }

    /// The immutable, aggregate-generated identity.
    pub fn organization_id(&self) -> OrganizationId {
        self.organization_id
    }

    /// The organization name established at creation.
    pub fn name(&self) -> &OrganizationName {
        &self.name
    }

    /// The organization type established at creation.
    pub fn organization_type(&self) -> &OrganizationType {
        &self.organization_type
    }

    /// The current lifecycle status.
    pub fn status(&self) -> OrganizationStatus {
        self.status
    }

    /// The current aggregate version used for optimistic concurrency.
    pub fn version(&self) -> OrganizationVersion {
        self.version
    }

    /// Collects the domain events recorded since the last drain.
    ///
    /// The aggregate records completed business facts (currently
    /// `OrganizationCreated`) without dispatching them. Dispatch after
    /// successful transaction commit is governed by ISP-0005 and ISP-0006;
    /// no consumer or dispatcher exists in this milestone.
    pub fn take_events(&mut self) -> Vec<OrganizationDomainEvent> {
        core::mem::take(&mut self.recorded_events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::id_generation::DefaultOrganizationIdGenerator;

    /// Deterministic generator for tests (ISP-0009).
    ///
    /// Produces fixed, predictable identities so tests never exercise random
    /// UUID generation directly.
    #[derive(Debug, Clone, Copy)]
    struct FixedGenerator(uuid::Uuid);

    impl OrganizationIdGenerator for FixedGenerator {
        fn generate(&self) -> OrganizationId {
            OrganizationId::from(self.0)
        }
    }

    fn fixed_generator() -> FixedGenerator {
        FixedGenerator(uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap())
    }

    #[test]
    fn valid_creation_produces_active_organization_with_version_one() {
        let generator = fixed_generator();
        let org = Organization::attempt_create("ForgeOS", "foundation", &generator).unwrap();

        assert_eq!(
            org.organization_id().as_str(),
            "550e8400-e29b-41d4-a716-446655440000"
        );
        assert_eq!(org.name().as_str(), "ForgeOS");
        assert_eq!(org.organization_type().as_str(), "foundation");
        assert_eq!(org.status(), OrganizationStatus::Active);
        assert_eq!(org.version(), OrganizationVersion::initial());
        assert_eq!(org.version().value(), 1);
    }

    #[test]
    fn invalid_organization_name_is_rejected() {
        let generator = fixed_generator();
        let err = Organization::attempt_create("   ", "foundation", &generator).unwrap_err();
        assert_eq!(
            err,
            OrganizationError::Validation(crate::errors::OrganizationField::Name)
        );
    }

    #[test]
    fn invalid_organization_type_is_rejected() {
        let generator = fixed_generator();
        let err = Organization::attempt_create("ForgeOS", "", &generator).unwrap_err();
        assert_eq!(
            err,
            OrganizationError::Validation(crate::errors::OrganizationField::OrganizationType)
        );
    }

    #[test]
    fn org_created_event_is_recorded_with_creation_facts() {
        let generator = fixed_generator();
        let mut org =
            Organization::attempt_create("ForgeOS", "foundation", &generator).unwrap();

        let events = org.take_events();
        assert_eq!(events.len(), 1);

        let OrganizationDomainEvent::OrganizationCreated(event) = &events[0];
        assert_eq!(
            event.organization_id().as_str(),
            "550e8400-e29b-41d4-a716-446655440000"
        );
        assert_eq!(event.name().as_str(), "ForgeOS");
        assert_eq!(event.organization_type().as_str(), "foundation");
        assert_eq!(event.status(), OrganizationStatus::Active);
        assert_eq!(event.version().value(), 1);
    }

    #[test]
    fn events_drain_and_do_not_duplicate() {
        let generator = fixed_generator();
        let mut org =
            Organization::attempt_create("ForgeOS", "foundation", &generator).unwrap();

        let first_drain = org.take_events();
        let second_drain = org.take_events();

        assert_eq!(first_drain.len(), 1);
        assert!(second_drain.is_empty());
    }

    #[test]
    fn deterministic_organization_id_generation_uses_injected_generator() {
        let generator = fixed_generator();
        let org = Organization::create(
            OrganizationName::new("ForgeOS").unwrap(),
            OrganizationType::new("foundation").unwrap(),
            &generator,
        );

        // The injected generator's identity is used verbatim.
        assert_eq!(
            org.organization_id().as_str(),
            "550e8400-e29b-41d4-a716-446655440000"
        );
    }

    #[test]
    fn default_generator_creates_distinct_organizations() {
        let generator = DefaultOrganizationIdGenerator;
        let first =
            Organization::attempt_create("ForgeOS", "foundation", &generator).unwrap();
        let second =
            Organization::attempt_create("ForgeOS", "foundation", &generator).unwrap();

        assert_ne!(first.organization_id(), second.organization_id());
    }

    #[test]
    fn create_preserves_supplied_name_and_type_exactly() {
        let generator = fixed_generator();
        let org = Organization::attempt_create("  ForgeOS  ", " custom ", &generator).unwrap();
        assert_eq!(org.name().as_str(), "  ForgeOS  ");
        assert_eq!(org.organization_type().as_str(), " custom ");
    }
}