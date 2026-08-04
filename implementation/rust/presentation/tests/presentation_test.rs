//! Presentation integration tests (ISP-0009, ISP-0010, MILESTONE-001.9).
//!
//! These tests exercise the Presentation Layer as a whole through its public
//! API: view model state transitions, form input validation at TB-1, request
//! DTO construction, `createOrganization` command dispatch via the IPC
//! contract, response DTO handling, error DTO handling, and the end-to-end
//! flow from form submission to response display.
//!
//! No domain, application, or infrastructure crates are imported. The
//! Presentation Layer depends only on the Platform crate for DTO types
//! (ARCH-0003), and this test file verifies that boundary contract.

use forgeos_organization_presentation::{
    build_request, form_title, invoke_create_organization, name_field_label, render_form,
    render_response, submit, CreateOrganizationViewModel, PresentationError, PresentationField,
    SubmissionStatus,
};
use forgeos_desktop_platform::{
    CreateOrganizationErrorDto, CreateOrganizationRequest, CreateOrganizationResponse,
};

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
fn end_to_end_form_submission_to_response_display() {
    let mut vm = view_model_with_inputs("ForgeOS", "foundation");

    let fields = render_form(&vm);
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].label(), name_field_label());
    assert_eq!(fields[0].value(), "ForgeOS");

    let dispatcher = |request: &CreateOrganizationRequest| {
        assert_eq!(request.name, "ForgeOS");
        assert_eq!(request.organization_type, "foundation");
        Ok(sample_response())
    };

    let status = submit(&mut vm, &dispatcher);
    assert_eq!(status, SubmissionStatus::Success);
    assert_eq!(vm.status(), &SubmissionStatus::Success);

    let rows = render_response(&vm).expect("response should be rendered");
    assert_eq!(rows.len(), 5);
    assert_eq!(rows[3].label(), "Status");
    assert_eq!(rows[3].value(), "Active");
    assert_eq!(rows[4].label(), "Version");
    assert_eq!(rows[4].value(), "1");
}

#[test]
fn invoke_dispatches_request_dto_and_applies_response() {
    let mut vm = view_model_with_inputs("ForgeOS", "foundation");

    let dispatched = std::cell::RefCell::new(None);
    let dispatcher = |request: &CreateOrganizationRequest| {
        *dispatched.borrow_mut() = Some(request.clone());
        Ok(sample_response())
    };

    invoke_create_organization(&mut vm, &dispatcher);

    let request = dispatched.borrow();
    let request = request.as_ref().expect("command should have been dispatched");
    assert_eq!(request.name, "ForgeOS");
    assert_eq!(request.organization_type, "foundation");

    assert_eq!(vm.status(), &SubmissionStatus::Success);
    let response = vm.response().expect("response should be present");
    assert_eq!(response.organization_id, "550e8400-e29b-41d4-a716-446655440000");
    assert_eq!(response.status, "Active");
    assert_eq!(response.version, 1);
    assert!(vm.error().is_none());
}

#[test]
fn invoke_applies_backend_error_dto_to_view_model() {
    let mut vm = view_model_with_inputs("ForgeOS", "foundation");
    let error_dto = CreateOrganizationErrorDto {
        code: "ORGANIZATION_ALREADY_EXISTS".to_string(),
        message: "An organization already exists for this ForgeOS instance".to_string(),
    };

    let dispatcher = |_request: &CreateOrganizationRequest| Err(error_dto.clone());

    invoke_create_organization(&mut vm, &dispatcher);

    assert_eq!(vm.status(), &SubmissionStatus::Error);
    let error = vm.error().expect("error should be present");
    assert_eq!(error.code, "ORGANIZATION_ALREADY_EXISTS");
    assert_eq!(
        error.message,
        "An organization already exists for this ForgeOS instance"
    );
    assert!(vm.response().is_none());
}

#[test]
fn invalid_form_blocks_dispatch_and_shows_validation_error() {
    let mut vm = view_model_with_inputs("", "foundation");
    let dispatch_called = std::cell::Cell::new(false);

    let dispatcher = |_request: &CreateOrganizationRequest| {
        dispatch_called.set(true);
        Ok(sample_response())
    };

    invoke_create_organization(&mut vm, &dispatcher);

    assert!(!dispatch_called.get());
    assert_eq!(vm.status(), &SubmissionStatus::Error);
    let error = vm.error().expect("error should be present");
    assert_eq!(error.code, "PRESENTATION_VALIDATION_ERROR");
    assert_eq!(error.message, "Organization name cannot be empty");
    assert!(vm.response().is_none());
}

#[test]
fn build_request_constructs_ipc_compatible_dto() {
    let vm = view_model_with_inputs("ForgeOS", "foundation");
    let request = build_request(&vm);

    let json = serde_json::to_string(&request).expect("serialization should succeed");
    let deserialized: CreateOrganizationRequest =
        serde_json::from_str(&json).expect("deserialization should succeed");

    assert_eq!(deserialized.name, "ForgeOS");
    assert_eq!(deserialized.organization_type, "foundation");
    assert!(
        json.contains("\"organizationType\""),
        "expected camelCase field name 'organizationType': {json}"
    );
    assert!(
        !json.contains("\"organization_type\""),
        "snake_case must not appear in IPC JSON: {json}"
    );
}

#[test]
fn presentation_error_codes_are_stable() {
    let name_error = PresentationError::validation(PresentationField::Name);
    let type_error = PresentationError::validation(PresentationField::OrganizationType);

    assert_eq!(name_error.code(), "PRESENTATION_VALIDATION_ERROR");
    assert_eq!(type_error.code(), "PRESENTATION_VALIDATION_ERROR");
    assert_eq!(name_error.message(), "Organization name cannot be empty");
    assert_eq!(type_error.message(), "Organization type cannot be empty");
}

#[test]
fn view_model_state_transitions_idle_to_success() {
    let mut vm = view_model_with_inputs("ForgeOS", "foundation");
    assert_eq!(vm.status(), &SubmissionStatus::Idle);

    let dispatcher = |_request: &CreateOrganizationRequest| Ok(sample_response());
    let status = submit(&mut vm, &dispatcher);

    assert_eq!(status, SubmissionStatus::Success);
    assert_eq!(vm.status(), &SubmissionStatus::Success);
}

#[test]
fn form_renders_static_ui_contract() {
    assert_eq!(form_title(), "Create Organization");
    assert_eq!(name_field_label(), "Organization Name");

    let vm = view_model_with_inputs("ForgeOS", "foundation");
    let fields = render_form(&vm);

    assert_eq!(fields[0].label(), "Organization Name");
    assert_eq!(fields[0].value(), "ForgeOS");
    assert_eq!(fields[1].label(), "Organization Type");
    assert_eq!(fields[1].value(), "foundation");
}
