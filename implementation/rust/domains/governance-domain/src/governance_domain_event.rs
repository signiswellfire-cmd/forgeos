//! Governance domain events.
//!
//! Domain events represent significant occurrences within the Governance
//! bounded context. They are immutable facts that have already happened
//! and are used to communicate state changes to other bounded contexts.

use std::fmt;

// GovernanceDomainEvent - Enumeration of all governance domain events

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GovernanceDomainEvent {
    DecisionApproved(DecisionApproved),
    DecisionRejected(DecisionRejected),
    PolicyPublished(PolicyPublished),
    PolicyRetired(PolicyRetired),
    AuthorityDelegated(AuthorityDelegated),
    AuthorityRevoked(AuthorityRevoked),
}

impl fmt::Display for GovernanceDomainEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DecisionApproved(event) => write!(f, "{}", event),
            Self::DecisionRejected(event) => write!(f, "{}", event),
            Self::PolicyPublished(event) => write!(f, "{}", event),
            Self::PolicyRetired(event) => write!(f, "{}", event),
            Self::AuthorityDelegated(event) => write!(f, "{}", event),
            Self::AuthorityRevoked(event) => write!(f, "{}", event),
        }
    }
}

// DecisionApproved - Published when a decision is approved

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionApproved {
    pub decision_id: DecisionId,
    pub approver: String,
    pub approved_at: chrono::DateTime<chrono::Utc>,
}

impl DecisionApproved {
    pub fn new(decision_id: DecisionId, approver: impl Into<String>) -> Self {
        Self {
            decision_id,
            approver: approver.into(),
            approved_at: chrono::Utc::now(),
        }
    }
}

impl fmt::Display for DecisionApproved {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Decision {} approved by {} at {}",
            self.decision_id, self.approver, self.approved_at
        )
    }
}

// DecisionRejected - Published when a decision is rejected

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionRejected {
    pub decision_id: DecisionId,
    pub rejector: String,
    pub reason: String,
    pub rejected_at: chrono::DateTime<chrono::Utc>,
}

impl DecisionRejected {
    pub fn new(
        decision_id: DecisionId,
        rejector: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            decision_id,
            rejector: rejector.into(),
            reason: reason.into(),
            rejected_at: chrono::Utc::now(),
        }
    }
}

impl fmt::Display for DecisionRejected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Decision {} rejected by {}: {}",
            self.decision_id, self.rejector, self.reason
        )
    }
}

// PolicyPublished - Published when a policy is published

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyPublished {
    pub policy_id: PolicyId,
    pub publisher: String,
    pub published_at: chrono::DateTime<chrono::Utc>,
}

impl PolicyPublished {
    pub fn new(policy_id: PolicyId, publisher: impl Into<String>) -> Self {
        Self {
            policy_id,
            publisher: publisher.into(),
            published_at: chrono::Utc::now(),
        }
    }
}

impl fmt::Display for PolicyPublished {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Policy {} published by {} at {}",
            self.policy_id, self.publisher, self.published_at
        )
    }
}

// PolicyRetired - Published when a policy is retired

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyRetired {
    pub policy_id: PolicyId,
    pub retired_by: String,
    pub retired_at: chrono::DateTime<chrono::Utc>,
}

impl PolicyRetired {
    pub fn new(policy_id: PolicyId, retired_by: impl Into<String>) -> Self {
        Self {
            policy_id,
            retired_by: retired_by.into(),
            retired_at: chrono::Utc::now(),
        }
    }
}

impl fmt::Display for PolicyRetired {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Policy {} retired by {} at {}",
            self.policy_id, self.retired_by, self.retired_at
        )
    }
}

// AuthorityDelegated - Published when authority is delegated

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityDelegated {
    pub delegation_id: String,
    pub delegator: String,
    pub delegatee: String,
    pub authority_level: u8,
    pub delegated_at: chrono::DateTime<chrono::Utc>,
}

impl AuthorityDelegated {
    pub fn new(
        delegation_id: impl Into<String>,
        delegator: impl Into<String>,
        delegatee: impl Into<String>,
        authority_level: u8,
    ) -> Self {
        Self {
            delegation_id: delegation_id.into(),
            delegator: delegator.into(),
            delegatee: delegatee.into(),
            authority_level,
            delegated_at: chrono::Utc::now(),
        }
    }
}

impl fmt::Display for AuthorityDelegated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Authority delegated from {} to {} (level {}) at {}",
            self.delegator, self.delegatee, self.authority_level, self.delegated_at
        )
    }
}

// AuthorityRevoked - Published when delegated authority is revoked

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityRevoked {
    pub delegation_id: String,
    pub revoked_by: String,
    pub revoked_at: chrono::DateTime<chrono::Utc>,
}

impl AuthorityRevoked {
    pub fn new(delegation_id: impl Into<String>, revoked_by: impl Into<String>) -> Self {
        Self {
            delegation_id: delegation_id.into(),
            revoked_by: revoked_by.into(),
            revoked_at: chrono::Utc::now(),
        }
    }
}

impl fmt::Display for AuthorityRevoked {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Authority {} revoked by {} at {}",
            self.delegation_id, self.revoked_by, self.revoked_at
        )
    }
}

// Re-export value objects for convenience
pub use crate::value_objects::{DecisionId, PolicyId};