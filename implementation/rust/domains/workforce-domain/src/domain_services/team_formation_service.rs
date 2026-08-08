//! TeamFormationService stub.
//!
//! This is a minimal stub implementation for the foundation milestone.

use crate::{ProfessionalId, Team, TeamId};

/// TeamFormationService stub.
///
/// Forms teams for missions (TDS-0002).
#[derive(Debug, Clone, Default)]
pub struct TeamFormationService;

impl TeamFormationService {
    /// Creates a new TeamFormationService stub.
    pub fn new() -> Self {
        Self
    }

    /// Forms a team for a mission (stub implementation).
    pub fn form_team(
        &self,
        _team_id: TeamId,
        _name: impl Into<String>,
        _professional_ids: Vec<ProfessionalId>,
    ) -> Team {
        // Stub implementation - returns a placeholder team
        Team::new(_team_id, _name).unwrap()
    }
}