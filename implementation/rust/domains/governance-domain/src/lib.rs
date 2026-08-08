//! Governance Domain ownership boundary.
//!
//! Implements the Governance Domain Foundation (Milestone 3.0): the
//! Governance aggregate, its approved value objects, domain errors, the
//! Governance domain events, the Domain-owned repository contract,
//! and domain services (TDS-0002, RFC-0007).
//!
//! This crate is independent of persistence, application, platform, and
//! presentation concerns. It depends only on the Rust standard library and
//! the approved `uuid` dependency.

#![forbid(unsafe_code)]

mod errors;
mod governance;
pub mod governance_domain_event;
pub mod governance_repository;
mod governance_rule;
mod delegated_authority;
mod approval_record;
mod decision;
mod policy;
mod standard;
pub mod domain_services;
mod value_objects;

pub use errors::{GovernanceError, GovernanceResult};
pub use governance::Governance;
pub use governance_domain_event::{
    AuthorityDelegated, AuthorityRevoked, DecisionApproved, DecisionRejected, GovernanceDomainEvent,
    PolicyPublished, PolicyRetired,
};
pub use governance_repository::GovernanceRepository;
pub use value_objects::{
    ApprovalStatus, AuthorityLevel, DecisionId, GovernanceScope, PolicyId, StandardIdentifier,
};
