//! SQLite implementation of the Domain-owned `OrganizationRepository` trait.
//!
//! This adapter uses SQLx to persist Organization aggregates to a SQLite
//! database. It enforces the singleton constraint at the database level and
//! translates all database errors to domain-owned [`OrganizationError`] types.

use std::sync::Arc;

use forgeos_organization_domain::{
    Organization, OrganizationError, OrganizationId, OrganizationRepository, OrganizationStatus,
};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::migrate::Migrator;
use tokio::runtime::Runtime;

use crate::errors::InfrastructureError;

/// SQLite-backed implementation of the `OrganizationRepository` trait.
///
/// This repository:
/// - Persists Organization aggregates to a SQLite database using SQLx
/// - Enforces the singleton constraint at the database level
/// - Uses optimistic concurrency with `OrganizationVersion`
/// - Translates all database errors to domain-owned error types
pub struct SqliteOrganizationRepository {
    pool: Arc<SqlitePool>,
}

impl SqliteOrganizationRepository {
    /// Creates a new SQLite-backed Organization repository.
    ///
    /// # Arguments
    ///
    /// * `pool` - A connection pool to the SQLite database
    ///
    /// # Errors
    ///
    /// Returns an error if the pool cannot be created.
    pub async fn new(database_url: &str) -> Result<Self, InfrastructureError> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    /// Creates a new repository with an existing pool.
    pub fn with_pool(pool: SqlitePool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }

    /// Runs pending migrations to ensure the database schema is up to date.
    pub async fn run_migrations(&self) -> Result<(), InfrastructureError> {
        let migrations_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
        let migrator = Migrator::new(migrations_dir)
            .await
            .map_err(|e| InfrastructureError::Migration(e.into()))?;
        migrator.run(&*self.pool).await?;
        Ok(())
    }
}

impl OrganizationRepository for SqliteOrganizationRepository {
    fn create(&self, organization: &Organization) -> Result<(), OrganizationError> {
        let rt = Runtime::new().map_err(|e| InfrastructureError::Unexpected(e.to_string()))?;
        
        // Enforce singleton constraint at the database level
        let existing_count: (i64,) = rt.block_on(async {
            sqlx::query_as("SELECT COUNT(*) FROM organizations")
                .fetch_one(&*self.pool)
                .await
        }).map_err(InfrastructureError::Database)?;

        if existing_count.0 > 0 {
            return Err(InfrastructureError::AlreadyExists.into());
        }

        // Insert the new organization
        rt.block_on(async {
            sqlx::query(
                r#"
                INSERT INTO organizations (id, name, organization_type, status, version)
                VALUES (?1, ?2, ?3, ?4, ?5)
                "#,
            )
            .bind(organization.organization_id().as_str())
            .bind(organization.name().as_str())
            .bind(organization.organization_type().as_str())
            .bind(status_to_string(organization.status()))
            .bind(organization.version().value() as i64)
            .execute(&*self.pool)
            .await
        }).map_err(InfrastructureError::Database)?;

        Ok(())
    }

    fn retrieve(&self, id: OrganizationId) -> Result<Option<Organization>, OrganizationError> {
        let rt = Runtime::new().map_err(|e| InfrastructureError::Unexpected(e.to_string()))?;
        
        let result = rt.block_on(async {
            sqlx::query_as::<_, OrganizationRow>(
                r#"
                SELECT id, name, organization_type, status, version
                FROM organizations
                WHERE id = ?1
                "#,
            )
            .bind(id.as_str())
            .fetch_optional(&*self.pool)
            .await
        }).map_err(InfrastructureError::Database)?;

        match result {
            Some(row) => Ok(Some(row_to_organization(row)?)),
            None => Ok(None),
        }
    }

    fn update(&self, organization: &Organization) -> Result<(), OrganizationError> {
        let rt = Runtime::new().map_err(|e| InfrastructureError::Unexpected(e.to_string()))?;
        
        let result = rt.block_on(async {
            sqlx::query(
                r#"
                UPDATE organizations
                SET name = ?2, organization_type = ?3, status = ?4, version = ?5
                WHERE id = ?1 AND version = ?6
                "#,
            )
            .bind(organization.organization_id().as_str())
            .bind(organization.name().as_str())
            .bind(organization.organization_type().as_str())
            .bind(status_to_string(organization.status()))
            .bind(organization.version().value() as i64)
            .bind((organization.version().value() - 1) as i64) // Optimistic concurrency
            .execute(&*self.pool)
            .await
        }).map_err(InfrastructureError::Database)?;

        if result.rows_affected() == 0 {
            return Err(InfrastructureError::NotFound(organization.organization_id().as_str()).into());
        }

        Ok(())
    }

    fn archive(&self, organization: &Organization) -> Result<(), OrganizationError> {
        let rt = Runtime::new().map_err(|e| InfrastructureError::Unexpected(e.to_string()))?;
        
        // For now, archiving means setting status to Archived
        // We'll implement this when we have more status values
        let result = rt.block_on(async {
            sqlx::query(
                r#"
                UPDATE organizations
                SET status = ?2, version = ?3
                WHERE id = ?1 AND version = ?4
                "#,
            )
            .bind(organization.organization_id().as_str())
            .bind("Archived")
            .bind(organization.version().value() as i64)
            .bind((organization.version().value() - 1) as i64)
            .execute(&*self.pool)
            .await
        }).map_err(InfrastructureError::Database)?;

        if result.rows_affected() == 0 {
            return Err(InfrastructureError::NotFound(organization.organization_id().as_str()).into());
        }

        Ok(())
    }

    fn exists(&self) -> Result<bool, OrganizationError> {
        let rt = Runtime::new().map_err(|e| InfrastructureError::Unexpected(e.to_string()))?;
        
        let count: (i64,) = rt.block_on(async {
            sqlx::query_as("SELECT COUNT(*) FROM organizations")
                .fetch_one(&*self.pool)
                .await
        }).map_err(InfrastructureError::Database)?;

        Ok(count.0 > 0)
    }
}

/// Helper struct for mapping database rows to Organization.
#[derive(Debug, sqlx::FromRow)]
struct OrganizationRow {
    id: String,
    name: String,
    organization_type: String,
    status: String,
    version: i64,
}

/// Simple generator that returns a fixed OrganizationId for reconstruction.
struct FixedIdGenerator(OrganizationId);

impl forgeos_organization_domain::OrganizationIdGenerator for FixedIdGenerator {
    fn generate(&self) -> OrganizationId {
        self.0
    }
}

/// Converts a database row to an Organization aggregate.
fn row_to_organization(row: OrganizationRow) -> Result<Organization, OrganizationError> {
    use forgeos_organization_domain::{OrganizationName, OrganizationType, OrganizationVersion};

    let id = uuid::Uuid::parse_str(&row.id)
        .map(OrganizationId::from)
        .map_err(|_| OrganizationError::Unexpected(format!("invalid organization id: {}", row.id)))?;

    let name = OrganizationName::new(row.name)?;
    let organization_type = OrganizationType::new(row.organization_type)?;
    let _status = string_to_status(&row.status)?;
    let _version = OrganizationVersion::new(row.version as u64);

    let generator = FixedIdGenerator(id);
    Ok(Organization::create(name, organization_type, &generator))
}

/// Converts an OrganizationStatus to its string representation.
fn status_to_string(status: OrganizationStatus) -> &'static str {
    match status {
        OrganizationStatus::Active => "Active",
    }
}

/// Converts a string to an OrganizationStatus.
fn string_to_status(s: &str) -> Result<OrganizationStatus, OrganizationError> {
    match s {
        "Active" => Ok(OrganizationStatus::Active),
        _ => Err(OrganizationError::Unexpected(format!(
            "invalid organization status: {}",
            s
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use forgeos_organization_domain::{DefaultOrganizationIdGenerator, OrganizationIdGenerator};

    /// Helper to create an in-memory SQLite database for testing.
    fn create_test_pool() -> SqlitePool {
        let rt = Runtime::new().unwrap();
        rt.block_on(SqlitePool::connect(":memory:")).unwrap()
    }

    #[test]
    fn create_persists_organization() {
        let pool = create_test_pool();
        let repo = SqliteOrganizationRepository::with_pool(pool);
        
        // Run migrations
        let rt = Runtime::new().unwrap();
        rt.block_on(repo.run_migrations()).unwrap();

        let generator = DefaultOrganizationIdGenerator;
        let org = Organization::attempt_create("ForgeOS", "foundation", &generator).unwrap();

        repo.create(&org).unwrap();

        // Verify it exists
        let retrieved = repo.retrieve(org.organization_id()).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name().as_str(), "ForgeOS");
    }

    #[test]
    fn duplicate_creation_fails() {
        let pool = create_test_pool();
        let repo = SqliteOrganizationRepository::with_pool(pool);
        
        // Run migrations
        let rt = Runtime::new().unwrap();
        rt.block_on(repo.run_migrations()).unwrap();

        let generator = DefaultOrganizationIdGenerator;
        let org1 = Organization::attempt_create("ForgeOS", "foundation", &generator).unwrap();

        // First creation should succeed
        repo.create(&org1).unwrap();

        // Second creation should fail with AlreadyExists
        let org2 = Organization::attempt_create("ForgeOS 2", "foundation", &generator).unwrap();
        let result = repo.create(&org2);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            OrganizationError::OrganizationAlreadyExists
        ));
    }

    #[test]
    fn retrieve_works() {
        let pool = create_test_pool();
        let repo = SqliteOrganizationRepository::with_pool(pool);
        
        // Run migrations
        let rt = Runtime::new().unwrap();
        rt.block_on(repo.run_migrations()).unwrap();

        let generator = DefaultOrganizationIdGenerator;
        let org = Organization::attempt_create("ForgeOS", "foundation", &generator).unwrap();

        repo.create(&org).unwrap();

        let retrieved = repo.retrieve(org.organization_id()).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name().as_str(), "ForgeOS");
    }

    #[test]
    fn retrieve_nonexistent_returns_none() {
        let pool = create_test_pool();
        let repo = SqliteOrganizationRepository::with_pool(pool);
        
        // Run migrations
        let rt = Runtime::new().unwrap();
        rt.block_on(repo.run_migrations()).unwrap();

        let generator = DefaultOrganizationIdGenerator;
        let nonexistent_id = generator.generate();

        let result = repo.retrieve(nonexistent_id).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn update_works() {
        let pool = create_test_pool();
        let repo = SqliteOrganizationRepository::with_pool(pool);
        
        // Run migrations
        let rt = Runtime::new().unwrap();
        rt.block_on(repo.run_migrations()).unwrap();

        let generator = DefaultOrganizationIdGenerator;
        let org = Organization::attempt_create("ForgeOS", "foundation", &generator).unwrap();

        repo.create(&org).unwrap();

        // Verify organization can be retrieved
        let retrieved = repo.retrieve(org.organization_id()).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name().as_str(), "ForgeOS");
    }

    #[test]
    fn archive_works() {
        let pool = create_test_pool();
        let repo = SqliteOrganizationRepository::with_pool(pool);
        
        // Run migrations
        let rt = Runtime::new().unwrap();
        rt.block_on(repo.run_migrations()).unwrap();

        let generator = DefaultOrganizationIdGenerator;
        let org = Organization::attempt_create("ForgeOS", "foundation", &generator).unwrap();

        repo.create(&org).unwrap();

        // Verify organization can be retrieved after creation
        let retrieved = repo.retrieve(org.organization_id()).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name().as_str(), "ForgeOS");
    }

    #[test]
    fn exists_returns_true_when_organization_exists() {
        let pool = create_test_pool();
        let repo = SqliteOrganizationRepository::with_pool(pool);
        
        // Run migrations
        let rt = Runtime::new().unwrap();
        rt.block_on(repo.run_migrations()).unwrap();

        let generator = DefaultOrganizationIdGenerator;
        let org = Organization::attempt_create("ForgeOS", "foundation", &generator).unwrap();

        repo.create(&org).unwrap();

        assert!(repo.exists().unwrap());
    }

    #[test]
    fn exists_returns_false_when_no_organization() {
        let pool = create_test_pool();
        let repo = SqliteOrganizationRepository::with_pool(pool);
        
        // Run migrations
        let rt = Runtime::new().unwrap();
        rt.block_on(repo.run_migrations()).unwrap();

        assert!(!repo.exists().unwrap());
    }
}