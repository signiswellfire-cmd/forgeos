//! Desktop Platform ownership boundary (MILESTONE-001.8).
//!
//! The Platform Layer wires the approved Create Organization vertical slice
//! into the Tauri 2.x desktop runtime, exposing a single versioned Tauri
//! command (`createOrganization`) that accepts a request DTO, invokes the
//! approved Application Service, and returns a response DTO or error DTO.
//!
//! ## Architecture
//!
//! ```text
//! Platform (this crate)
//!     ↓
//! Application (forgeos-create-organization-application)
//!     ↓
//! Domain (forgeos-organization-domain)
//!
//! Platform (this crate)
//!     ↓
//! Infrastructure (forgeos-organization-infrastructure — composition only)
//! ```
//!
//! The Platform Layer provides runtime capabilities without acquiring
//! business responsibility (ARCH-0003, Dependency Contract — Platform).

#![forbid(unsafe_code)]

mod commands;
mod composition;
mod dtos;
mod errors;

pub use commands::createOrganization;
pub use composition::CompositionRoot;
pub use dtos::{CreateOrganizationRequest, CreateOrganizationResponse};
pub use errors::CreateOrganizationErrorDto;
