//! Policy entity stub.
//!
//! This module provides a minimal Policy entity implementation.
//! Full implementation will be completed in future milestones.

use crate::{ApprovalStatus, PolicyId};

/// Policy - Represents a governance policy
#[derive(Debug, Clone)]
pub struct Policy {
    id: PolicyId,
    title: String,
    content: String,
    version: u32,
    status: ApprovalStatus,
}

impl Policy {
    pub fn new(
        id: PolicyId,
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            content: content.into(),
            version: 1,
            status: ApprovalStatus::Proposed,
        }
    }

    pub fn id(&self) -> PolicyId {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn status(&self) -> ApprovalStatus {
        self.status
    }
}