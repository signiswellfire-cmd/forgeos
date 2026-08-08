//! Governance aggregate root.
//!
//! The Governance aggregate is the authoritative root of the Governance bounded
//! context. It owns governance identity, authority level, and scope (TDS-0002, RFC-0007).
//!
//! This milestone implements only the foundation creation behavior.

use crate::{AuthorityLevel, GovernanceError, GovernanceResult, GovernanceScope};

/// The `Governance` aggregate root.
///
/// Owns identity, authority level, and scope. Created governance starts
/// with an aggregate-generated `DecisionId`, the supplied authority level
/// and scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Governance {
    id: crate::DecisionId,
    authority_level: AuthorityLevel,
    scope: GovernanceScope,
    events: Vec<crate::GovernanceDomainEvent>,
}

impl Governance {
    /// Creates a Governance aggregate from validated domain values.
    ///
    /// The aggregate:
    /// 1. uses the supplied identity
    /// 2. establishes the supplied authority level and scope
    /// 3. records the initial governance creation event
    pub fn new(
        id: crate::DecisionId,
        authority_level: AuthorityLevel,
        scope: GovernanceScope,
    ) -> GovernanceResult<Self> {
        Ok(Self {
            id,
            authority_level,
            scope,
            events: Vec::new(),
        })
    }

    /// The immutable, aggregate-generated identity.
    pub fn id(&self) -> crate::DecisionId {
        self.id
    }

    /// The authority level established at creation.
    pub fn authority_level(&self) -> AuthorityLevel {
        self.authority_level
    }

    /// The scope established at creation.
    pub fn scope(&self) -> &str {
        self.scope.as_str()
    }

    /// Collects the domain events recorded since the last drain.
    pub fn take_events(&mut self) -> Vec<crate::GovernanceDomainEvent> {
        core::mem::take(&mut self.events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_creation_produces_governance_with_supplied_values() {
        let id = crate::DecisionId::new();
        let authority_level = AuthorityLevel::Level1Founder;
        let scope = GovernanceScope::new("test scope").unwrap();

        let governance = Governance::new(id, authority_level, scope).unwrap();

        assert_eq!(governance.id(), id);
        assert_eq!(governance.authority_level(), AuthorityLevel::Level1Founder);
        assert_eq!(governance.scope(), "test scope");
    }

    #[test]
    fn empty_scope_is_rejected() {
        let id = crate::DecisionId::new();
        let authority_level = AuthorityLevel::Level1Founder;
        let result = GovernanceScope::new("");
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GovernanceError::EmptyGovernanceScope));
    }
}