# Milestone 1.8 — Organization Platform Layer

**Document ID:** MILESTONE-001.8

**Title:** Organization Platform Layer

**Status:** Approved

**Version:** 1.0.0

**Date:** 2026-08-02

---

# Derived From

- MILESTONE-001 — Create Organization Vertical Slice
- MILESTONE-001.5 — Organization Domain
- MILESTONE-001.6 — Create Organization Application Layer
- MILESTONE-001.7 — Organization Infrastructure Layer
- TDS-0001 — System Architecture
- TDS-0004 — Application Model
- TDR-0001 — Programming Language (Rust/Cargo)
- TDR-0002 — Desktop Framework (Tauri 2.x)
- TDR-0004 — IPC Serialization Strategy (Serde/JSON)
- TDR-0005 — Workspace Location Reconciliation
- ARCH-0001 — System Context
- ARCH-0003 — Architecture Enforcement Specification
- ARCH-0004 — Workspace Specification

---

# Related Documents

- TDR-0002 — Desktop Framework (Tauri 2.x)
- TDR-0004 — IPC Serialization Strategy (Serde/JSON DTOs)
- ARCH-0001 — System Context (Trust Boundary TB-2)
- ARCH-0003 — Architecture Enforcement Specification (Dependency Contracts)
- ARCH-0004 — Workspace Specification (Platform category)
- ISP-0001 — Application Service Pattern
- ISP-0002 — Command Handler Pattern
- ISP-0007 — Dependency Injection Pattern
- ISP-0008 — Error Handling Pattern
- ISP-0009 — Testing Pattern
- ISP-0010 — Vertical Slice Pattern
- MILESTONE-001-DOMAIN-DECISIONS — Create Organization domain contract

---

# Purpose

This milestone defines the **Implementation Contract** for the Organization Platform Layer.

It specifies the Tauri IPC boundary, DTO definitions, dependency composition, and error translation required to expose the approved Create Organization capability through the desktop runtime.

This document is an **Implementation Contract Only**. It introduces no new architecture, RFC, TDS, TDR, ARCH, or ISP. Every responsibility herein traces to one or more approved authority documents.

This milestone does **not** implement:
- new architecture
- new RFC
- new TDS
- new TDR
- new ARCH
- new ISP
- new technology decisions
- new framework decisions

---

# Objective

Implement the Platform Layer that wires the approved Create Organization vertical slice into the Tauri 2.x desktop runtime, exposing a single versioned Tauri command (`createOrganization`) that accepts a request DTO, invokes the approved Application Service, and returns a response DTO or error DTO.

---

# Scope

This milestone delivers:

- **Tauri command registration** — one `createOrganization` command per TDR-0004
- **IPC Request DTO** — `CreateOrganizationRequest` with `name` and `organizationType` fields
- **IPC Response DTO** — `CreateOrganizationResponse` with `organizationId`, `name`, `organizationType`, `status`, and `version` fields
- **IPC Error DTO** — `CreateOrganizationError` with stable error code and safe user-facing message
- **Dependency composition** — wiring `SqliteOrganizationRepository` and `CreateOrganization` application service through constructor injection
- **Error translation** — mapping `CreateOrganizationError` variants to stable IPC error codes
- **Serde serialization** — JSON serialization of DTOs over Tauri command IPC
- **Platform tests** — unit tests for DTO mapping, error translation, and command behavior

---

# Out of Scope

This milestone does **not** introduce:

- authentication
- authorization
- workflow engine
- event bus redesign
- additional bounded contexts
- unrelated desktop capabilities
- frontend framework integration
- window management strategy
- plugin selection or configuration
- application lifecycle customization
- state-management architecture beyond approved DI (ISP-0007)
- IPC protocol changes
- serialization strategy changes
- async runtime changes
- logging framework changes
- command naming conventions beyond the approved `createOrganization` command (TDR-0004)

---

# Ownership

| Artifact | Architectural Owner |
|----------|---------------------|
| `forgeos-desktop-platform` crate | Platform Domain |
| Tauri command registration | Platform Domain |
| IPC DTOs | Platform Domain |
| Dependency composition root | Platform Domain |
| Error translation to IPC | Platform Domain |

Platform provides runtime capabilities without acquiring business responsibility (ARCH-0003, Dependency Contract — Platform).

---

# Crate Structure

```
implementation/rust/platform/desktop/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── commands.rs
    ├── dtos.rs
    ├── composition.rs
    └── errors.rs
```

The `forgeos-desktop-platform` crate already exists as a workspace member with dependencies on `forgeos-create-organization-application` and `forgeos-organization-infrastructure`. This milestone populates the crate with the Platform Layer implementation.

---

# Dependency Direction

## Allowed Dependencies

```text
Platform
    ↓
Application (forgeos-create-organization-application)
    ↓
Domain (forgeos-organization-domain)

Platform
    ↓
Infrastructure (forgeos-organization-infrastructure — composition/wiring only)
```

The Platform Layer depends on:
- `forgeos-create-organization-application` — Application Service invocation
- `forgeos-organization-infrastructure` — Infrastructure composition and wiring
- `forgeos-organization-domain` — transitive dependency through Application and Infrastructure
- `tauri` — desktop runtime (TDR-0002)
- `serde` — DTO serialization (TDR-0004)

## Forbidden Dependencies

Never allow:

- Domain → Platform
- Application → Platform
- Infrastructure → Platform

No Domain, Application, or Infrastructure crate depends on the Platform crate.

---

# Platform Responsibilities

The Platform Layer is responsible for:

1. **Tauri command registration** — registering the `createOrganization` command with the Tauri runtime
2. **IPC request deserialization** — receiving and deserializing `CreateOrganizationRequest` from the frontend
3. **Structural boundary validation** — validating that required DTO fields are present and structurally valid
4. **DTO-to-Command mapping** — translating `CreateOrganizationRequest` to `CreateOrganizationCommand`
5. **Application Service invocation** — invoking `CreateOrganization::execute()` through the approved application boundary
6. **Result-to-DTO mapping** — translating the `OrganizationId` result to `CreateOrganizationResponse`
7. **Error-to-DTO translation** — translating `CreateOrganizationError` to `CreateOrganizationError` IPC DTO
8. **Dependency composition** — constructing and wiring the repository, generator, and application service
9. **Serde serialization** — JSON serialization of all DTOs across the IPC boundary

The Platform Layer does **not**:
- implement business rules
- bypass aggregate boundaries
- access repositories directly for business operations
- serialize domain entities across IPC
- define command/query semantics
- own transaction boundaries

---

# Component Details

## Tauri Command Registration

**File:** `src/commands.rs`

The `createOrganization` Tauri command is the IPC boundary for the Create Organization capability.

**Command Name:** `createOrganization` (TDR-0004)

**Responsibilities:**
- Accept `CreateOrganizationRequest` DTO as input
- Perform structural boundary validation (required fields present, non-empty)
- Map DTO to `CreateOrganizationCommand`
- Retrieve composed dependencies (application service, ID generator) from Tauri managed state
- Invoke `CreateOrganization::execute(command, generator)`
- Map `Result<OrganizationId, CreateOrganizationError>` to `Result<CreateOrganizationResponse, CreateOrganizationError>` IPC DTO
- Return serialized response or error DTO

**Design:**
- Uses Tauri command attribute (`#[tauri::command]`)
- Receives composed dependencies through Tauri's state management (the Tauri-native realization of ISP-0007 constructor injection)
- No domain entities cross the IPC boundary (TDR-0002, TDR-0004, ARCH-0001 TB-2)
- Command function is thin: it translates transport data and invokes the Application Service

**Execution Flow:**

```text
CreateOrganizationRequest (DTO)
    ↓
Structural Validation
    ↓
DTO-to-Command Mapping
    ↓
CreateOrganization::execute(command, generator)
    ↓
Result<OrganizationId, CreateOrganizationError>
    ↓
Result-to-DTO Mapping
    ↓
CreateOrganizationResponse | CreateOrganizationError (DTO)
```

## IPC Request DTOs

**File:** `src/dtos.rs`

### CreateOrganizationRequest

The request DTO for the `createOrganization` command.

**Fields (TDR-0004):**

| Field | Type | Description |
|-------|------|-------------|
| `name` | `String` | The desired organization name |
| `organizationType` | `String` | The organization type classification |

**Design:**
- Derives `Debug`, `Clone`, `serde::Serialize`, `serde::Deserialize`
- Field names use camelCase for IPC compatibility (`name`, `organizationType`)
- Immutable boundary type; contains no business behavior
- Independent of domain entities and value objects
- Dedicated DTO type; not a domain entity serialization

## IPC Response DTOs

**File:** `src/dtos.rs`

### CreateOrganizationResponse

The response DTO for the `createOrganization` command on success.

**Fields (TDR-0004):**

| Field | Type | Description |
|-------|------|-------------|
| `organizationId` | `String` | The created organization's UUID |
| `name` | `String` | The organization name |
| `organizationType` | `String` | The organization type |
| `status` | `String` | The lifecycle status (`"Active"`) |
| `version` | `u64` | The aggregate version (`1`) |

**Design:**
- Derives `Debug`, `Clone`, `serde::Serialize`, `serde::Deserialize`
- Field names use camelCase for IPC compatibility
- Immutable boundary type; contains no business behavior
- Constructed from: application service return value (`organizationId`), request DTO fields (`name`, `organizationType`), and approved domain defaults (`status = "Active"`, `version = 1` per MILESTONE-001-DOMAIN-DECISIONS)
- Independent of domain entities and value objects

**Response Construction:**

The current `CreateOrganization::execute()` returns `OrganizationId`. The response DTO requires additional fields (`name`, `organizationType`, `status`, `version`). These fields are constructed from:

| Response Field | Source |
|----------------|--------|
| `organizationId` | Application Service return value (`OrganizationId::as_str()`) |
| `name` | Request DTO `name` field |
| `organizationType` | Request DTO `organizationType` field |
| `status` | Approved default: `"Active"` (MILESTONE-001-DOMAIN-DECISIONS) |
| `version` | Approved default: `1` (MILESTONE-001-DOMAIN-DECISIONS) |

This construction is a boundary translation responsibility owned by the Platform Layer per TDR-0004.

## IPC Error DTO

**File:** `src/dtos.rs` (or `src/errors.rs`)

### CreateOrganizationError

The error DTO returned when `createOrganization` fails.

**Design (TDR-0004):**
- Contains a stable error code and a safe, user-facing message
- Must not serialize Rust error chains, database details, or domain internals
- Derives `Debug`, `Clone`, `serde::Serialize`, `serde::Deserialize`

**Error Code Mapping:**

| Application Error | IPC Error Code | Safe Message |
|-------------------|----------------|--------------|
| `CreateOrganizationError::Validation(OrganizationField::Name)` | `VALIDATION_ERROR` | "Organization name is invalid" |
| `CreateOrganizationError::Validation(OrganizationField::OrganizationType)` | `VALIDATION_ERROR` | "Organization type is invalid" |
| `CreateOrganizationError::OrganizationAlreadyExists` | `ORGANIZATION_ALREADY_EXISTS` | "An organization already exists for this ForgeOS instance" |
| `CreateOrganizationError::Unexpected(_)` | `UNEXPECTED_ERROR` | "An unexpected error occurred" |

**Constraints:**
- The `Unexpected` variant's internal message is never serialized across IPC
- Error codes are stable API identifiers (TDR-0004, Compatibility Rule 3)
- Safe messages contain no infrastructure, database, or domain internal details

## Dependency Composition

**File:** `src/composition.rs`

The composition root wires the Create Organization vertical slice.

**Responsibilities (ISP-0007):**
- Construct `SqliteOrganizationRepository` (Infrastructure)
- Construct `DefaultOrganizationIdGenerator` (Domain)
- Construct `CreateOrganization` application service with the repository
- Register composed dependencies with the Tauri runtime for command access
- Provide deterministic, explicit object graph construction

**Composition Flow:**

```text
Create Composition Root
    ↓
Construct SqliteOrganizationRepository (Infrastructure)
    ↓
Run Database Migrations
    ↓
Construct DefaultOrganizationIdGenerator (Domain)
    ↓
Construct CreateOrganization Application Service
    ↓
Register Dependencies with Tauri State
    ↓
Register createOrganization Command
```

**Design:**
- Uses constructor injection (ISP-0007 recommended practice)
- Dependencies are explicit, declared, and testable
- No hidden dependencies or service locator patterns
- No business rules in the composition root
- Object graph is deterministic and immutable after construction
- Tauri's state management (`tauri::State`) is the runtime mechanism for providing composed dependencies to command functions; this is the Tauri-native realization of ISP-0007, not a separate state-management architecture

**Composed Dependencies:**

| Dependency | Type | Source |
|------------|------|--------|
| Repository | `SqliteOrganizationRepository` | `forgeos-organization-infrastructure` |
| ID Generator | `DefaultOrganizationIdGenerator` | `forgeos-organization-domain` |
| Application Service | `CreateOrganization<'_, SqliteOrganizationRepository>` | `forgeos-create-organization-application` |

## Error Translation

**File:** `src/errors.rs` (or `src/commands.rs`)

The Platform Layer translates `CreateOrganizationError` (Application) to the IPC error DTO at the boundary.

**Responsibilities (ISP-0008, TDR-0004):**
- Map each `CreateOrganizationError` variant to a stable error code and safe message
- Preserve error category meaning without leaking infrastructure details
- Never serialize Rust error chains, database details, or domain internals
- Provide deterministic error translation for equivalent failures

**Translation Rules:**

```text
CreateOrganizationError::Validation(Name)
    → CreateOrganizationError { code: "VALIDATION_ERROR", message: "Organization name is invalid" }

CreateOrganizationError::Validation(OrganizationType)
    → CreateOrganizationError { code: "VALIDATION_ERROR", message: "Organization type is invalid" }

CreateOrganizationError::OrganizationAlreadyExists
    → CreateOrganizationError { code: "ORGANIZATION_ALREADY_EXISTS", message: "An organization already exists for this ForgeOS instance" }

CreateOrganizationError::Unexpected(_)
    → CreateOrganizationError { code: "UNEXPECTED_ERROR", message: "An unexpected error occurred" }
```

**Constraints:**
- The `Unexpected` variant's internal `String` message is never exposed across IPC
- Error codes are stable public API identifiers (TDR-0004)
- Translation preserves error category (Validation, BusinessRule, Unexpected) without preserving internal details
- Translation is deterministic: equivalent application errors produce equivalent IPC error DTOs

---

# Integration Points

## Application Layer Integration

**Application Service:** `CreateOrganization<'_, R: OrganizationRepository>`

The Platform Layer invokes the approved Application Service:

```rust
let result = service.execute(command, generator);
// Returns Result<OrganizationId, CreateOrganizationError>
```

**Command Mapping:**

| IPC Request Field | Application Command Field |
|-------------------|--------------------------|
| `name` | `CreateOrganizationCommand::name` |
| `organizationType` | `CreateOrganizationCommand::organization_type` |

**Error Integration:**

| Application Error | Platform Translation |
|-------------------|---------------------|
| `Validation(OrganizationField::Name)` | IPC error DTO with `VALIDATION_ERROR` code |
| `Validation(OrganizationField::OrganizationType)` | IPC error DTO with `VALIDATION_ERROR` code |
| `OrganizationAlreadyExists` | IPC error DTO with `ORGANIZATION_ALREADY_EXISTS` code |
| `Unexpected(String)` | IPC error DTO with `UNEXPECTED_ERROR` code (internal message suppressed) |

## Infrastructure Layer Integration

**Repository:** `SqliteOrganizationRepository`

The Platform Layer constructs and wires the Infrastructure repository for the composition root:

```rust
let repository = SqliteOrganizationRepository::new(database_url).await?;
repository.run_migrations().await?;
```

The Platform Layer interacts with Infrastructure for **composition and wiring only**. It does not invoke repository methods directly for business operations.

## Domain Layer Integration

**ID Generator:** `DefaultOrganizationIdGenerator`

The Platform Layer constructs the Domain-owned ID generator and passes it to the Application Service:

```rust
let generator = DefaultOrganizationIdGenerator;
let result = service.execute(command, &generator);
```

The Platform Layer does not access Domain aggregates, value objects, or repository traits directly for business operations. All business execution flows through the Application Service.

## Future Integration Points

This milestone does **not** implement:

- **Presentation Layer** — Frontend components and UI integration in future milestone
- **Event Dispatch** — Domain event publishing in future milestone (ISP-0005, ISP-0006)
- **Transaction Coordination** — Explicit transaction boundaries in Application Service (future milestone)
- **Authentication** — Caller authentication at IPC boundary (future authority required)
- **Authorization** — Operation authorization at Application Layer (future authority required)

---

# Testing Responsibility

## Platform Layer Tests

**File:** `src/commands.rs` (test module), `src/dtos.rs` (test module), `src/errors.rs` (test module)

Tests verify Platform Layer behavior:

### DTO Tests

- `request_dto_serializes_and_deserializes` — Validates Serde round-trip for `CreateOrganizationRequest`
- `response_dto_serializes_correctly` — Validates Serde serialization for `CreateOrganizationResponse`
- `error_dto_serializes_correctly` — Validates Serde serialization for IPC error DTO

### Error Translation Tests

- `validation_name_error_translates_to_ipc_error` — Validates `Validation(Name)` mapping
- `validation_type_error_translates_to_ipc_error` — Validates `Validation(OrganizationType)` mapping
- `already_exists_error_translates_to_ipc_error` — Validates `OrganizationAlreadyExists` mapping
- `unexpected_error_translates_to_ipc_error` — Validates `Unexpected(_)` mapping and message suppression

### Command Tests

- `create_organization_command_maps_dto_to_command` — Validates DTO-to-Command mapping
- `create_organization_command_maps_result_to_response` — Validates result-to-response mapping
- `create_organization_command_maps_error_to_ipc_error` — Validates error-to-DTO translation

**Test Strategy (ISP-0009):**
- Uses mock or in-memory dependencies for isolation
- Tests both success and failure paths
- Validates DTO serialization round-trips
- Validates error code stability
- Validates that no domain internals leak across IPC boundary
- Tests are deterministic and independently executable

---

# Validation Requirements

## Compilation

**Expected Command:**
```bash
cd implementation/rust && cargo check --workspace
```

**Expected Result:** Success (exit code 0)

## Tests

**Expected Command:**
```bash
cd implementation/rust && cargo test --workspace
```

**Expected Result:** All tests pass

**Expected Test Count:**
- Existing tests: 49 (Domain + Application + Infrastructure)
- Platform DTO tests: 3
- Platform error translation tests: 4
- Platform command tests: 3
- **Total:** 59 tests (estimated)

## Architecture Validation

**Dependency Direction Verification:**
- Platform depends on Application (allowed)
- Platform depends on Infrastructure for composition only (allowed)
- Application does not depend on Platform (enforced)
- Domain does not depend on Platform (enforced)
- Infrastructure does not depend on Platform (enforced)

**IPC Boundary Verification:**
- Only DTOs cross the IPC boundary
- Domain entities are never serialized across IPC
- Error DTOs contain no infrastructure or domain internal details

## Git Diff Check

**Expected Command:**
```bash
git diff --check
```

**Expected Result:** No whitespace errors or conflicts

---

# Expected Files

The following files are expected to be created or modified during Phase 4 implementation. They are **not** created by this milestone document.

## New Files

1. `implementation/rust/platform/desktop/src/commands.rs` — Tauri command registration (`createOrganization`)
2. `implementation/rust/platform/desktop/src/dtos.rs` — IPC Request, Response, and Error DTOs with Serde
3. `implementation/rust/platform/desktop/src/composition.rs` — Dependency composition root
4. `implementation/rust/platform/desktop/src/errors.rs` — Error translation from Application to IPC

## Modified Files

1. `implementation/rust/platform/desktop/Cargo.toml` — Add `tauri` and `serde` dependencies
2. `implementation/rust/platform/desktop/src/lib.rs` — Module declarations and re-exports

---

# Traceability Matrix

Every major implementation responsibility traces to one or more approved authority documents. No responsibility relies solely on this milestone document.

| Responsibility | Authority | Notes |
|----------------|-----------|-------|
| Tauri 2.x as desktop runtime | TDR-0002 | Desktop Framework selection |
| `createOrganization` command name | TDR-0004 | IPC Serialization Strategy, single versioned command |
| Serde/JSON serialization for DTOs | TDR-0004 | IPC Serialization Strategy |
| Request DTO fields (`name`, `organizationType`) | TDR-0004, MILESTONE-001-DOMAIN-DECISIONS | Approved input contract |
| Response DTO fields (`organizationId`, `name`, `organizationType`, `status`, `version`) | TDR-0004 | IPC Serialization Strategy |
| Error DTO with stable error code and safe message | TDR-0004, ISP-0008 | Error DTO must not leak internals |
| No domain entities across IPC | TDR-0002, TDR-0004, ARCH-0001 TB-2 | Trust Boundary TB-2 enforcement |
| Structural validation at IPC boundary | ARCH-0001 TB-2, TDR-0004 | Trust Boundary TB-2 responsibilities |
| DTO-to-Command mapping | TDR-0004, ISP-0002 | Tauri commands own DTO-to-command mapping |
| Application Service invocation | TDR-0004, ISP-0001 | Commands invoke Application Services |
| Result-to-DTO mapping | TDR-0004 | Tauri commands own result-to-DTO mapping |
| Error-to-DTO translation | TDR-0004, ISP-0008 | Error translation at boundary |
| Dependency composition through constructor injection | ISP-0007 | Dependency Injection Pattern |
| Composition root constructs Infrastructure and Application | ISP-0007, ARCH-0003 | DI Pattern, Platform Dependency Contract |
| Platform → Application dependency direction | ARCH-0003 | Dependency Contract — Platform, Application Services |
| Platform → Infrastructure (composition only) | ARCH-0003, Task Dependency Rules | Infrastructure composition/wiring |
| Forbidden: Domain → Platform | ARCH-0003 | Dependency Contract — Organization Domain |
| Forbidden: Application → Platform | ARCH-0003 | Dependency Contract — Application Services |
| Forbidden: Infrastructure → Platform | ARCH-0003 | Dependency Contract — Infrastructure |
| Platform crate location (`platform/desktop`) | ARCH-0004, TDR-0005 | Workspace Specification, Workspace Location |
| Platform workspace ownership | ARCH-0004 | Platform category, Platform Domain owner |
| `DefaultOrganizationIdGenerator` usage | TDR-0006, MILESTONE-001.5 | Organization ID Generation |
| Response `status` default `"Active"` | MILESTONE-001-DOMAIN-DECISIONS, MILESTONE-001.5 | Approved initial status |
| Response `version` default `1` | MILESTONE-001-DOMAIN-DECISIONS, MILESTONE-001.5 | Approved initial version |
| Platform tests follow ISP-0009 | ISP-0009 | Testing Pattern |
| Vertical slice scope limited to Create Organization | ISP-0010, MILESTONE-001 | Vertical Slice Pattern |
| No business rules in Platform | ARCH-0003 AV-001, TDR-0002 | Business logic only in domains |
| No frontend framework selection | TDR-0002 | Frontend framework intentionally deferred |
| No state-management beyond approved DI | ISP-0007 | DI Pattern, constructor injection |

---

# Tauri Implementation Constraints

This milestone **shall not** decide or introduce:

- command naming conventions beyond the approved `createOrganization` command (TDR-0004)
- plugin selection
- plugin configuration
- application lifecycle customization
- state-management architecture beyond approved DI (ISP-0007)
- frontend framework integration
- window management strategy
- IPC protocol changes
- serialization strategy changes
- async runtime changes
- logging framework changes

If any of the above are required during implementation:

**STOP.**

Record the missing authority.

Do not invent a solution.

---

# Next Steps

## Phase 4 — Implementation

1. **Modify `Cargo.toml`** — Add `tauri` and `serde` dependencies to `forgeos-desktop-platform`
2. **Implement `dtos.rs`** — Create `CreateOrganizationRequest`, `CreateOrganizationResponse`, and IPC error DTO with Serde derives
3. **Implement `errors.rs`** — Create error translation from `CreateOrganizationError` to IPC error DTO
4. **Implement `composition.rs`** — Create dependency composition root wiring repository, generator, and application service
5. **Implement `commands.rs`** — Create `createOrganization` Tauri command with DTO mapping and service invocation
6. **Modify `lib.rs`** — Add module declarations and re-exports
7. **Implement tests** — DTO tests, error translation tests, command tests
8. **Validate compilation** — Run `cargo check --workspace`
9. **Validate tests** — Run `cargo test --workspace`
10. **Validate git diff** — Run `git diff --check`

## Future Milestones

1. **Milestone 1.9** — Organization Presentation Layer (frontend integration, UI components)
2. **Milestone 2.0** — Event dispatch and workflow orchestration
3. **Milestone 2.1** — Transaction coordination in Application Service

---

# References

## Technology Decision Records

- **TDR-0001** — Programming Language (Rust/Cargo)
- **TDR-0002** — Desktop Framework (Tauri 2.x)
- **TDR-0004** — IPC Serialization Strategy (Serde/JSON DTOs)
- **TDR-0005** — Workspace Location Reconciliation
- **TDR-0006** — Organization ID Generation (UUID v4)

## Architecture Documents

- **ARCH-0001** — System Context (Trust Boundaries TB-1 through TB-6)
- **ARCH-0003** — Architecture Enforcement Specification (Dependency Contracts)
- **ARCH-0004** — Workspace Specification (Platform category)

## Implementation Specifications

- **ISP-0001** — Application Service Pattern
- **ISP-0002** — Command Handler Pattern
- **ISP-0007** — Dependency Injection Pattern
- **ISP-0008** — Error Handling Pattern
- **ISP-0009** — Testing Pattern
- **ISP-0010** — Vertical Slice Pattern

## Implementation Documents

- **MILESTONE-001** — Create Organization Vertical Slice plan
- **MILESTONE-001-DOMAIN-DECISIONS** — Create Organization domain contract
- **MILESTONE-001.5** — Organization Domain Foundation
- **MILESTONE-001.6** — Create Organization Application Layer
- **MILESTONE-001.7** — Organization Infrastructure Layer

---

# Document Completion

This document is complete.

It provides the **Implementation Contract** for Milestone 1.8 — Organization Platform Layer, including Tauri command registration, IPC DTO definitions, dependency composition, error translation, testing responsibility, validation requirements, expected files, and full traceability to approved authority documents.

This document introduces no new architecture, RFC, TDS, TDR, ARCH, ISP, technology decisions, or framework decisions. Every responsibility traces to one or more approved authority documents in the ForgeOS authority chain.

*End of Document*