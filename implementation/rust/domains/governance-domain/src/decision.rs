//! Decision entity stub.
//!
//! This module provides a minimal Decision entity implementation.
//! Full implementation will be completed in future milestones.

use crate::{ApprovalStatus, DecisionId, GovernanceResult};

/// Decision - Represents a governance decision
#[derive(Debug, Clone)]
pub struct Decision {
    id: DecisionId,
    title: String,
    description: String,
    owner: String,
    status: ApprovalStatus,
}

impl Decision {
    pub fn new(
        id: DecisionId,
        title: impl Into<String>,
        description: impl Into<String>,
        owner: impl Into<String>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            description: description.into(),
            owner: owner.into(),
            status: ApprovalStatus::Proposed,
        }
    }

    pub fn id(&self) -> DecisionId {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn status(&self) -> ApprovalStatus {
        self.status
    }
}