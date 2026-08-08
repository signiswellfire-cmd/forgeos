//! Workforce domain events.
//!
//! Domain events represent significant business facts that have occurred in the
//! Workforce bounded context. Events are immutable and can be published to
//! interested parties after successful transaction commit.

use crate::{
    CapabilityReference, CompetencyLevel, ProfessionalId, SkillIdentifier, TeamId, WorkforceStatus,
};

/// Event publisher trait for in-process event dispatch.
///
/// This trait defines the contract for publishing domain events within the
/// Workforce bounded context. The implementation resides in the infrastructure
/// layer (ISP-0005).
pub trait EventPublisher {
    /// Publishes a domain event.
    fn publish(&mut self, event: WorkforceDomainEvent);

    /// Drains all pending events.
    fn drain_events(&mut self) -> Vec<WorkforceDomainEvent>;
}

/// Workforce domain events.
///
/// Each variant represents a significant business fact that has occurred in the
/// Workforce bounded context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkforceDomainEvent {
    /// A professional was created.
    ProfessionalCreated(ProfessionalCreated),
    /// A professional was updated.
    ProfessionalUpdated(ProfessionalUpdated),
    /// A team was created.
    TeamCreated(TeamCreated),
    /// Team membership changed.
    TeamMembershipChanged(TeamMembershipChanged),
    /// A skill was registered.
    SkillRegistered(SkillRegistered),
    /// A competency was evaluated.
    CompetencyEvaluated(CompetencyEvaluated),
    /// A capability was assigned.
    CapabilityAssigned(CapabilityAssigned),
}

// ProfessionalCreated - Published when a professional is registered

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfessionalCreated {
    professional_id: ProfessionalId,
    name: String,
    status: WorkforceStatus,
}

impl ProfessionalCreated {
    pub fn new(professional_id: ProfessionalId, name: String, status: WorkforceStatus) -> Self {
        Self {
            professional_id,
            name,
            status,
        }
    }

    pub fn professional_id(&self) -> ProfessionalId {
        self.professional_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn status(&self) -> WorkforceStatus {
        self.status
    }
}

// ProfessionalUpdated - Published when a professional is modified

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfessionalUpdated {
    professional_id: ProfessionalId,
    name: String,
    status: WorkforceStatus,
}

impl ProfessionalUpdated {
    pub fn new(professional_id: ProfessionalId, name: String, status: WorkforceStatus) -> Self {
        Self {
            professional_id,
            name,
            status,
        }
    }

    pub fn professional_id(&self) -> ProfessionalId {
        self.professional_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn status(&self) -> WorkforceStatus {
        self.status
    }
}

// TeamCreated - Published when a team is formed

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamCreated {
    team_id: TeamId,
    name: String,
    mission_id: Option<String>,
}

impl TeamCreated {
    pub fn new(team_id: TeamId, name: String, mission_id: Option<String>) -> Self {
        Self {
            team_id,
            name,
            mission_id,
        }
    }

    pub fn team_id(&self) -> TeamId {
        self.team_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn mission_id(&self) -> Option<&str> {
        self.mission_id.as_deref()
    }
}

// TeamMembershipChanged - Published when team membership changes

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamMembershipChanged {
    team_id: TeamId,
    professional_id: ProfessionalId,
    action: MembershipAction,
}

impl TeamMembershipChanged {
    pub fn new(
        team_id: TeamId,
        professional_id: ProfessionalId,
        action: MembershipAction,
    ) -> Self {
        Self {
            team_id,
            professional_id,
            action,
        }
    }

    pub fn team_id(&self) -> TeamId {
        self.team_id
    }

    pub fn professional_id(&self) -> ProfessionalId {
        self.professional_id
    }

    pub fn action(&self) -> MembershipAction {
        self.action
    }
}

/// Team membership action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MembershipAction {
    Added,
    Removed,
}

// SkillRegistered - Published when a skill is registered

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillRegistered {
    skill_id: SkillIdentifier,
    name: String,
    description: String,
}

impl SkillRegistered {
    pub fn new(skill_id: SkillIdentifier, name: String, description: String) -> Self {
        Self {
            skill_id,
            name,
            description,
        }
    }

    pub fn skill_id(&self) -> SkillIdentifier {
        self.skill_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

// CompetencyEvaluated - Published when competency is evaluated

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompetencyEvaluated {
    professional_id: ProfessionalId,
    skill_id: SkillIdentifier,
    level: CompetencyLevel,
}

impl CompetencyEvaluated {
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

// CapabilityAssigned - Published when capability is assigned

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityAssigned {
    professional_id: ProfessionalId,
    capability_reference: CapabilityReference,
}

impl CapabilityAssigned {
    pub fn new(
        professional_id: ProfessionalId,
        capability_reference: CapabilityReference,
    ) -> Self {
        Self {
            professional_id,
            capability_reference,
        }
    }

    pub fn professional_id(&self) -> ProfessionalId {
        self.professional_id
    }

    pub fn capability_reference(&self) -> &CapabilityReference {
        &self.capability_reference
    }
}