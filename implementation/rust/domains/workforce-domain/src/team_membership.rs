//! TeamMembership entity stub.
//!
//! This is a minimal stub implementation for the foundation milestone.

use crate::{ProfessionalId, TeamId, WorkforceError};

/// TeamMembership entity stub.
///
/// Represents team membership governance (TDS-0002, ARCH-0002).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamMembership {
    team_id: TeamId,
    professional_id: ProfessionalId,
}

impl TeamMembership {
    /// Creates a new TeamMembership entity stub.
    pub fn new(team_id: TeamId, professional_id: ProfessionalId) -> Self {
        Self {
            team_id,
            professional_id,
        }
    }

    pub fn team_id(&self) -> TeamId {
        self.team_id
    }

    pub fn professional_id(&self) -> ProfessionalId {
        self.professional_id
    }
}