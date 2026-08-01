//! Immutable `OrganizationCreated` domain event.
//!
//! A completed business fact published by the Organization bounded context.
//! It contains no workflow coordination or infrastructure behavior. Event
//! dispatch after successful transaction commit is governed by ISP-0005 and
//! ISP-0006; no consumer or dispatcher exists in this milestone.

use crate::value_objects::{
    OrganizationId, OrganizationName, OrganizationStatus, OrganizationType, OrganizationVersion,
};

/// Immutable record of a completed Organization creation.
///
/// Contains only stable creation facts approved by
/// MILESTONE-001-DOMAIN-DECISIONS:
///
/// * `organization_id`
/// * `name`
/// * `organization_type`
/// * `status` (`Active`)
/// * `version` (`1`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationCreated {
    organization_id: OrganizationId,
    name: OrganizationName,
    organization_type: OrganizationType,
    status: OrganizationStatus,
    version: OrganizationVersion,
}

impl OrganizationCreated {
    /// Constructs the completed creation event.
    #[doc(hidden)]
    pub fn new(
        organization_id: OrganizationId,
        name: OrganizationName,
        organization_type: OrganizationType,
        status: OrganizationStatus,
        version: OrganizationVersion,
    ) -> Self {
        Self {
            organization_id,
            name,
            organization_type,
            status,
            version,
        }
    }

    /// The immutable identity of the created Organization.
    pub fn organization_id(&self) -> OrganizationId {
        self.organization_id
    }

    /// The name established at creation.
    pub fn name(&self) -> &OrganizationName {
        &self.name
    }

    /// The type established at creation.
    pub fn organization_type(&self) -> &OrganizationType {
        &self.organization_type
    }

    /// The initial externally observable status.
    pub fn status(&self) -> OrganizationStatus {
        self.status
    }

    /// The initial aggregate version.
    pub fn version(&self) -> OrganizationVersion {
        self.version
    }
}