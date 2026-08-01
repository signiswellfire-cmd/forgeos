# TDR-0004 — IPC Serialization Strategy

**TDR Number:** TDR-0004  
**Title:** IPC Serialization Strategy  
**Status:** Approved  
**Version:** 1.0.0  
**Related TDS:** TDS-0001 — System Architecture; TDS-0004 — Application Model  
**Related TDR:** TDR-0002 — Desktop Framework

---

# Purpose

This Technology Decision Record defines the DTO serialization and Tauri command contract for Milestone 1. It supplies a stable IPC boundary for Create Organization without selecting a frontend framework or allowing Domain types to cross process boundaries.

---

# Context

TDR-0002 selects Tauri 2.x and requires all frontend-to-backend operations to use published Tauri commands, dedicated DTOs, and no domain entities across IPC. It intentionally deferred the serialization detail. The Create Organization slice requires an explicit, versionable request, response, and error representation at that boundary.

The chosen serialization must be native to the selected Rust/Tauri runtime, preserve the DTO-only boundary in TDS-0001, and permit clients using a future frontend framework to interoperate without gaining access to application or domain internals.

---

# Decision

ForgeOS Milestone 1 shall use **Serde** to serialize and deserialize dedicated, immutable DTOs over Tauri's standard **JSON command IPC**.

The public Create Organization capability is exposed as one versioned Tauri command named `createOrganization`. It accepts one `CreateOrganizationRequest` DTO and returns either one `CreateOrganizationResponse` DTO or one `CreateOrganizationError` DTO.

The request DTO contains the `name` and `organizationType` fields approved in `MILESTONE-001-DOMAIN-DECISIONS.md`. The response DTO contains `organizationId`, `name`, `organizationType`, `status`, and `version`. Error DTOs contain a stable error code and a safe, user-facing message; they must not serialize Rust error chains, database details, or domain internals.

Tauri commands own deserialization, structural validation, authorization coordination, DTO-to-command mapping, application-service invocation, and result/error-to-DTO mapping. The Application and Domain layers do not depend on Tauri or Serde IPC types.

---

# Compatibility Considerations

The Milestone 1 IPC contract follows these rules:

1. Public command and DTO field names are stable API identifiers.
2. Backward-compatible changes may add optional request fields or additive response fields with documented defaults.
3. Renaming, removing, changing the meaning of a required field, or changing an error code is breaking and requires a new versioned command contract.
4. DTOs are dedicated boundary types; domain entities, value objects, repository records, and persistence errors never become IPC payloads.
5. JSON representation is the interoperability contract. A frontend implementation may use any framework that can invoke the approved command contract; this record selects no framework, state library, or component system.

---

# Rationale

Serde is Rust's established serialization mechanism and is used by Tauri command argument and return handling. JSON is Tauri's standard structured command payload format and provides inspectable, versionable DTO exchange for a local desktop client. A single request and response DTO prevent a broad command signature from becoming an implicit business API and make compatibility review explicit.

The boundary remains thin: it translates transport data and invokes an Application Service. This directly implements TDR-0002's command/DTO requirements and prevents Presentation from accessing repositories or domain state.

---

# Architectural Alignment

* **TDS-0001:** Presentation depends on Application, not Domain or Infrastructure; DTOs are the communication boundary.
* **TDS-0004:** commands coordinate state-changing requests through Application Services without business-rule ownership.
* **TDR-0002:** Tauri hosts IPC; published commands validate inputs, invoke application services, and return DTOs.
* **System Context:** only DTOs cross Trust Boundary TB-2; domain entities are never serialized across IPC.
* **ISP-0001, ISP-0002, and ISP-0008:** application services, command handlers, and classified error translation retain their proper ownership.

---

# Alternatives Considered

## Raw or binary IPC payloads

Rejected because Create Organization is structured business data, not a streaming or binary transfer. Raw payloads would add custom parsing and weaken DTO inspectability without a Milestone 1 benefit.

## Manual JSON construction

Rejected because manually assembled JSON duplicates DTO shape and error handling, increasing the risk of boundary drift. Serde keeps the mapping explicit and typed at the Platform boundary.

## Direct domain serialization

Rejected because TDR-0002 and the System Context prohibit domain entities from crossing IPC. It would expose behavior and persistence-independent internal structures as transport contracts.

## GraphQL, REST, or a local HTTP server

Rejected because Tauri command IPC is the approved desktop boundary. Adding a network protocol would introduce an undocumented runtime and security surface.

## Frontend-framework-specific RPC integration

Rejected because frontend framework selection remains intentionally deferred by TDR-0002. The DTO contract must remain independent of that choice.

---

# Consequences

Positive consequences:

* a type-oriented, DTO-only Tauri boundary;
* explicit contract compatibility rules;
* safe error responses that do not leak Infrastructure detail;
* no frontend framework coupling.

Trade-offs:

* DTO contracts require deliberate version management;
* JSON serialization overhead is accepted for this local, command-oriented MVP boundary;
* command naming and DTO fields become maintained public API once implemented.

---

# Future Considerations

Future decisions may define authentication context propagation, richer error localization, generated client bindings, streaming/binary transfers, API schema tooling, and additional command versions. They must preserve DTO-only IPC and Application-mediated business execution.

---

# References

* TDS-0001 — System Architecture
* TDS-0004 — Application Model
* TDR-0002 — Desktop Framework
* `docs/architecture/system-context.md`
* `docs/architecture/integration-boundaries.md`
* ISP-0001 — Application Service Pattern
* ISP-0002 — Command Handler Pattern
* ISP-0008 — Error Handling Pattern
* [Tauri command IPC documentation](https://v2.tauri.app/develop/calling-rust/)
