//! View model for the Create Organization form (MILESTONE-001.9).
//!
//! The view model owns transient, framework-neutral UI state for the Create
//! Organization form: form input state (name, organization type), submission
//! status, response display state, and error display state.
//!
//! The view model contains **no business logic** and **no domain logic**. It is
//! presentation-only state, per ARCH-0002 and TDR-0002. Structural validation
//! of form inputs at TB-1 is provided by the `ipc` module; the view model
//! itself only stores and exposes its state.

use forgeos_desktop_platform::{CreateOrganizationErrorDto, CreateOrganizationResponse};

/// Submission status for the Create Organization form (MILESTONE-001.9).
///
/// Represents the lifecycle of a form submission:
///
/// - [`SubmissionStatus::Idle`] — the form is ready for input and no
///   submission has been attempted.
/// - [`SubmissionStatus::Submitting`] — the `createOrganization` command is
///   in flight across the IPC boundary.
/// - [`SubmissionStatus::Success`] — the command returned a
///   `CreateOrganizationResponse` DTO.
/// - [`SubmissionStatus::Error`] — the command returned a
///   `CreateOrganizationErrorDto` DTO.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SubmissionStatus {
    /// The form is ready for input; no submission has been attempted.
    #[default]
    Idle,
    /// The `createOrganization` command is in flight.
    Submitting,
    /// The command returned a `CreateOrganizationResponse` DTO.
    Success,
    /// The command returned a `CreateOrganizationErrorDto` DTO.
    Error,
}

/// Transient UI state for the Create Organization form (ARCH-0002, TDR-0002).
///
/// Owned exclusively by the Presentation Layer. Holds the user's form input
/// (organization name, organization type), the submission status, and the
/// response or error DTO returned by the backend when a submission completes.
///
/// Field names declared in camelCase for IPC request construction; the `ipc`
/// module builds the immutable `CreateOrganizationRequest` DTO from this state.
///
/// The view model contains no business behavior, no domain entities, and no
/// persistence logic (MILESTONE-001.9).
#[derive(Debug, Clone, Default)]
pub struct CreateOrganizationViewModel {
    name: String,
    organization_type: String,
    status: SubmissionStatus,
    response: Option<CreateOrganizationResponse>,
    error: Option<CreateOrganizationErrorDto>,
}

impl CreateOrganizationViewModel {
    /// Creates a new view model with default (empty) form state
    /// (MILESTONE-001.9).
    ///
    /// The form starts with an empty name, an empty organization type, and
    /// [`SubmissionStatus::Idle`]. No response or error is displayed.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the current organization name input (MILESTONE-001.9).
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Updates the organization name input (MILESTONE-001.9).
    ///
    /// The input is stored as supplied by the user. No case folding, display
    /// transformation, or length policy is applied at the view model layer.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Returns the current organization type input (MILESTONE-001.9).
    pub fn organization_type(&self) -> &str {
        &self.organization_type
    }

    /// Updates the organization type input (MILESTONE-001.9).
    ///
    /// The input is stored as supplied by the user. No taxonomy or
    /// normalization policy is applied at the view model layer.
    pub fn set_organization_type(&mut self, organization_type: String) {
        self.organization_type = organization_type;
    }

    /// Returns the current submission status (MILESTONE-001.9).
    pub fn status(&self) -> &SubmissionStatus {
        &self.status
    }

    /// Returns the response DTO if creation succeeded (TDR-0004).
    pub fn response(&self) -> Option<&CreateOrganizationResponse> {
        self.response.as_ref()
    }

    /// Returns the error DTO if creation failed (TDR-0004, ISP-0008).
    pub fn error(&self) -> Option<&CreateOrganizationErrorDto> {
        self.error.as_ref()
    }

    pub(crate) fn set_status(&mut self, status: SubmissionStatus) {
        self.status = status;
    }

    pub(crate) fn set_response(&mut self, response: CreateOrganizationResponse) {
        self.response = Some(response);
        self.error = None;
        self.status = SubmissionStatus::Success;
    }

    pub(crate) fn set_error(&mut self, error: CreateOrganizationErrorDto) {
        self.error = Some(error);
        self.response = None;
        self.status = SubmissionStatus::Error;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_response() -> CreateOrganizationResponse {
        CreateOrganizationResponse {
            organization_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
            name: "ForgeOS".to_string(),
            organization_type: "foundation".to_string(),
            status: "Active".to_string(),
            version: 1,
        }
    }

    fn sample_error() -> CreateOrganizationErrorDto {
        CreateOrganizationErrorDto {
            code: "VALIDATION_ERROR".to_string(),
            message: "Organization name is invalid".to_string(),
        }
    }

    #[test]
    fn new_view_model_starts_idle_with_empty_inputs() {
        let vm = CreateOrganizationViewModel::new();

        assert_eq!(vm.name(), "");
        assert_eq!(vm.organization_type(), "");
        assert_eq!(vm.status(), &SubmissionStatus::Idle);
        assert!(vm.response().is_none());
        assert!(vm.error().is_none());
    }

    #[test]
    fn set_name_updates_name_input() {
        let mut vm = CreateOrganizationViewModel::new();
        vm.set_name("ForgeOS".to_string());

        assert_eq!(vm.name(), "ForgeOS");
    }

    #[test]
    fn set_organization_type_updates_organization_type_input() {
        let mut vm = CreateOrganizationViewModel::new();
        vm.set_organization_type("foundation".to_string());

        assert_eq!(vm.organization_type(), "foundation");
    }

    #[test]
    fn status_transitions_to_success_with_response() {
        let mut vm = CreateOrganizationViewModel::new();
        let response = sample_response();

        vm.set_status(SubmissionStatus::Submitting);
        assert_eq!(vm.status(), &SubmissionStatus::Submitting);

        vm.set_response(response);
        assert_eq!(vm.status(), &SubmissionStatus::Success);
        assert!(vm.response().is_some());
        assert!(vm.error().is_none());
    }

    #[test]
    fn status_transitions_to_error_with_error_dto() {
        let mut vm = CreateOrganizationViewModel::new();
        let error = sample_error();

        vm.set_status(SubmissionStatus::Submitting);
        assert_eq!(vm.status(), &SubmissionStatus::Submitting);

        vm.set_error(error);
        assert_eq!(vm.status(), &SubmissionStatus::Error);
        assert!(vm.error().is_some());
        assert!(vm.response().is_none());
    }

    #[test]
    fn response_dto_field_access_is_available() {
        let mut vm = CreateOrganizationViewModel::new();
        vm.set_response(sample_response());

        let response = vm.response().expect("response should be present");
        assert_eq!(response.organization_id, "550e8400-e29b-41d4-a716-446655440000");
        assert_eq!(response.name, "ForgeOS");
        assert_eq!(response.organization_type, "foundation");
        assert_eq!(response.status, "Active");
        assert_eq!(response.version, 1);
    }

    #[test]
    fn error_dto_field_access_is_available() {
        let mut vm = CreateOrganizationViewModel::new();
        vm.set_error(sample_error());

        let error = vm.error().expect("error should be present");
        assert_eq!(error.code, "VALIDATION_ERROR");
        assert_eq!(error.message, "Organization name is invalid");
    }

    #[test]
    fn success_clears_previous_error() {
        let mut vm = CreateOrganizationViewModel::new();
        vm.set_error(sample_error());
        vm.set_response(sample_response());

        assert_eq!(vm.status(), &SubmissionStatus::Success);
        assert!(vm.error().is_none());
        assert!(vm.response().is_some());
    }

    #[test]
    fn error_clears_previous_response() {
        let mut vm = CreateOrganizationViewModel::new();
        vm.set_response(sample_response());
        vm.set_error(sample_error());

        assert_eq!(vm.status(), &SubmissionStatus::Error);
        assert!(vm.response().is_none());
        assert!(vm.error().is_some());
    }
}