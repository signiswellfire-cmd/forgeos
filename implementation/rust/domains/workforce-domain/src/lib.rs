//! Workforce Domain ownership boundary.
//!
//! Implements the Workforce Domain Foundation (Milestone 4.0): the
//! Workforce aggregate, its approved value objects, domain errors, domain
//! events, the Domain-owned repository contract, and domain services.
//!
//! This crate is independent of persistence, application, platform, and
//! presentation concerns. It depends only on the Rust standard library and
//! the approved `uuid` and `thiserror` dependencies.

#![forbid(unsafe_code)]

mod errors;
mod professional;
mod team;
mod competency;
mod skill;
mod capability_assignment;
mod team_membership;
mod workforce_domain_event;
mod workforce_repository;
mod workforce;
pub mod domain_services;
mod value_objects;

pub use errors::WorkforceError;
pub use professional::Professional;
pub use team::Team;
pub use competency::Competency;
pub use skill::Skill;
pub use capability_assignment::CapabilityAssignment;
pub use team_membership::TeamMembership;
pub use workforce_domain_event::{EventPublisher, WorkforceDomainEvent};
pub use workforce::Workforce;
pub use workforce_repository::WorkforceRepository;
pub use value_objects::{
    ProfessionalId, TeamId, CompetencyLevel, SkillIdentifier, WorkforceStatus, CapabilityReference,
};

/// Result type for Workforce operations.
pub type WorkforceResult<T> = Result<T, WorkforceError>;
