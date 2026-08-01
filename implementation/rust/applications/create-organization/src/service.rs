//! The `CreateOrganization` application service.
//!
//! Application Services coordinate use case execution without implementing
//! business rules. They orchestrate domain operations, manage transaction
//! boundaries, and coordinate with infrastructure concerns.

use crate::command::CreateOrganizationCommand;
use crate::errors::{CreateOrganizationError, OrganizationField};
use crate::OrganizationRepository;
use forgeos_organization_domain::{
    Organization, OrganizationIdGenerator, OrganizationName, OrganizationType,
};

/// Application service that coordinates the Create Organization use case.
///
/// This service orchestrates the creation of a new Organization by:
/// 1. Validating the command input
/// 2. Creating the Organization aggregate through the domain
/// 3. Persisting the aggregate through the repository
/// 4. Returning the created Organization's identity
///
/// Business rules remain in the Domain Layer. This service coordinates
/// only.
pub struct CreateOrganization<'a, R: OrganizationRepository> {
    repository: &'a R,
}

impl<'a, R: OrganizationRepository> CreateOrganization<'a, R> {
    /// Creates a new CreateOrganization application service.
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    /// Executes the Create Organization use case.
    ///
    /// # Arguments
    ///
    /// * `command` - The validated command containing organization name and type
    /// * `generator` - The identity generator for creating the Organization ID
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
    pub fn execute(
        &self,
        command: CreateOrganizationCommand,
        generator: &dyn OrganizationIdGenerator,
    ) -> Result<forgeos_organization_domain::OrganizationId, CreateOrganizationError> {
        // Step 1: Validate command input through domain value objects
        let name = OrganizationName::new(command.name).map_err(|_| {
            CreateOrganizationError::Validation(OrganizationField::Name)
        })?;

        let organization_type = OrganizationType::new(command.organization_type).map_err(|_| {
            CreateOrganizationError::Validation(OrganizationField::OrganizationType)
        })?;

        // Step 2: Create the Organization aggregate through the domain
        let organization = Organization::create(name, organization_type, generator);

        // Step 3: Persist the aggregate through the repository
        // The repository enforces the singleton constraint (OrganizationAlreadyExists)
        self.repository
            .create(&organization)
            .map_err(CreateOrganizationError::from)?;

        // Step 4: Return the created Organization's identity
        Ok(organization.organization_id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use forgeos_organization_domain::id_generation::DefaultOrganizationIdGenerator;

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

        let result = service.execute(command, &fixed_generator());

        assert!(result.is_ok());
        let org_id = result.unwrap();
        assert_eq!(org_id.as_str(), "550e8400-e29b-41d4-a716-446655440000");
    }

    #[test]
    fn execute_returns_validation_error_for_empty_name() {
        let repository = MockOrganizationRepository::default();
        let service = CreateOrganization::new(&repository);
        let command = CreateOrganizationCommand::new("", "foundation");

        let result = service.execute(command, &fixed_generator());

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

        let result = service.execute(command, &fixed_generator());

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

        let result = service.execute(command, &fixed_generator());

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

        let result = service.execute(command, &fixed_generator());

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

        let result = service.execute(command, &fixed_generator());

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

        let result = service.execute(command, &fixed_generator());

        assert!(result.is_ok());
        let org_id = result.unwrap();
        assert_eq!(org_id.as_str(), "550e8400-e29b-41d4-a716-446655440000");
    }
}