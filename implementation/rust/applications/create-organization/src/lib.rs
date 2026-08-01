//! Create Organization application ownership boundary.
//!
//! Implements the Create Organization use case (Milestone 1.6): the
//! `CreateOrganizationCommand`, the `CreateOrganization` application service,
//! application-level errors, and repository interaction through the
//! Domain-owned `OrganizationRepository` trait.
//!
//! This crate coordinates use case execution without implementing business
//! rules. Business logic remains exclusively in the Domain Layer.

#![forbid(unsafe_code)]

mod command;
mod errors;
mod service;

pub use command::CreateOrganizationCommand;
pub use errors::{CreateOrganizationError, OrganizationField};
pub use service::CreateOrganization;