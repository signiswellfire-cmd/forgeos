//! Presentation-layer error types (ISP-0008, TDR-0004).
//!
//! The Presentation Layer maps a failed `createOrganization` IPC dispatch to a
//! `PresentationError` with a stable error code and a safe, user-facing
//! message. No domain, application, or infrastructure error types leak into
//! the Presentation Layer; the only cross-boundary error type consumed is the
//! Platform's `CreateOrganizationErrorDto` (TDR-0004, ISP-0008).

use forgeos_desktop_platform::CreateOrganizationErrorDto;

/// The form field associated with a structural validation failure at TB-1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentationField {
    /// The organization name form field.
    Name,
    /// The organization type form field.
    OrganizationType,
}

/// Presentation-layer error with a stable error code and safe message
/// (ISP-0008, TDR-0004).
///
/// The `code` is a stable public identifier for the failure category and the
/// `message` is a safe, user-facing message. Error DTOs received from the
/// backend are preserved as-is: their codes and messages are already safe and
/// stable by the Platform boundary contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresentationError {
    code: String,
    message: String,
}

impl PresentationError {
    /// Creates a structural validation error for a form field (TB-1).
    pub fn validation(field: PresentationField) -> Self {
        match field {
            PresentationField::Name => Self {
                code: "PRESENTATION_VALIDATION_ERROR".to_string(),
                message: "Organization name cannot be empty".to_string(),
            },
            PresentationField::OrganizationType => Self {
                code: "PRESENTATION_VALIDATION_ERROR".to_string(),
                message: "Organization type cannot be empty".to_string(),
            },
        }
    }

    /// Creates a presentation error from the backend error DTO (TDR-0004).
    ///
    /// The backend error DTO already carries a stable error code and a safe,
    /// user-facing message (ISP-0008), so it is preserved unchanged.
    pub fn from_error_dto(dto: &CreateOrganizationErrorDto) -> Self {
        Self {
            code: dto.code.clone(),
            message: dto.message.clone(),
        }
    }

    /// Returns the stable error code.
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Returns the safe, user-facing message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Converts the presentation error to a display-safe IPC error DTO
    /// (TDR-0004, ISP-0008).
    ///
    /// The DTO carries the same stable error code and safe message so the view
    /// model can display the failure uniformly.
    pub(crate) fn to_error_dto(&self) -> CreateOrganizationErrorDto {
        CreateOrganizationErrorDto {
            code: self.code.clone(),
            message: self.message.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_validation_error_has_stable_code_and_safe_message() {
        let error = PresentationError::validation(PresentationField::Name);

        assert_eq!(error.code(), "PRESENTATION_VALIDATION_ERROR");
        assert_eq!(error.message(), "Organization name cannot be empty");
    }

    #[test]
    fn organization_type_validation_error_has_stable_code_and_safe_message() {
        let error = PresentationError::validation(PresentationField::OrganizationType);

        assert_eq!(error.code(), "PRESENTATION_VALIDATION_ERROR");
        assert_eq!(error.message(), "Organization type cannot be empty");
    }

    #[test]
    fn backend_error_dto_is_preserved_without_leakage() {
        let dto = CreateOrganizationErrorDto {
            code: "ORGANIZATION_ALREADY_EXISTS".to_string(),
            message: "An organization already exists for this ForgeOS instance".to_string(),
        };

        let error = PresentationError::from_error_dto(&dto);

        assert_eq!(error.code(), "ORGANIZATION_ALREADY_EXISTS");
        assert_eq!(
            error.message(),
            "An organization already exists for this ForgeOS instance"
        );
        // No backend internals leak into the presentation error.
        assert!(!error.message().contains("database"));
        assert!(!error.message().contains("internal"));
    }

    #[test]
    fn to_error_dto_preserves_stable_code_and_safe_message() {
        let error = PresentationError::validation(PresentationField::Name);
        let dto = error.to_error_dto();

        assert_eq!(dto.code, "PRESENTATION_VALIDATION_ERROR");
        assert_eq!(dto.message, "Organization name cannot be empty");
    }

    #[test]
    fn error_code_mapping_is_stable_and_deterministic() {
        let name1 = PresentationError::validation(PresentationField::Name);
        let name2 = PresentationError::validation(PresentationField::Name);
        let org_type1 = PresentationError::validation(PresentationField::OrganizationType);
        let org_type2 = PresentationError::validation(PresentationField::OrganizationType);

        assert_eq!(name1, name2);
        assert_eq!(org_type1, org_type2);
        assert_ne!(name1, org_type1);
    }
}