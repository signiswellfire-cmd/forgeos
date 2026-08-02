//! IPC Error DTO and error translation for the Organization Platform Layer
//! (TDR-0004, ISP-0008).
//!
//! The Platform Layer translates `CreateOrganizationError` (Application) to a
//! stable IPC error DTO at the boundary. The error DTO contains a stable error
//! code and a safe, user-facing message. It never serializes Rust error chains,
//! database details, or domain internals (TDR-0004, ISP-0008).

use forgeos_create_organization_application::{CreateOrganizationError, OrganizationField};
use serde::{Deserialize, Serialize};

/// IPC error DTO returned when the `createOrganization` command fails
/// (TDR-0004, ISP-0008).
///
/// Contains a stable error code and a safe, user-facing message. No
/// infrastructure, database, or domain internal details are exposed across
/// the IPC boundary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrganizationErrorDto {
    /// Stable error code identifying the failure category (TDR-0004).
    pub code: String,

    /// Safe, user-facing message with no internal details (ISP-0008).
    pub message: String,
}

impl CreateOrganizationErrorDto {
    /// Creates a validation error DTO for the `Name` field.
    pub fn validation_name() -> Self {
        Self {
            code: "VALIDATION_ERROR".to_string(),
            message: "Organization name is invalid".to_string(),
        }
    }

    /// Creates a validation error DTO for the `OrganizationType` field.
    pub fn validation_type() -> Self {
        Self {
            code: "VALIDATION_ERROR".to_string(),
            message: "Organization type is invalid".to_string(),
        }
    }

    /// Creates an organization-already-exists error DTO.
    pub fn already_exists() -> Self {
        Self {
            code: "ORGANIZATION_ALREADY_EXISTS".to_string(),
            message: "An organization already exists for this ForgeOS instance".to_string(),
        }
    }

    /// Creates an unexpected error DTO with the internal message suppressed.
    pub fn unexpected() -> Self {
        Self {
            code: "UNEXPECTED_ERROR".to_string(),
            message: "An unexpected error occurred".to_string(),
        }
    }
}

/// Translates an Application `CreateOrganizationError` to the IPC error DTO
/// at the Platform boundary (TDR-0004, ISP-0008).
///
/// The `Unexpected` variant's internal `String` message is never exposed
/// across IPC. Error codes are stable public API identifiers.
impl From<CreateOrganizationError> for CreateOrganizationErrorDto {
    fn from(error: CreateOrganizationError) -> Self {
        match error {
            CreateOrganizationError::Validation(OrganizationField::Name) => {
                CreateOrganizationErrorDto::validation_name()
            }
            CreateOrganizationError::Validation(OrganizationField::OrganizationType) => {
                CreateOrganizationErrorDto::validation_type()
            }
            CreateOrganizationError::OrganizationAlreadyExists => {
                CreateOrganizationErrorDto::already_exists()
            }
            CreateOrganizationError::Unexpected(_) => CreateOrganizationErrorDto::unexpected(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_name_error_translates_to_ipc_error() {
        let app_error = CreateOrganizationError::Validation(OrganizationField::Name);
        let ipc_error = CreateOrganizationErrorDto::from(app_error);

        assert_eq!(ipc_error.code, "VALIDATION_ERROR");
        assert_eq!(ipc_error.message, "Organization name is invalid");
    }

    #[test]
    fn validation_type_error_translates_to_ipc_error() {
        let app_error = CreateOrganizationError::Validation(OrganizationField::OrganizationType);
        let ipc_error = CreateOrganizationErrorDto::from(app_error);

        assert_eq!(ipc_error.code, "VALIDATION_ERROR");
        assert_eq!(ipc_error.message, "Organization type is invalid");
    }

    #[test]
    fn already_exists_error_translates_to_ipc_error() {
        let app_error = CreateOrganizationError::OrganizationAlreadyExists;
        let ipc_error = CreateOrganizationErrorDto::from(app_error);

        assert_eq!(ipc_error.code, "ORGANIZATION_ALREADY_EXISTS");
        assert_eq!(
            ipc_error.message,
            "An organization already exists for this ForgeOS instance"
        );
    }

    #[test]
    fn unexpected_error_translates_to_ipc_error() {
        let app_error = CreateOrganizationError::Unexpected("database connection failed".to_string());
        let ipc_error = CreateOrganizationErrorDto::from(app_error);

        assert_eq!(ipc_error.code, "UNEXPECTED_ERROR");
        assert_eq!(ipc_error.message, "An unexpected error occurred");
        // The internal message must never leak across IPC.
        assert!(!ipc_error.message.contains("database connection failed"));
    }

    #[test]
    fn error_dto_serializes_correctly() {
        let error = CreateOrganizationErrorDto::validation_name();

        let json = serde_json::to_string(&error).expect("serialization should succeed");
        let deserialized: CreateOrganizationErrorDto =
            serde_json::from_str(&json).expect("deserialization should succeed");

        assert_eq!(deserialized.code, "VALIDATION_ERROR");
        assert_eq!(deserialized.message, "Organization name is invalid");
    }

    #[test]
    fn error_translation_is_deterministic() {
        // Equivalent application errors produce equivalent IPC error DTOs.
        let error1 = CreateOrganizationErrorDto::from(CreateOrganizationError::OrganizationAlreadyExists);
        let error2 = CreateOrganizationErrorDto::from(CreateOrganizationError::OrganizationAlreadyExists);

        assert_eq!(error1.code, error2.code);
        assert_eq!(error1.message, error2.message);
    }
}