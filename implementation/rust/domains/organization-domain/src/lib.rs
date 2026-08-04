//! Organization Domain ownership boundary.
//!
//! Implements the Organization Domain Foundation (Milestone 1.5): the
//! Organization aggregate, its approved value objects, domain errors, the
//! `OrganizationCreated` domain event, the Domain-owned repository contract,
//! and the identity-generation abstraction (TDR-0006).
//!
//! This crate is independent of persistence, application, platform, and
//! presentation concerns. It depends only on the Rust standard library and
//! the approved `uuid` dependency.

#![forbid(unsafe_code)]

mod errors;
mod id_generation;
mod org_domain_event;
mod organization;
pub mod organization_created;
mod organization_repository;
mod value_objects;

pub use errors::{OrganizationError, OrganizationField};
pub use id_generation::{DefaultOrganizationIdGenerator, OrganizationIdGenerator};
pub use org_domain_event::{EventPublisher, OrganizationDomainEvent};
pub use organization::Organization;
pub use organization_created::OrganizationCreated;
pub use organization_repository::OrganizationRepository;
pub use value_objects::{
    OrganizationId, OrganizationName, OrganizationStatus, OrganizationType, OrganizationVersion,
};
