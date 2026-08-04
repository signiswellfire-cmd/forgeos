//! Organization Infrastructure Layer.
//!
//! This crate provides the SQLite persistence implementation and event dispatch
//! for the Organization Domain. It implements the Domain-owned `OrganizationRepository`
//! trait using SQLx and SQLite, and the `EventPublisher` trait for in-process
//! event coordination.
//!
//! ## Architecture
//!
//! ```text
//! Application Layer
//!        ↓
//! Domain Layer (OrganizationRepository trait, EventPublisher trait)
//!        ↓
//! Infrastructure Layer (this crate)
//!        ↓
//! SQLite via SQLx
//! ```
//!
//! ## Components
//!
//! - `repository` — SQLite implementation of `OrganizationRepository`
//! - `event_publisher` — In-memory implementation of `EventPublisher` trait
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
pub mod event_publisher;
pub mod repository;

// Re-export the main types for convenience
pub use event_publisher::InMemoryEventPublisher;
pub use repository::SqliteOrganizationRepository;
