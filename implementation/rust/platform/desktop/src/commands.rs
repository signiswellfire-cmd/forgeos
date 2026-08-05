//! Tauri command registration for the Organization Platform Layer (TDR-0004).
//!
//! The `createOrganization` command is the IPC boundary for the Create
//! Organization capability. It is a thin function that translates transport
//! data and invokes the Application Service. No domain entities cross the
//! IPC boundary (TDR-0002, TDR-0004, ARCH-0001 TB-2).

use forgeos_create_organization_application::{CreateOrganization, CreateOrganizationCommand};
use forgeos_organization_domain::{
    DefaultOrganizationIdGenerator, OrganizationId, OrganizationIdGenerator,
};
use forgeos_organization_infrastructure::{InMemoryEventPublisher, SqliteOrganizationRepository, SqlxTransaction};
use std::sync::{Arc, Mutex};

use crate::dtos::{CreateOrganizationRequest, CreateOrganizationResponse};
use crate::errors::CreateOrganizationErrorDto;

/// The `createOrganization` Tauri command (TDR-0004).
///
/// This is the IPC boundary for the Create Organization capability. It:
///
/// 1. Performs structural boundary validation (required fields present, non-empty)
/// 2. Maps the request DTO to `CreateOrganizationCommand`
/// 3. Retrieves composed dependencies from Tauri managed state (ISP-0007)
/// 4. Constructs the `CreateOrganization` application service from the repository
/// 5. Invokes `CreateOrganization::execute(command, generator, transaction, event_publisher)`
/// 6. Maps the result to a response DTO or error DTO
///
/// No domain entities cross the IPC boundary. The command function is thin:
/// it translates transport data and invokes the Application Service.
#[tauri::command]
#[allow(non_snake_case)]
pub fn createOrganization(
    request: CreateOrganizationRequest,
    repository: tauri::State<'_, SqliteOrganizationRepository>,
    generator: tauri::State<'_, DefaultOrganizationIdGenerator>,
    transaction: tauri::State<'_, Arc<Mutex<SqlxTransaction>>>,
    event_publisher: tauri::State<'_, Arc<Mutex<InMemoryEventPublisher>>>,
) -> Result<CreateOrganizationResponse, CreateOrganizationErrorDto> {
    // Step 1: Structural boundary validation (ARCH-0001 TB-2, TDR-0004).
    if let Some(error) = validate_request(&request) {
        return Err(error);
    }

    // Step 2: DTO-to-Command mapping (TDR-0004, ISP-0002).
    let command = map_request_to_command(&request);

    // Step 3: Construct application service from composed repository (ISP-0007).
    // The service holds a borrowed reference; it is constructed per-request
    // because Tauri's state management requires `'static` lifetimes.
    let service = CreateOrganization::new(&*repository);

    // Step 4: Invoke application service with transaction coordination (TDR-0004, ISP-0001, ISP-0006).
    let generator_ref: &dyn OrganizationIdGenerator = &*generator;
    let mut transaction_lock = transaction.inner().lock().unwrap();
    let mut event_publisher_lock = event_publisher.inner().lock().unwrap();
    let result = service.execute(command, generator_ref, &mut *transaction_lock, &mut *event_publisher_lock);

    // Step 5: Map result to response DTO or error DTO (TDR-0004, ISP-0008).
    result
        .map(|org_id| map_result_to_response(org_id, &request))
        .map_err(CreateOrganizationErrorDto::from)
}

/// Performs structural boundary validation on the request DTO
/// (ARCH-0001 TB-2, TDR-0004).
///
/// Validates that required fields are present and non-empty. This is a
/// trust-boundary concern distinct from the business validation performed
/// by the Application Service through domain value objects.
///
/// Returns `Some(error)` if structural validation fails, `None` if the
/// request is structurally valid.
fn validate_request(request: &CreateOrganizationRequest) -> Option<CreateOrganizationErrorDto> {
    if request.name.is_empty() {
        return Some(CreateOrganizationErrorDto::validation_name());
    }
    if request.organization_type.is_empty() {
        return Some(CreateOrganizationErrorDto::validation_type());
    }
    None
}

/// Maps a request DTO to an application command (TDR-0004, ISP-0002).
///
/// This is a boundary translation: the IPC DTO is translated to the
/// Application Layer command. No business validation occurs here.
fn map_request_to_command(request: &CreateOrganizationRequest) -> CreateOrganizationCommand {
    CreateOrganizationCommand::new(&request.name, &request.organization_type)
}

/// Maps an `OrganizationId` result to a response DTO (TDR-0004).
///
/// The response DTO is constructed from:
/// - `organizationId`: the application service return value
/// - `name`, `organizationType`: the request DTO fields
/// - `status`: approved default `"Active"` (MILESTONE-001-DOMAIN-DECISIONS)
/// - `version`: approved default `1` (MILESTONE-001-DOMAIN-DECISIONS)
fn map_result_to_response(
    organization_id: OrganizationId,
    request: &CreateOrganizationRequest,
) -> CreateOrganizationResponse {
    CreateOrganizationResponse {
        organization_id: organization_id.as_str(),
        name: request.name.clone(),
        organization_type: request.organization_type.clone(),
        status: "Active".to_string(),
        version: 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use forgeos_create_organization_application::{CreateOrganizationError, OrganizationField};

    #[test]
    fn create_organization_command_maps_dto_to_command() {
        let request = CreateOrganizationRequest {
            name: "ForgeOS".to_string(),
            organization_type: "foundation".to_string(),
        };

        let command = map_request_to_command(&request);

        assert_eq!(command.name, "ForgeOS");
        assert_eq!(command.organization_type, "foundation");
    }

    #[test]
    fn create_organization_command_maps_result_to_response() {
        let request = CreateOrganizationRequest {
            name: "ForgeOS".to_string(),
            organization_type: "foundation".to_string(),
        };

        let uuid = uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let org_id = OrganizationId::from(uuid);

        let response = map_result_to_response(org_id, &request);

        assert_eq!(
            response.organization_id,
            "550e8400-e29b-41d4-a716-446655440000"
        );
        assert_eq!(response.name, "ForgeOS");
        assert_eq!(response.organization_type, "foundation");
        assert_eq!(response.status, "Active");
        assert_eq!(response.version, 1);
    }

    #[test]
    fn create_organization_command_maps_error_to_ipc_error() {
        // Validation(Name) → VALIDATION_ERROR
        let app_error = CreateOrganizationError::Validation(OrganizationField::Name);
        let ipc_error = CreateOrganizationErrorDto::from(app_error);
        assert_eq!(ipc_error.code, "VALIDATION_ERROR");
        assert_eq!(ipc_error.message, "Organization name is invalid");

        // Validation(OrganizationType) → VALIDATION_ERROR
        let app_error = CreateOrganizationError::Validation(OrganizationField::OrganizationType);
        let ipc_error = CreateOrganizationErrorDto::from(app_error);
        assert_eq!(ipc_error.code, "VALIDATION_ERROR");
        assert_eq!(ipc_error.message, "Organization type is invalid");

        // OrganizationAlreadyExists → ORGANIZATION_ALREADY_EXISTS
        let app_error = CreateOrganizationError::OrganizationAlreadyExists;
        let ipc_error = CreateOrganizationErrorDto::from(app_error);
        assert_eq!(ipc_error.code, "ORGANIZATION_ALREADY_EXISTS");

        // Unexpected(_) → UNEXPECTED_ERROR (internal message suppressed)
        let app_error = CreateOrganizationError::Unexpected("internal db failure".to_string());
        let ipc_error = CreateOrganizationErrorDto::from(app_error);
        assert_eq!(ipc_error.code, "UNEXPECTED_ERROR");
        assert_eq!(ipc_error.message, "An unexpected error occurred");
        assert!(!ipc_error.message.contains("internal db failure"));
    }

    #[test]
    fn structural_validation_rejects_empty_name() {
        let request = CreateOrganizationRequest {
            name: "".to_string(),
            organization_type: "foundation".to_string(),
        };

        let error = validate_request(&request);
        assert!(error.is_some());
        let error = error.unwrap();
        assert_eq!(error.code, "VALIDATION_ERROR");
        assert_eq!(error.message, "Organization name is invalid");
    }

    #[test]
    fn structural_validation_rejects_empty_organization_type() {
        let request = CreateOrganizationRequest {
            name: "ForgeOS".to_string(),
            organization_type: "".to_string(),
        };

        let error = validate_request(&request);
        assert!(error.is_some());
        let error = error.unwrap();
        assert_eq!(error.code, "VALIDATION_ERROR");
        assert_eq!(error.message, "Organization type is invalid");
    }

    #[test]
    fn structural_validation_accepts_non_empty_fields() {
        let request = CreateOrganizationRequest {
            name: "ForgeOS".to_string(),
            organization_type: "foundation".to_string(),
        };

        let error = validate_request(&request);
        assert!(error.is_none());
    }

    #[test]
    fn response_construction_uses_approved_defaults() {
        // Verify that status = "Active" and version = 1 per
        // MILESTONE-001-DOMAIN-DECISIONS.
        let request = CreateOrganizationRequest {
            name: "ForgeOS".to_string(),
            organization_type: "foundation".to_string(),
        };

        let uuid = uuid::Uuid::new_v4();
        let org_id = OrganizationId::from(uuid);
        let response = map_result_to_response(org_id, &request);

        assert_eq!(response.status, "Active");
        assert_eq!(response.version, 1);
    }
}