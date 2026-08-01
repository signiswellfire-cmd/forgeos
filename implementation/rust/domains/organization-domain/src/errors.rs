//! Domain-owned error model for the Organization bounded context.
//!
//! Per the approved error model (MILESTONE-001-DOMAIN-DECISIONS and ISP-0008),
//! the Organization Domain exposes only domain-level error categories:
//!
//! * validation errors for missing or whitespace-only required values;
//! * the `OrganizationAlreadyExists` business-rule failure;
//! * a structured `Unexpected` category for failures translated across a
//!   boundary.
//!
//! No SQL, IO, IPC, infrastructure, or platform error types are exposed here.

/// The field of the Create Organization contract that failed domain validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrganizationField {
    /// The `name` creation input.
    Name,
    /// The `organization_type` creation input.
    OrganizationType,
}

/// Domain-owned error for the Organization bounded context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrganizationError {
    /// A domain validation failure for a required creation input.
    ///
    /// Raised when the value is missing or contains no non-whitespace
    /// character. No persistence attempt occurs.
    Validation(OrganizationField),

    /// Business-rule failure: an Organization already exists for this
    /// ForgeOS instance (RFC-0004 exactly-one-Organization rule).
    ///
    /// No second aggregate is created.
    OrganizationAlreadyExists,

    /// A structured unexpected failure translated across a boundary.
    ///
    /// Used for failures outside the categories above; it is never silently
    /// suppressed and is never converted into a business rule.
    Unexpected(String),
}