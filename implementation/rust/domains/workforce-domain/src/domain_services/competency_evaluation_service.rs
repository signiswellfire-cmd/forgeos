//! CompetencyEvaluationService stub.
//!
//! This is a minimal stub implementation for the foundation milestone.

use crate::{Competency, CompetencyLevel, ProfessionalId, SkillIdentifier};

/// CompetencyEvaluationService stub.
///
/// Evaluates professional competency (TDS-0002).
#[derive(Debug, Clone, Default)]
pub struct CompetencyEvaluationService;

impl CompetencyEvaluationService {
    /// Creates a new CompetencyEvaluationService stub.
    pub fn new() -> Self {
        Self
    }

    /// Evaluates competency for a professional (stub implementation).
    pub fn evaluate(
        &self,
        _professional_id: ProfessionalId,
        _skill_id: SkillIdentifier,
        _level: CompetencyLevel,
    ) -> Competency {
        // Stub implementation - returns a placeholder competency
        Competency::new(_professional_id, _skill_id, _level)
    }
}