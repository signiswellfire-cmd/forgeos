//! Organization Infrastructure Layer.
//!
//! This crate provides the SQLite persistence implementation for the
//! Organization Domain. It implements the Domain-owned `OrganizationRepository`
//! trait using SQLx and SQLite.
//!
//! ## Architecture
//!
//! ```text
//! Application Layer
//!        ↓
//! Domain Layer (OrganizationRepository trait)
//!        ↓
//! Infrastructure Layer (this crate)
//!        ↓
//! SQLite via SQLx
//! ```
//!
//! ## Components
//!
//! - `repository` — SQLite implementation of `OrganizationRepository`
//! - `errors` — Infrastructure-level error types (translated to domain errors)
//! - `migrations` — Forward-only SQLx migrations
//!
//! ## Dependencies
//!
//! This crate depends on:
//! - `forgeos-organization-domain` — Domain contracts and types
//! - `sqlx` — SQLite driver and async runtime
//! - `tokio` — Async runtime
//! - `thiserror` — Error handling
//!
//! No Domain, Application, or Platform crate depends on this crate.

pub mod errors;
pub mod repository;

// Re-export the main repository type for convenience
pub use repository::SqliteOrganizationRepository;