//! Competency entity stub.
//!
//! This is a minimal stub implementation for the foundation milestone.

use crate::{CompetencyLevel, ProfessionalId, SkillIdentifier, WorkforceError};

/// Competency entity stub.
///
/// Represents a measure of professional capability (TDS-0002, RFC-0028).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Competency {
    professional_id: ProfessionalId,
    skill_id: SkillIdentifier,
    level: CompetencyLevel,
}

impl Competency {
    /// Creates a new Competency entity stub.
    pub fn new(
        professional_id: ProfessionalId,
        skill_id: SkillIdentifier,
        level: CompetencyLevel,
    ) -> Self {
        Self {
            professional_id,
            skill_id,
            level,
        }
    }

    pub fn professional_id(&self) -> ProfessionalId {
        self.professional_id
    }

    pub fn skill_id(&self) -> SkillIdentifier {
        self.skill_id
    }

    pub fn level(&self) -> CompetencyLevel {
        self.level
    }
}