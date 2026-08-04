//! The `CreateOrganization` application service.
//!
//! Application Services coordinate use case execution without implementing
//! business rules. They orchestrate domain operations, manage transaction
//! boundaries, and coordinate with infrastructure concerns.
//!
//! This service demonstrates the canonical ForgeOS workflow orchestration
//! pattern (ISP-0001; TDS-0004):
//! 1. Validate command input
//! 2. Create the Organization aggregate
//! 3. Persist the aggregate (transaction begin)
//! 4. Collect domain events via `take_events()`
//! 5. Commit transaction
//! 6. Publish events after successful commit (ISP-0005; ISP-0006)
//! 7. Return the created Organization's identity

use crate::command::CreateOrganizationCommand;
use crate::errors::{CreateOrganizationError, OrganizationField};
use forgeos_organization_domain::EventPublisher;
use forgeos_organization_domain::OrganizationRepository;
use forgeos_organization_domain::{
    Organization, OrganizationIdGenerator, OrganizationName, OrganizationType,
};

/// Application service that coordinates the Create Organization use case.
///
/// This service orchestrates the creation of a new Organization by:
/// 1. Validating the command input
/// 2. Creating the Organization aggregate through the domain
/// 3. Persisting the aggregate through the repository
/// 4. Collecting domain events via `take_events()`
/// 5. Committing the transaction
/// 6. Publishing events after successful commit (ISP-0005; ISP-0006)
/// 7. Returning the created Organization's identity
///
/// Business rules remain in the Domain Layer. This service coordinates
/// only. Event publication occurs only after successful transaction commit,
/// ensuring that no events are published for rolled-back operations.
pub struct CreateOrganization<'a, R: OrganizationRepository> {
    repository: &'a R,
}

impl<'a, R: OrganizationRepository> CreateOrganization<'a, R> {
    /// Creates a new CreateOrganization application service.
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    /// Executes the Create Organization use case with event orchestration.
    ///
    /// # Arguments
    ///
    /// * `command` - The validated command containing organization name and type
    /// * `generator` - The identity generator for creating the Organization ID
    /// * `event_publisher` - The event publisher for dispatching domain events after commit
    ///
    /// # Returns
    ///
    /// The `OrganizationId` of the newly created Organization, or an application error.
    ///
    /// # Errors
    ///
    /// Returns `CreateOrganizationError::Validation` when the command contains
    /// invalid input (empty or whitespace-only name or type).
    ///
    /// Returns `CreateOrganizationError::OrganizationAlreadyExists` when an
    /// Organization already exists for this ForgeOS instance.
    ///
    /// Returns `CreateOrganizationError::Unexpected` for infrastructure or
    /// other unexpected failures.
    ///
    /// # Event Publication
    ///
    /// Domain events are published only after successful transaction commit
    /// (ISP-0005; ISP-0006). If the repository operation fails, no events
    /// are published. Event publication failures do not rollback committed
    /// business state.
    pub fn execute(
        &self,
        command: CreateOrganizationCommand,
        generator: &dyn OrganizationIdGenerator,
        event_publisher: &mut dyn EventPublisher,
    ) -> Result<forgeos_organization_domain::OrganizationId, CreateOrganizationError> {
        // Step 1: Validate command input through domain value objects
        let name = OrganizationName::new(command.name).map_err(|_| {
            CreateOrganizationError::Validation(OrganizationField::Name)
        })?;

        let organization_type = OrganizationType::new(command.organization_type).map_err(|_| {
            CreateOrganizationError::Validation(OrganizationField::OrganizationType)
        })?;

        // Step 2: Create the Organization aggregate through the domain
        let mut organization = Organization::create(name, organization_type, generator);

        // Step 3: Persist the aggregate through the repository
        // The repository enforces the singleton constraint (OrganizationAlreadyExists)
        // This represents the transaction boundary (ISP-0006)
        self.repository
            .create(&organization)
            .map_err(CreateOrganizationError::from)?;

        // Step 4: Collect domain events after successful persistence (ISP-0005)
        let events = organization.take_events();

        // Step 5: Publish events only after successful commit (ISP-0005; ISP-0006)
        // Event publication failures do not rollback committed business state
        if let Err(e) = event_publisher.publish_all(&events) {
            // Log the error but do not rollback the committed business state
            // In a production system, this would use a proper logging framework
            eprintln!("Warning: Failed to publish domain events: {}", e);
        }

        // Step 6: Return the created Organization's identity
        Ok(organization.organization_id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use forgeos_organization_domain::{DefaultOrganizationIdGenerator, OrganizationError, OrganizationIdGenerator};

    // Mock repository for testing
    #[derive(Debug, Clone, Default)]
    struct MockOrganizationRepository {
        create_called: bool,
        create_should_fail: bool,
        stored_organization: Option<Organization>,
    }

    impl OrganizationRepository for MockOrganizationRepository {
        fn create(&self, organization: &Organization) -> Result<(), OrganizationError> {
            if self.create_should_fail {
                return Err(OrganizationError::Unexpected(
                    "Mock repository failure".to_string(),
                ));
            }
            // In a real mock, we'd store the organization, but for this test
            // we just track that create was called
            Ok(())
        }

        fn retrieve(
            &self,
            _id: forgeos_organization_domain::OrganizationId,
        ) -> Result<Option<Organization>, OrganizationError> {
            unimplemented!("Not needed for create organization tests")
        }

        fn update(&self, _organization: &Organization) -> Result<(), OrganizationError> {
            unimplemented!("Not needed for create organization tests")
        }

        fn archive(&self, _organization: &Organization) -> Result<(), OrganizationError> {
            unimplemented!("Not needed for create organization tests")
        }

        fn exists(&self) -> Result<bool, OrganizationError> {
            unimplemented!("Not needed for create organization tests")
        }
    }

    // Deterministic generator for tests
    #[derive(Debug, Clone, Copy)]
    struct FixedGenerator(uuid::Uuid);

    impl OrganizationIdGenerator for FixedGenerator {
        fn generate(&self) -> forgeos_organization_domain::OrganizationId {
            forgeos_organization_domain::OrganizationId::from(self.0)
        }
    }

    fn fixed_generator() -> FixedGenerator {
        FixedGenerator(uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap())
    }

    #[test]
    fn execute_creates_organization_with_valid_input() {
        let repository = MockOrganizationRepository::default();
        let service = CreateOrganization::new(&repository);
        let command = CreateOrganizationCommand::new("ForgeOS", "foundation");
        let mut event_publisher = forgeos_organization_infrastructure::InMemoryEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut event_publisher);

        assert!(result.is_ok());
        let org_id = result.unwrap();
        assert_eq!(org_id.as_str(), "550e8400-e29b-41d4-a716-446655440000");
    }

    #[test]
    fn execute_returns_validation_error_for_empty_name() {
        let repository = MockOrganizationRepository::default();
        let service = CreateOrganization::new(&repository);
        let command = CreateOrganizationCommand::new("", "foundation");
        let mut event_publisher = forgeos_organization_infrastructure::InMemoryEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut event_publisher);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            CreateOrganizationError::Validation(OrganizationField::Name)
        );
    }

    #[test]
    fn execute_returns_validation_error_for_whitespace_name() {
        let repository = MockOrganizationRepository::default();
        let service = CreateOrganization::new(&repository);
        let command = CreateOrganizationCommand::new("   ", "foundation");
        let mut event_publisher = forgeos_organization_infrastructure::InMemoryEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut event_publisher);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            CreateOrganizationError::Validation(OrganizationField::Name)
        );
    }

    #[test]
    fn execute_returns_validation_error_for_empty_type() {
        let repository = MockOrganizationRepository::default();
        let service = CreateOrganization::new(&repository);
        let command = CreateOrganizationCommand::new("ForgeOS", "");
        let mut event_publisher = forgeos_organization_infrastructure::InMemoryEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut event_publisher);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            CreateOrganizationError::Validation(OrganizationField::OrganizationType)
        );
    }

    #[test]
    fn execute_returns_validation_error_for_whitespace_type() {
        let repository = MockOrganizationRepository::default();
        let service = CreateOrganization::new(&repository);
        let command = CreateOrganizationCommand::new("ForgeOS", "  ");
        let mut event_publisher = forgeos_organization_infrastructure::InMemoryEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut event_publisher);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            CreateOrganizationError::Validation(OrganizationField::OrganizationType)
        );
    }

    #[test]
    fn execute_propagates_repository_unexpected_error() {
        let mut repository = MockOrganizationRepository::default();
        repository.create_should_fail = true;
        let service = CreateOrganization::new(&repository);
        let command = CreateOrganizationCommand::new("ForgeOS", "foundation");
        let mut event_publisher = forgeos_organization_infrastructure::InMemoryEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut event_publisher);

        assert!(result.is_err());
        match result.unwrap_err() {
            CreateOrganizationError::Unexpected(message) => {
                assert_eq!(message, "Mock repository failure");
            }
            _ => panic!("Expected Unexpected error"),
        }
    }

    #[test]
    fn execute_uses_generator_for_organization_id() {
        let repository = MockOrganizationRepository::default();
        let service = CreateOrganization::new(&repository);
        let command = CreateOrganizationCommand::new("ForgeOS", "foundation");
        let mut event_publisher = forgeos_organization_infrastructure::InMemoryEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut event_publisher);

        assert!(result.is_ok());
        let org_id = result.unwrap();
        assert_eq!(org_id.as_str(), "550e8400-e29b-41d4-a716-446655440000");
    }

    #[test]
    fn execute_publishes_event_after_successful_commit() {
        let repository = MockOrganizationRepository::default();
        let service = CreateOrganization::new(&repository);
        let command = CreateOrganizationCommand::new("ForgeOS", "foundation");
        let mut event_publisher = forgeos_organization_infrastructure::InMemoryEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut event_publisher);

        assert!(result.is_ok());
        let events = event_publisher.drain_events();
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], forgeos_organization_domain::OrganizationDomainEvent::OrganizationCreated(_)));
    }

    #[test]
    fn execute_does_not_publish_events_when_repository_fails() {
        let mut repository = MockOrganizationRepository::default();
        repository.create_should_fail = true;
        let service = CreateOrganization::new(&repository);
        let command = CreateOrganizationCommand::new("ForgeOS", "foundation");
        let mut event_publisher = forgeos_organization_infrastructure::InMemoryEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut event_publisher);

        assert!(result.is_err());
        assert!(event_publisher.is_empty());
    }
}