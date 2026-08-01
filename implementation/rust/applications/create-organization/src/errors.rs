//! Application-level errors for the Create Organization use case.
//!
//! Application errors wrap domain errors and add application-specific context.
//! They are distinct from domain errors and may include infrastructure-level
//! failure information that is not appropriate for the domain layer.

use crate::OrganizationError;

/// Application-level error for the Create Organization use case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateOrganizationError {
    /// Domain validation failed for the provided input.
    Validation(OrganizationField),

    /// Business rule violation: an Organization already exists.
    OrganizationAlreadyExists,

    /// Infrastructure or unexpected failure during repository operation.
    ///
    /// Contains a structured message describing the failure for logging
    /// and diagnostics. This error is never silently suppressed.
    Unexpected(String),
}

/// The field that failed application-level validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrganizationField {
    /// The organization name.
    Name,
    /// The organization type.
    OrganizationType,
}

impl From<OrganizationError> for CreateOrganizationError {
    fn from(error: OrganizationError) -> Self {
        match error {
            OrganizationError::Validation(field) => {
                let app_field = match field {
                    crate::OrganizationField::Name => OrganizationField::Name,
                    crate::OrganizationField::OrganizationType => OrganizationField::OrganizationType,
                };
                CreateOrganizationError::Validation(app_field)
            }
            OrganizationError::OrganizationAlreadyExists => {
                CreateOrganizationError::OrganizationAlreadyExists
            }
            OrganizationError::Unexpected(message) => CreateOrganizationError::Unexpected(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_validation_error_maps_to_application_validation() {
        let domain_error = OrganizationError::Validation(crate::OrganizationField::Name);
        let app_error = CreateOrganizationError::from(domain_error);

        assert_eq!(
            app_error,
            CreateOrganizationError::Validation(OrganizationField::Name)
        );
    }

    #[test]
    fn domain_already_exists_error_maps_correctly() {
        let domain_error = OrganizationError::OrganizationAlreadyExists;
        let app_error = CreateOrganizationError::from(domain_error);

        assert_eq!(
            app_error,
            CreateOrganizationError::OrganizationAlreadyExists
        );
    }

    #[test]
    fn domain_unexpected_error_maps_correctly() {
        let domain_error = OrganizationError::Unexpected("test failure".to_string());
        let app_error = CreateOrganizationError::from(domain_error);

        assert_eq!(
            app_error,
            CreateOrganizationError::Unexpected("test failure".to_string())
        );
    }
}