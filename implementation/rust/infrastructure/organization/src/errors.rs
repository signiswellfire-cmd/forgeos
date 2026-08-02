//! Infrastructure-level error types for the Organization persistence adapter.
//!
//! These errors never leak outside the Infrastructure layer. They are translated
//! to domain-owned [`OrganizationError`] at the repository boundary.

use forgeos_organization_domain::OrganizationError;
use thiserror::Error;

/// Errors that can occur during Organization persistence operations.
///
/// All errors are translated to [`OrganizationError`] before leaving the
/// Infrastructure layer. No SQLx, database, or IO error types are exposed
/// to the Domain or Application layers.
#[derive(Debug, Error)]
pub enum InfrastructureError {
    /// A database-level error occurred.
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    /// A migration error occurred.
    #[error("migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    /// An organization already exists (singleton constraint violation).
    #[error("organization already exists")]
    AlreadyExists,

    /// An organization was not found.
    #[error("organization not found: {0}")]
    NotFound(String),

    /// An unexpected infrastructure error occurred.
    #[error("unexpected infrastructure error: {0}")]
    Unexpected(String),
}

impl From<InfrastructureError> for OrganizationError {
    fn from(error: InfrastructureError) -> Self {
        match error {
            InfrastructureError::AlreadyExists => Self::OrganizationAlreadyExists,
            InfrastructureError::NotFound(id) => {
                // This is an unexpected state - we tried to operate on an organization that doesn't exist
                Self::Unexpected(format!("organization not found: {}", id))
            }
            InfrastructureError::Database(_) | InfrastructureError::Migration(_) => {
                // Database and migration errors are infrastructure failures
                Self::Unexpected(error.to_string())
            }
            InfrastructureError::Unexpected(message) => Self::Unexpected(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn already_exists_maps_to_domain_error() {
        let infra_error = InfrastructureError::AlreadyExists;
        let domain_error = OrganizationError::from(infra_error);
        assert!(matches!(domain_error, OrganizationError::OrganizationAlreadyExists));
    }

    #[test]
    fn not_found_maps_to_unexpected() {
        let infra_error = InfrastructureError::NotFound("test-id".to_string());
        let domain_error = OrganizationError::from(infra_error);
        assert!(matches!(domain_error, OrganizationError::Unexpected(_)));
    }

    #[test]
    fn database_error_maps_to_unexpected() {
        let infra_error = InfrastructureError::Database(sqlx::Error::PoolTimedOut);
        let domain_error = OrganizationError::from(infra_error);
        assert!(matches!(domain_error, OrganizationError::Unexpected(_)));
    }

    #[test]
    fn unexpected_maps_to_unexpected() {
        let infra_error = InfrastructureError::Unexpected("test error".to_string());
        let domain_error = OrganizationError::from(infra_error);
        match domain_error {
            OrganizationError::Unexpected(msg) => assert_eq!(msg, "test error"),
            _ => panic!("expected Unexpected error"),
        }
    }
}