//! ApprovalRecord entity stub.
//!
//! This module provides a minimal ApprovalRecord entity implementation.
//! Full implementation will be completed in future milestones.

use crate::{ApprovalStatus, AuthorityLevel};

/// ApprovalRecord - Represents an approval record for a decision
#[derive(Debug, Clone)]
pub struct ApprovalRecord {
    approver: String,
    authority_level: AuthorityLevel,
    status: ApprovalStatus,
    comments: String,
}

impl ApprovalRecord {
    pub fn new(
        approver: impl Into<String>,
        authority_level: AuthorityLevel,
        status: ApprovalStatus,
        comments: impl Into<String>,
    ) -> Self {
        Self {
            approver: approver.into(),
            authority_level,
            status,
            comments: comments.into(),
        }
    }

    pub fn approver(&self) -> &str {
        &self.approver
    }

    pub fn status(&self) -> ApprovalStatus {
        self.status
    }
}