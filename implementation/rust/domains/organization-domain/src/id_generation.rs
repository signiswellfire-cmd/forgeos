//! Domain-owned identity generation for `OrganizationId` (TDR-0006).
//!
//! The Organization Domain owns the `OrganizationIdGenerator` contract.
//! The aggregate requests an identity from an injected generator during
//! creation; production uses UUID v4 (`DefaultOrganizationIdGenerator`),
//! while tests inject a deterministic generator.

use crate::value_objects::OrganizationId;

/// Domain-owned contract for generating `OrganizationId` values.
///
/// The generator is injected into Organization creation so that tests can
/// supply deterministic identities without exercising random UUID generation
/// directly.
pub trait OrganizationIdGenerator {
    /// Generates a new, immutable, globally unique `OrganizationId`.
    fn generate(&self) -> OrganizationId;
}

/// Default generator: produces a UUID v4 `OrganizationId` (TDR-0006).
///
/// Uses 122 random bits from a cryptographically secure random source.
/// The result is non-sequential and carries no ordering semantics.
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultOrganizationIdGenerator;

impl OrganizationIdGenerator for DefaultOrganizationIdGenerator {
    fn generate(&self) -> OrganizationId {
        let uuid = uuid::Uuid::new_v4();
        OrganizationId::from(uuid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_generator_produces_distinct_ids() {
        let generator = DefaultOrganizationIdGenerator;
        let first = generator.generate();
        let second = generator.generate();
        assert_ne!(first, second);
    }

    #[test]
    fn default_generator_produces_v4_uuid_text() {
        let generator = DefaultOrganizationIdGenerator;
        let id = generator.generate();
        let text = id.as_str();
        // The 13th hex nibble is the version nibble; v4 => "4".
        assert_eq!(text.as_bytes()[14], b'4');
        // Variant nibble is '8', '9', 'a', or 'b'.
        let variant = text.as_bytes()[19];
        assert!(
            matches!(variant, b'8' | b'9' | b'a' | b'b'),
            "unexpected variant nibble: {variant}"
        );
    }
}