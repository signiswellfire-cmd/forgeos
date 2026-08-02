//! Dependency composition root for the Organization Platform Layer (ISP-0007).
//!
//! The composition root wires the Create Organization vertical slice by
//! constructing the Infrastructure repository, running database migrations,
//! constructing the Domain-owned ID generator, and registering composed
//! dependencies with the Tauri runtime for command access.
//!
//! This is the Tauri-native realization of ISP-0007 constructor injection:
//! dependencies are explicitly constructed, declared, and registered with
//! Tauri's state management so that command functions can access them through
//! `tauri::State`. No hidden dependencies or service locator patterns are used.

use forgeos_organization_domain::DefaultOrganizationIdGenerator;
use forgeos_organization_infrastructure::errors::InfrastructureError;
use forgeos_organization_infrastructure::SqliteOrganizationRepository;

/// Composed dependencies for the Create Organization vertical slice (ISP-0007).
///
/// Holds the Infrastructure repository and Domain ID generator constructed
/// by the composition root. These dependencies are registered with the Tauri
/// runtime so that command functions can access them through `tauri::State`.
///
/// The `CreateOrganization` application service is constructed per-request
/// from the stored repository reference, as Tauri's state management requires
/// `'static` lifetimes and the service holds a borrowed reference. This is
/// the Tauri-native realization of ISP-0007 constructor injection, not a
/// separate state-management architecture.
pub struct CompositionRoot {
    repository: SqliteOrganizationRepository,
    generator: DefaultOrganizationIdGenerator,
}

impl CompositionRoot {
    /// Constructs the composition root with all dependencies wired (ISP-0007).
    ///
    /// This method:
    ///
    /// 1. Constructs the `SqliteOrganizationRepository` (Infrastructure)
    /// 2. Runs database migrations
    /// 3. Constructs the `DefaultOrganizationIdGenerator` (Domain)
    ///
    /// # Errors
    ///
    /// Returns `InfrastructureError` if the repository cannot be created or
    /// migrations fail.
    pub async fn new(database_url: &str) -> Result<Self, InfrastructureError> {
        let repository = SqliteOrganizationRepository::new(database_url).await?;
        repository.run_migrations().await?;
        let generator = DefaultOrganizationIdGenerator;
        Ok(Self {
            repository,
            generator,
        })
    }

    /// Returns a reference to the composed repository.
    pub fn repository(&self) -> &SqliteOrganizationRepository {
        &self.repository
    }

    /// Returns a reference to the composed ID generator.
    pub fn generator(&self) -> &DefaultOrganizationIdGenerator {
        &self.generator
    }

    /// Registers composed dependencies and the `createOrganization` command
    /// with the Tauri runtime (ISP-0007).
    ///
    /// This is the Tauri-native realization of dependency injection: composed
    /// dependencies are registered with Tauri's state management so that
    /// command functions can access them through `tauri::State`.
    ///
    /// The `CreateOrganization` application service is constructed per-request
    /// inside the command function from the registered repository, as the
    /// service holds a borrowed reference that cannot satisfy Tauri's
    /// `'static` state requirement.
    pub fn register<R: tauri::Runtime>(
        self,
        builder: tauri::Builder<R>,
    ) -> tauri::Builder<R> {
        builder
            .manage(self.repository)
            .manage(self.generator)
            .invoke_handler(tauri::generate_handler![
                crate::commands::createOrganization
            ])
    }
}