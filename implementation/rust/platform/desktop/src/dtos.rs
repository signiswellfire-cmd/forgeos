//! IPC Data Transfer Objects for the Organization Platform Layer (TDR-0004).
//!
//! DTOs are immutable boundary types that cross the Tauri IPC boundary. They
//! are independent of domain entities and value objects — no domain object is
//! ever serialized across IPC (ARCH-0001 TB-2, TDR-0004).
//!
//! Field names use camelCase for IPC compatibility with the frontend.

use serde::{Deserialize, Serialize};

/// IPC request DTO for the `createOrganization` command (TDR-0004).
///
/// Carries the user-supplied organization name and type classification from
/// the frontend across the IPC boundary. Contains no business behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrganizationRequest {
    /// The desired organization name.
    pub name: String,

    /// The organization type classification.
    #[serde(rename = "organizationType")]
    pub organization_type: String,
}

/// IPC response DTO for the `createOrganization` command on success (TDR-0004).
///
/// Constructed from the application service return value (`organizationId`),
/// the request DTO fields (`name`, `organizationType`), and approved domain
/// defaults (`status = "Active"`, `version = 1` per MILESTONE-001-DOMAIN-DECISIONS).
/// Contains no business behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrganizationResponse {
    /// The created organization's UUID.
    #[serde(rename = "organizationId")]
    pub organization_id: String,

    /// The organization name.
    pub name: String,

    /// The organization type.
    #[serde(rename = "organizationType")]
    pub organization_type: String,

    /// The lifecycle status (`"Active"`).
    pub status: String,

    /// The aggregate version (`1`).
    pub version: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_dto_serializes_and_deserializes() {
        let request = CreateOrganizationRequest {
            name: "ForgeOS".to_string(),
            organization_type: "foundation".to_string(),
        };

        let json = serde_json::to_string(&request).expect("serialization should succeed");
        let deserialized: CreateOrganizationRequest =
            serde_json::from_str(&json).expect("deserialization should succeed");

        assert_eq!(deserialized.name, "ForgeOS");
        assert_eq!(deserialized.organization_type, "foundation");
    }

    #[test]
    fn request_dto_uses_camel_case_field_names() {
        let request = CreateOrganizationRequest {
            name: "ForgeOS".to_string(),
            organization_type: "foundation".to_string(),
        };

        let json = serde_json::to_string(&request).expect("serialization should succeed");

        // The JSON must use camelCase "organizationType" for IPC compatibility.
        assert!(
            json.contains("\"organizationType\""),
            "expected camelCase field name 'organizationType' in JSON: {json}"
        );
        assert!(
            !json.contains("\"organization_type\""),
            "snake_case field name must not appear in JSON: {json}"
        );
    }

    #[test]
    fn response_dto_serializes_correctly() {
        let response = CreateOrganizationResponse {
            organization_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
            name: "ForgeOS".to_string(),
            organization_type: "foundation".to_string(),
            status: "Active".to_string(),
            version: 1,
        };

        let json = serde_json::to_string(&response).expect("serialization should succeed");
        let deserialized: CreateOrganizationResponse =
            serde_json::from_str(&json).expect("deserialization should succeed");

        assert_eq!(deserialized.organization_id, "550e8400-e29b-41d4-a716-446655440000");
        assert_eq!(deserialized.name, "ForgeOS");
        assert_eq!(deserialized.organization_type, "foundation");
        assert_eq!(deserialized.status, "Active");
        assert_eq!(deserialized.version, 1);
    }

    #[test]
    fn response_dto_uses_camel_case_field_names() {
        let response = CreateOrganizationResponse {
            organization_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
            name: "ForgeOS".to_string(),
            organization_type: "foundation".to_string(),
            status: "Active".to_string(),
            version: 1,
        };

        let json = serde_json::to_string(&response).expect("serialization should succeed");

        assert!(
            json.contains("\"organizationId\""),
            "expected camelCase field name 'organizationId' in JSON: {json}"
        );
        assert!(
            json.contains("\"organizationType\""),
            "expected camelCase field name 'organizationType' in JSON: {json}"
        );
        assert!(
            !json.contains("\"organization_id\""),
            "snake_case field name must not appear in JSON: {json}"
        );
        assert!(
            !json.contains("\"organization_type\""),
            "snake_case field name must not appear in JSON: {json}"
        );
    }
}