//! UI composition and rendering for the Create Organization form (ARCH-0002,
//! TDR-0002, MILESTONE-001.9).
//!
//! The frontend framework selection remains deferred per `TDR-0002` Future
//! Considerations. This module therefore provides a **framework-neutral**
//! rendering contract that a future frontend binding can consume:
//!
//! - field labels and the form title;
//! - a form renderer that returns the current form fields and values for
//!   binding;
//! - a response renderer that maps the `CreateOrganizationResponse` DTO to
//!   display rows;
//! - an error renderer that maps the `CreateOrganizationErrorDto` to a
//!   display row;
//! - a submit handler that dispatches the `createOrganization` command
//!   through the IPC boundary.
//!
//! The UI contains **no business logic**, **no domain logic**, **no
//! persistence logic**, **no workflow rules**, and **no governance rules**
//! (MILESTONE-001.9, ARCH-0003). User input is untrusted (ARCH-0001 TB-1).
//! No UI framework or component library is selected here.

use forgeos_desktop_platform::CreateOrganizationErrorDto;

use crate::ipc::{invoke_create_organization, CreateOrganizationDispatcher};
use crate::view_model::{CreateOrganizationViewModel, SubmissionStatus};

/// A label–value pair for framework-neutral display rendering.
///
/// Used by the form, response, and error renderers so the (deferred)
/// frontend framework binding can bind or display each row without the
/// Presentation Layer depending on any UI framework.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayField {
    label: String,
    value: String,
}

impl DisplayField {
    /// Creates a new display field with a label and value.
    pub fn new(label: &str, value: &str) -> Self {
        Self {
            label: label.to_string(),
            value: value.to_string(),
        }
    }

    /// Returns the field label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the field value.
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// The title displayed above the Create Organization form.
pub fn form_title() -> &'static str {
    "Create Organization"
}

/// The label for the organization name form field.
pub fn name_field_label() -> &'static str {
    "Organization Name"
}

/// The label for the organization type form field.
pub fn organization_type_field_label() -> &'static str {
    "Organization Type"
}

/// The label for the submit button.
pub fn submit_button_label() -> &'static str {
    "Create Organization"
}

/// Renders the Create Organization form fields with their current values
/// from the view model.
///
/// Returns the form fields in display order for binding by the (deferred)
/// frontend framework:
///
/// 1. Organization Name
/// 2. Organization Type
pub fn render_form(view_model: &CreateOrganizationViewModel) -> Vec<DisplayField> {
    vec![
        DisplayField::new(name_field_label(), view_model.name()),
        DisplayField::new(
            organization_type_field_label(),
            view_model.organization_type(),
        ),
    ]
}

/// Renders the `CreateOrganizationResponse` DTO for display on successful
/// creation (TDR-0004).
///
/// Returns `None` when no response is present in the view model. The response
/// DTO fields are mapped to display rows:
///
/// 1. Organization ID
/// 2. Name
/// 3. Organization Type
/// 4. Status
/// 5. Version
pub fn render_response(view_model: &CreateOrganizationViewModel) -> Option<Vec<DisplayField>> {
    let response = view_model.response()?;
    Some(vec![
        DisplayField::new("Organization ID", &response.organization_id),
        DisplayField::new("Name", &response.name),
        DisplayField::new("Organization Type", &response.organization_type),
        DisplayField::new("Status", &response.status),
        DisplayField::new("Version", &response.version.to_string()),
    ])
}

/// Renders the `CreateOrganizationErrorDto` for display on failure
/// (TDR-0004, ISP-0008).
///
/// Returns `None` when no error is present in the view model. The rendered
/// row shows the stable error code as the label and the safe, user-facing
/// message as the value. No backend or domain internals are displayed.
pub fn render_error(view_model: &CreateOrganizationViewModel) -> Option<DisplayField> {
    let error = view_model.error()?;
    Some(render_error_dto(error))
}

/// Renders an error DTO as a display field (stable code + safe message).
fn render_error_dto(error: &CreateOrganizationErrorDto) -> DisplayField {
    DisplayField::new(&error.code, &error.message)
}

/// Submits the Create Organization form.
///
/// Dispatches the `createOrganization` command through the IPC boundary via
/// `dispatcher` and updates the view model with the response or error DTO.
/// Returns the resulting submission status so the UI can react to it.
///
/// `dispatcher` is the IPC boundary transport supplied by the (deferred)
/// frontend framework binding per `TDR-0002` future considerations, or by the
/// late wiring described in the Presentation composition root (ISP-0007).
pub fn submit<D>(
    view_model: &mut CreateOrganizationViewModel,
    dispatcher: &D,
) -> SubmissionStatus
where
    D: CreateOrganizationDispatcher,
{
    invoke_create_organization(view_model, dispatcher);
    view_model.status().clone()
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn form_title_and_labels_are_stable() {
        assert_eq!(form_title(), "Create Organization");
        assert_eq!(name_field_label(), "Organization Name");
        assert_eq!(organization_type_field_label(), "Organization Type");
        assert_eq!(submit_button_label(), "Create Organization");
    }

    #[test]
    fn render_form_returns_bindable_field_rows() {
        let vm = view_model_with_inputs("ForgeOS", "foundation");

        let fields = render_form(&vm);
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].label(), "Organization Name");
        assert_eq!(fields[0].value(), "ForgeOS");
        assert_eq!(fields[1].label(), "Organization Type");
        assert_eq!(fields[1].value(), "foundation");
    }

    #[test]
    fn render_response_maps_response_dto_to_display_rows() {
        let mut vm = view_model_with_inputs("ForgeOS", "foundation");
        vm.set_response(sample_response());

        let rows = render_response(&vm).expect("response should be rendered");
        assert_eq!(rows.len(), 5);
        assert_eq!(rows[0].label(), "Organization ID");
        assert_eq!(rows[0].value(), "550e8400-e29b-41d4-a716-446655440000");
        assert_eq!(rows[1].label(), "Name");
        assert_eq!(rows[1].value(), "ForgeOS");
        assert_eq!(rows[2].label(), "Organization Type");
        assert_eq!(rows[2].value(), "foundation");
        assert_eq!(rows[3].label(), "Status");
        assert_eq!(rows[3].value(), "Active");
        assert_eq!(rows[4].label(), "Version");
        assert_eq!(rows[4].value(), "1");
    }

    #[test]
    fn render_response_returns_none_without_response() {
        let vm = view_model_with_inputs("ForgeOS", "foundation");
        assert!(render_response(&vm).is_none());
    }

    #[test]
    fn render_error_maps_error_dto_to_display_row() {
        let mut vm = view_model_with_inputs("ForgeOS", "foundation");
        let error = CreateOrganizationErrorDto {
            code: "ORGANIZATION_ALREADY_EXISTS".to_string(),
            message: "An organization already exists for this ForgeOS instance".to_string(),
        };
        vm.set_error(error);

        let row = render_error(&vm).expect("error should be rendered");
        assert_eq!(row.label(), "ORGANIZATION_ALREADY_EXISTS");
        assert_eq!(
            row.value(),
            "An organization already exists for this ForgeOS instance"
        );
    }

    #[test]
    fn render_error_returns_none_without_error() {
        let vm = view_model_with_inputs("ForgeOS", "foundation");
        assert!(render_error(&vm).is_none());
    }

    #[test]
    fn submit_dispatches_command_and_reports_success_status() {
        let mut vm = view_model_with_inputs("ForgeOS", "foundation");
        let response = sample_response();

        let dispatcher = |_request: &CreateOrganizationRequest| Ok(response.clone());

        let status = submit(&mut vm, &dispatcher);

        assert_eq!(status, SubmissionStatus::Success);
        assert_eq!(vm.status(), &SubmissionStatus::Success);
        assert!(vm.response().is_some());
        assert!(vm.error().is_none());
    }

    #[test]
    fn submit_dispatches_command_and_reports_error_status() {
        let mut vm = view_model_with_inputs("ForgeOS", "foundation");
        let error = CreateOrganizationErrorDto {
            code: "ORGANIZATION_ALREADY_EXISTS".to_string(),
            message: "An organization already exists for this ForgeOS instance".to_string(),
        };

        let dispatcher = |_request: &CreateOrganizationRequest| Err(error.clone());

        let status = submit(&mut vm, &dispatcher);

        assert_eq!(status, SubmissionStatus::Error);
        assert_eq!(vm.status(), &SubmissionStatus::Error);
        assert!(vm.error().is_some());
        assert!(vm.response().is_none());
    }
}
