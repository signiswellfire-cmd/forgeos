//! IPC integration for the Presentation Layer (TDR-0002, TDR-0004).
//!
//! The Presentation Layer communicates with the backend **exclusively**
//! through the `createOrganization` Tauri command published by the Platform
//! Layer (`forgeos-desktop-platform`). This module owns the IPC dispatch
//! contract:
//!
//! - `validate_view_model` — structural validation of form inputs at TB-1
//!   (ARCH-0001). User input is untrusted; business validation remains in the
//!   Domain Layer.
//! - `build_request` — construction of the immutable `CreateOrganizationRequest`
//!   DTO from view model state (TDR-0004).
//! - `invoke_create_organization` — dispatch of the `createOrganization`
//!   command with the request DTO and application of the command's response or
//!   error DTO to the view model.
//!
//! Only DTOs cross the IPC boundary (ARCH-0001 TB-2, TDR-0004). No domain
//! entities are imported or referenced. No business logic exists here.

use forgeos_desktop_platform::{
    CreateOrganizationErrorDto, CreateOrganizationRequest, CreateOrganizationResponse,
};

use crate::errors::{PresentationError, PresentationField};
use crate::view_model::{CreateOrganizationViewModel, SubmissionStatus};

/// The `createOrganization` command IPC dispatch function (TDR-0002, TDR-0004).
///
/// Dispatches the Platform's `createOrganization` Tauri command through the
/// IPC boundary with a `CreateOrganizationRequest` DTO and returns the
/// command's `Result` — either a `CreateOrganizationResponse` DTO or a
/// `CreateOrganizationErrorDto`.
///
/// The Platform command is a `#[tauri::command]` invoked by the Tauri runtime
/// with its managed state (ISP-0007). The actual IPC round-trip is performed
/// by the (deferred) frontend framework binding per TDR-0002 future
/// considerations. This trait describes that dispatch contract so the
/// Presentation Layer and its tests can exercise the full submission flow
/// deterministically.
pub trait CreateOrganizationDispatcher: Fn(
    &CreateOrganizationRequest,
) -> Result<CreateOrganizationResponse, CreateOrganizationErrorDto> {
}

impl<F> CreateOrganizationDispatcher for F where
    F: Fn(
        &CreateOrganizationRequest,
    ) -> Result<CreateOrganizationResponse, CreateOrganizationErrorDto>
{
}

/// Performs structural validation of the form inputs at TB-1 (ARCH-0001,
/// MILESTONE-001.9).
///
/// User input is considered untrusted. Structural validation verifies that
/// required form fields are present (non-empty), mirroring the Platform
/// boundary's structural validation at TB-2. Business validation (e.g.,
/// non-whitespace content rules) remains in the Domain Layer and is not
/// duplicated here.
///
/// Returns `Ok(())` when the form is structurally valid.
pub fn validate_view_model(
    view_model: &CreateOrganizationViewModel,
) -> Result<(), PresentationError> {
    if view_model.name().is_empty() {
        return Err(PresentationError::validation(PresentationField::Name));
    }
    if view_model.organization_type().is_empty() {
        return Err(PresentationError::validation(
            PresentationField::OrganizationType,
        ));
    }
    Ok(())
}

/// Builds the immutable `CreateOrganizationRequest` DTO from view model state
/// (TDR-0004).
///
/// The request DTO carries only the user-supplied `name` and `organizationType`
/// across the IPC boundary. It contains no business behavior and exposes no
/// domain entities.
pub fn build_request(view_model: &CreateOrganizationViewModel) -> CreateOrganizationRequest {
    CreateOrganizationRequest {
        name: view_model.name().to_string(),
        organization_type: view_model.organization_type().to_string(),
    }
}

/// Dispatches the `createOrganization` Tauri command with the request DTO and
/// updates the view model with the response or error (MILESTONE-001.9,
/// TDR-0002, TDR-0004).
///
/// This is the Presentation Layer's IPC dispatch entry. It:
///
/// 1. Performs structural validation of the form inputs at TB-1. If
///    validation fails, the view model is updated with the presentation
///    validation error and **no dispatch is attempted**.
/// 2. Builds the immutable `CreateOrganizationRequest` DTO from view model
///    state.
/// 3. Marks the submission as in flight ([`SubmissionStatus::Submitting`]).
/// 4. Dispatches the `createOrganization` command through `dispatcher` — the
///    IPC boundary wiring supplied by the (deferred) frontend framework
///    binding (TDR-0002) or by the composition root (ISP-0007).
/// 5. Updates the view model with the command's response DTO (success) or
///    error DTO (failure).
///
/// # Traceability note
///
/// The approved API signature in MILESTONE-001.9 is
/// `invoke_create_organization(view_model: &CreateOrganizationViewModel)`.
/// The approved behavior — "updates the view model with the response or
/// error" — requires a mutable view model in Rust, and the dispatch outcome
/// requires the IPC transport result. This implementation therefore takes
/// `&mut CreateOrganizationViewModel` and the `dispatcher`, which is the
/// minimal faithful realization of the approved behavior.
pub fn invoke_create_organization<D>(
    view_model: &mut CreateOrganizationViewModel,
    dispatcher: &D,
) where
    D: CreateOrganizationDispatcher,
{
    // Step 1: Structural validation at TB-1 (ARCH-0001).
    if let Err(error) = validate_view_model(view_model) {
        view_model.set_error(error.to_error_dto());
        return;
    }

    // Step 2: Request DTO construction from view model state (TDR-0004).
    let request = build_request(view_model);

    // Step 3: Submission is in flight.
    view_model.set_status(SubmissionStatus::Submitting);

    // Step 4: Dispatch the `createOrganization` command through the IPC
    // boundary and apply the result to the view model.
    let command_result = dispatcher(&request);
    match command_result {
        Ok(response) => view_model.set_response(response),
        Err(error_dto) => view_model.set_error(error_dto),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn view_model_with_inputs(name: &str, organization_type: &str) -> CreateOrganizationViewModel {
        let mut vm = CreateOrganizationViewModel::new();
        vm.set_name(name.to_string());
        vm.set_organization_type(organization_type.to_string());
        vm
    }

    fn sample_response() -> CreateOrganizationResponse {
        CreateOrganizationResponse {
            organization_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
            name: "ForgeOS".to_string(),
            organization_type: "foundation".to_string(),
            status: "Active".to_string(),
            version: 1,
        }
    }

    #[test]
    fn validate_view_model_accepts_non_empty_inputs() {
        let vm = view_model_with_inputs("ForgeOS", "foundation");
        assert!(validate_view_model(&vm).is_ok());
    }

    #[test]
    fn validate_view_model_rejects_empty_name() {
        let vm = view_model_with_inputs("", "foundation");
        let error = validate_view_model(&vm).unwrap_err();
        assert_eq!(error.code(), "PRESENTATION_VALIDATION_ERROR");
        assert_eq!(error.message(), "Organization name cannot be empty");
    }

    #[test]
    fn validate_view_model_rejects_empty_organization_type() {
        let vm = view_model_with_inputs("ForgeOS", "");
        let error = validate_view_model(&vm).unwrap_err();
        assert_eq!(error.code(), "PRESENTATION_VALIDATION_ERROR");
        assert_eq!(error.message(), "Organization type cannot be empty");
    }

    #[test]
    fn build_request_constructs_request_dto_from_view_model() {
        let vm = view_model_with_inputs("ForgeOS", "foundation");
        let request = build_request(&vm);

        assert_eq!(request.name, "ForgeOS");
        assert_eq!(request.organization_type, "foundation");
    }

    #[test]
    fn invoke_dispatches_command_and_applies_response_to_view_model() {
        let mut vm = view_model_with_inputs("ForgeOS", "foundation");
        let response = sample_response();
        let dispatched_request = std::cell::RefCell::new(None);

        let dispatcher = |request: &CreateOrganizationRequest| {
            *dispatched_request.borrow_mut() = Some(request.clone());
            Ok(response.clone())
        };

        invoke_create_organization(&mut vm, &dispatcher);

        // The command was dispatched with the request DTO built from the VM.
        let dispatched = dispatched_request.borrow();
        let dispatched = dispatched.as_ref().expect("dispatch should have occurred");
        assert_eq!(dispatched.name, "ForgeOS");
        assert_eq!(dispatched.organization_type, "foundation");

        // The view model was updated with the response DTO.
        assert_eq!(vm.status(), &SubmissionStatus::Success);
        let vm_response = vm.response().expect("response should be present");
        assert_eq!(vm_response.organization_id, "550e8400-e29b-41d4-a716-446655440000");
        assert_eq!(vm_response.status, "Active");
        assert_eq!(vm_response.version, 1);
        assert!(vm.error().is_none());
    }

    #[test]
    fn invoke_dispatches_command_and_applies_error_to_view_model() {
        let mut vm = view_model_with_inputs("ForgeOS", "foundation");
        let error_dto = CreateOrganizationErrorDto {
            code: "ORGANIZATION_ALREADY_EXISTS".to_string(),
            message: "An organization already exists for this ForgeOS instance".to_string(),
        };

        let dispatcher = |_request: &CreateOrganizationRequest| Err(error_dto.clone());

        invoke_create_organization(&mut vm, &dispatcher);

        // The view model was updated with the error DTO.
        assert_eq!(vm.status(), &SubmissionStatus::Error);
        let vm_error = vm.error().expect("error should be present");
        assert_eq!(vm_error.code, "ORGANIZATION_ALREADY_EXISTS");
        assert_eq!(
            vm_error.message,
            "An organization already exists for this ForgeOS instance"
        );
        assert!(vm.response().is_none());
    }

    #[test]
    fn invoke_does_not_dispatch_when_form_is_invalid() {
        let mut vm = view_model_with_inputs("", "foundation");
        let dispatch_called = std::cell::Cell::new(false);

        let dispatcher = |_request: &CreateOrganizationRequest| {
            dispatch_called.set(true);
            Ok(sample_response())
        };

        invoke_create_organization(&mut vm, &dispatcher);

        // No dispatch occurred and the view model shows the validation error.
        assert!(!dispatch_called.get());
        assert_eq!(vm.status(), &SubmissionStatus::Error);
        let error = vm.error().expect("error should be present");
        assert_eq!(error.code, "PRESENTATION_VALIDATION_ERROR");
        assert_eq!(error.message, "Organization name cannot be empty");
        assert!(vm.response().is_none());
    }

    #[test]
    fn request_dto_serializes_with_camel_case_fields_for_ipc() {
        // Verify the IPC contract: the request DTO serializes with camelCase
        // "organizationType" exactly as the Platform command deserializes it
        // (TDR-0004).
        let vm = view_model_with_inputs("ForgeOS", "foundation");
        let request = build_request(&vm);

        let json = serde_json::to_string(&request).expect("serialization should succeed");
        let deserialized: CreateOrganizationRequest =
            serde_json::from_str(&json).expect("deserialization should succeed");

        assert_eq!(deserialized.name, "ForgeOS");
        assert_eq!(deserialized.organization_type, "foundation");
        assert!(
            json.contains("\"organizationType\""),
            "expected camelCase field name 'organizationType' in JSON: {json}"
        );
        assert!(
            !json.contains("\"organization_type\""),
            "snake_case field name must not appear in JSON: {json}"
        );
    }
}