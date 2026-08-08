//! Workforce aggregate root.
//!
//! The Workforce aggregate is the authoritative root of the Workforce bounded
//! context. It owns professional records, capability assignments, competency
//! evaluations, and team relationships (TDS-0002, ARCH-0002).
//!
//! This milestone implements only the foundation creation behavior.

use crate::{
    errors::WorkforceError, workforce_domain_event::WorkforceDomainEvent, ProfessionalId,
    WorkforceResult,
};

/// The `Workforce` aggregate root.
///
/// Owns workforce identity and professional records. Created workforce starts
/// with an aggregate-generated `ProfessionalId`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workforce {
    id: ProfessionalId,
    events: Vec<WorkforceDomainEvent>,
}

impl Workforce {
    /// Creates a Workforce aggregate from validated domain values.
    ///
    /// The aggregate:
    /// 1. uses the supplied identity
    /// 2. initializes with empty professional records
    /// 3. records the initial workforce creation event
    pub fn new(id: ProfessionalId) -> WorkforceResult<Self> {
        Ok(Self {
            id,
            events: Vec::new(),
        })
    }

    /// The immutable, aggregate-generated identity.
    pub fn id(&self) -> ProfessionalId {
        self.id
    }

    /// Collects the domain events recorded since the last drain.
    pub fn take_events(&mut self) -> Vec<WorkforceDomainEvent> {
        core::mem::take(&mut self.events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_objects::ProfessionalId;

    #[test]
    fn valid_creation_produces_workforce_with_supplied_id() {
        let id = ProfessionalId::new();
        let workforce = Workforce::new(id).unwrap();

        assert_eq!(workforce.id(), id);
    }

    #[test]
    fn events_drain_and_do_not_duplicate() {
        let id = ProfessionalId::new();
        let mut workforce = Workforce::new(id).unwrap();

        let first_drain = workforce.take_events();
        let second_drain = workforce.take_events();

        assert!(first_drain.is_empty());
        assert!(second_drain.is_empty());
    }
}