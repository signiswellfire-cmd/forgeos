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
use crate::transaction::{Transaction, TransactionError};
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

    /// Executes the Create Organization use case with explicit transaction coordination.
    ///
    /// # Arguments
    ///
    /// * `command` - The validated command containing organization name and type
    /// * `generator` - The identity generator for creating the Organization ID
    /// * `transaction` - The transaction coordinator for managing the transaction boundary
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
    /// # Transaction Coordination
    ///
    /// This method follows the canonical ForgeOS transaction lifecycle (ISP-0006; TDS-0004):
    /// 1. Begin transaction
    /// 2. Execute domain operations
    /// 3. Commit transaction on success OR rollback on failure
    /// 4. Publish events only after successful commit (ISP-0005)
    ///
    /// If the repository operation fails, the transaction is rolled back and no events
    /// are published. Event publication failures do not rollback committed business state.
    pub fn execute(
        &self,
        command: CreateOrganizationCommand,
        generator: &dyn OrganizationIdGenerator,
        transaction: &mut dyn Transaction,
        event_publisher: &mut dyn EventPublisher,
    ) -> Result<forgeos_organization_domain::OrganizationId, CreateOrganizationError> {
        // Step 1: Validate command input through domain value objects
        let name = OrganizationName::new(command.name).map_err(|_| {
            CreateOrganizationError::Validation(OrganizationField::Name)
        })?;

        let organization_type = OrganizationType::new(command.organization_type).map_err(|_| {
            CreateOrganizationError::Validation(OrganizationField::OrganizationType)
        })?;

        // Step 2: Begin transaction boundary (ISP-0006; TDS-0004)
        transaction
            .begin()
            .map_err(|e| CreateOrganizationError::from(TransactionErrorWrapper(e)))?;

        // Step 3: Create the Organization aggregate through the domain
        let mut organization = Organization::create(name, organization_type, generator);

        // Step 4: Persist the aggregate through the repository within the transaction
        // The repository enforces the singleton constraint (OrganizationAlreadyExists)
        self.repository
            .create(&organization)
            .map_err(|e| {
                // Rollback transaction on repository failure
                let _ = transaction.rollback();
                CreateOrganizationError::from(e)
            })?;

        // Step 5: Commit transaction after successful persistence (ISP-0006)
        transaction
            .commit()
            .map_err(|e| CreateOrganizationError::from(TransactionErrorWrapper(e)))?;

        // Step 6: Collect domain events after successful commit (ISP-0005)
        let events = organization.take_events();

        // Step 7: Publish events only after successful commit (ISP-0005; ISP-0006)
        // Event publication failures do not rollback committed business state
        if let Err(e) = event_publisher.publish_all(&events) {
            // Log the error but do not rollback the committed business state
            // In a production system, this would use a proper logging framework
            eprintln!("Warning: Failed to publish domain events: {}", e);
        }

        // Step 8: Return the created Organization's identity
        Ok(organization.organization_id())
    }
}

/// Wrapper to convert TransactionError to CreateOrganizationError.
///
/// This wrapper allows us to preserve the transaction error information
/// while conforming to the application's error type system.
#[derive(Debug)]
struct TransactionErrorWrapper(TransactionError);

impl From<TransactionErrorWrapper> for CreateOrganizationError {
    fn from(wrapper: TransactionErrorWrapper) -> Self {
        CreateOrganizationError::Unexpected(format!("Transaction error: {}", wrapper.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockTransaction;
    use forgeos_organization_domain::{DefaultOrganizationIdGenerator, OrganizationError, OrganizationIdGenerator};

    // Mock event publisher for testing (Application-owned, no Infrastructure dependency)
    #[derive(Debug, Clone, Default)]
    struct MockEventPublisher {
        events: Vec<forgeos_organization_domain::OrganizationDomainEvent>,
    }

    impl MockEventPublisher {
        fn new() -> Self {
            Self::default()
        }

        fn drain_events(&mut self) -> Vec<forgeos_organization_domain::OrganizationDomainEvent> {
            std::mem::take(&mut self.events)
        }

        fn is_empty(&self) -> bool {
            self.events.is_empty()
        }
    }

    impl forgeos_organization_domain::EventPublisher for MockEventPublisher {
        fn publish(&mut self, event: &forgeos_organization_domain::OrganizationDomainEvent) -> Result<(), String> {
            self.events.push(event.clone());
            Ok(())
        }

        fn publish_all(&mut self, events: &[forgeos_organization_domain::OrganizationDomainEvent]) -> Result<(), String> {
            for event in events {
                self.events.push(event.clone());
            }
            Ok(())
        }
    }

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
        let mut transaction = MockTransaction::new();
        let mut event_publisher = MockEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut transaction, &mut event_publisher);

        assert!(result.is_ok());
        let org_id = result.unwrap();
        assert_eq!(org_id.as_str(), "550e8400-e29b-41d4-a716-446655440000");
    }

    #[test]
    fn execute_returns_validation_error_for_empty_name() {
        let repository = MockOrganizationRepository::default();
        let service = CreateOrganization::new(&repository);
        let command = CreateOrganizationCommand::new("", "foundation");
        let mut transaction = MockTransaction::new();
        let mut event_publisher = MockEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut transaction, &mut event_publisher);

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
        let mut transaction = MockTransaction::new();
        let mut event_publisher = MockEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut transaction, &mut event_publisher);

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
        let mut transaction = MockTransaction::new();
        let mut event_publisher = MockEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut transaction, &mut event_publisher);

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
        let mut transaction = MockTransaction::new();
        let mut event_publisher = MockEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut transaction, &mut event_publisher);

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
        let mut transaction = MockTransaction::new();
        let mut event_publisher = MockEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut transaction, &mut event_publisher);

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
        let mut transaction = MockTransaction::new();
        let mut event_publisher = MockEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut transaction, &mut event_publisher);

        assert!(result.is_ok());
        let org_id = result.unwrap();
        assert_eq!(org_id.as_str(), "550e8400-e29b-41d4-a716-446655440000");
    }

    #[test]
    fn execute_publishes_event_after_successful_commit() {
        let repository = MockOrganizationRepository::default();
        let service = CreateOrganization::new(&repository);
        let command = CreateOrganizationCommand::new("ForgeOS", "foundation");
        let mut transaction = MockTransaction::new();
        let mut event_publisher = MockEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut transaction, &mut event_publisher);

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
        let mut transaction = MockTransaction::new();
        let mut event_publisher = MockEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut transaction, &mut event_publisher);

        assert!(result.is_err());
        assert!(event_publisher.is_empty());
    }

    #[test]
    fn execute_calls_transaction_begin_before_domain_operations() {
        let repository = MockOrganizationRepository::default();
        let service = CreateOrganization::new(&repository);
        let command = CreateOrganizationCommand::new("ForgeOS", "foundation");
        let mut transaction = MockTransaction::new();
        let mut event_publisher = MockEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut transaction, &mut event_publisher);

        assert!(result.is_ok());
        assert!(transaction.begin_was_called());
    }

    #[test]
    fn execute_calls_transaction_commit_after_success() {
        let repository = MockOrganizationRepository::default();
        let service = CreateOrganization::new(&repository);
        let command = CreateOrganizationCommand::new("ForgeOS", "foundation");
        let mut transaction = MockTransaction::new();
        let mut event_publisher = MockEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut transaction, &mut event_publisher);

        assert!(result.is_ok());
        assert!(transaction.commit_was_called());
        assert!(!transaction.rollback_was_called());
    }

    #[test]
    fn execute_calls_transaction_rollback_on_failure() {
        let mut repository = MockOrganizationRepository::default();
        repository.create_should_fail = true;
        let service = CreateOrganization::new(&repository);
        let command = CreateOrganizationCommand::new("ForgeOS", "foundation");
        let mut transaction = MockTransaction::new();
        let mut event_publisher = MockEventPublisher::new();

        let result = service.execute(command, &fixed_generator(), &mut transaction, &mut event_publisher);

        assert!(result.is_err());
        assert!(transaction.rollback_was_called());
        assert!(!transaction.commit_was_called());
    }
}
