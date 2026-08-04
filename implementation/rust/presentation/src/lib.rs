//! Organization Presentation Layer (MILESTONE-001.9).
//!
//! The Presentation Layer renders the Create Organization user interface,
//! collects untrusted user input for organization name and organization type,
//! dispatches the `createOrganization` Tauri command through the IPC boundary,
//! and displays the `CreateOrganizationResponse` DTO on success or the
//! `CreateOrganizationErrorDto` on failure.
//!
//! ## Architecture
//!
//! ```text
//! Presentation (this crate)
//!     │
//!     ▼
//! Platform (forgeos-desktop-platform)
//!     │
//!     ▼
//! Application Services
//!     │
//!     ▼
//! Domains → Infrastructure
//! ```
//!
//! The Presentation Layer contains **no business logic**, **no domain logic**,
//! **no persistence logic**, **no workflow rules**, and **no governance rules**
//! (ARCH-0002, ARCH-0003, TDS-0001).
//!
//! This crate depends **only** on the Platform crate
//! (`forgeos-desktop-platform`) for the `createOrganization` Tauri command and
//! the IPC DTO types. It does not depend on Domain, Infrastructure, or
//! Application crates (ARCH-0003, MILESTONE-001.9).
//!
//! Only DTOs cross the IPC boundary. Domain entities never cross TB-2
//! (ARCH-0001, TDR-0002, TDR-0004).
//!
//! The frontend framework selection remains deferred per `TDR-0002` Future
//! Considerations. The UI module provides a framework-neutral composition and
//! rendering contract.

#![forbid(unsafe_code)]

mod composition;
mod errors;
mod ipc;
mod ui;
mod view_model;

pub use composition::PresentationCompositionRoot;
pub use errors::{PresentationError, PresentationField};
pub use ipc::{
    build_request, invoke_create_organization, validate_view_model, CreateOrganizationDispatcher,
};
pub use ui::{
    form_title, name_field_label, organization_type_field_label, render_error, render_form,
    render_response, submit, submit_button_label,
};
pub use view_model::{CreateOrganizationViewModel, SubmissionStatus};