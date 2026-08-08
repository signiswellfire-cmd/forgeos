//! DelegatedAuthority entity stub.
//!
//! This module provides a minimal DelegatedAuthority entity implementation.
//! Full implementation will be completed in future milestones.

use crate::{AuthorityLevel, GovernanceResult};

/// DelegatedAuthority - Represents delegated governance authority
#[derive(Debug, Clone)]
pub struct DelegatedAuthority {
    delegation_id: String,
    delegatee: String,
    authority_level: AuthorityLevel,
}

impl DelegatedAuthority {
    pub fn new(
        delegation_id: impl Into<String>,
        delegatee: impl Into<String>,
        authority_level: AuthorityLevel,
    ) -> Self {
        Self {
            delegation_id: delegation_id.into(),
            delegatee: delegatee.into(),
            authority_level,
        }
    }

    pub fn delegation_id(&self) -> &str {
        &self.delegation_id
    }

    pub fn delegatee(&self) -> &str {
        &self.delegatee
    }

    pub fn authority_level(&self) -> AuthorityLevel {
        self.authority_level
    }
}