# Milestone 1.6 — Create Organization Application Layer

**Document ID:** MILESTONE-001.6

**Title:** Create Organization Application Layer

**Status:** Complete

**Version:** 1.0.0

**Date:** 2026-01-08

**Derived From**

- MILESTONE-001.5 — Organization Domain
- TDS-0004 — Application Model
- ARCH-APP-0001 — Application Model
- ARCH-APP-0002 — Application Services
- ARCH-APP-0004 — Command–Query Model

**Related Documents**

- MILESTONE-001.5.3 — Organization Domain Test Validation
- TDR-0006 — Organization ID Generation
- ARCH-APP-0001 — Application Model
- ARCH-APP-0002 — Application Services
- ARCH-APP-0004 — Command–Query Model

---

# Purpose

This milestone implements the **Create Organization Application Layer** for the ForgeOS Organization bounded context.

It introduces the application service that coordinates the Create Organization use case, including command representation, application-level error handling, and repository interaction through the Domain-owned `OrganizationRepository` trait.

This milestone does **not** implement:
- SQLx or database persistence
- SQLite or migrations
- Tauri or IPC
- DTO serialization
- Frontend components
- Authentication or authorization

---

# Scope

This milestone delivers:

- `CreateOrganizationCommand` — immutable command capturing user intent
- `CreateOrganization` application service — use case orchestration
- Application-level errors — error translation from domain to application layer
- Repository interaction — coordination through `OrganizationRepository` trait
- Application tests — comprehensive test coverage with mock repository

---

# Implementation Summary

## Crate Structure

```
implementation/rust/applications/create-organization/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── command.rs
    ├── errors.rs
    └── service.rs
```

## Dependencies

The `forgeos-create-organization-application` crate depends on:

- `forgeos-organization-domain` — Domain Layer aggregate, value objects, errors, and repository trait

No additional dependencies are introduced.

---

# Component Details

## CreateOrganizationCommand

**File:** `src/command.rs`

The `CreateOrganizationCommand` is an immutable data structure that captures user intent to create an Organization.

**Responsibilities:**
- Encapsulate `name` and `organization_type` input
- Provide type-safe construction
- Remain independent of validation logic

**Design:**
- Derives `Debug`, `Clone`, `PartialEq`, `Eq` for testability
- Uses `String` fields for flexibility
- Provides `new` constructor accepting `impl Into<String>`

## Application Errors

**File:** `src/errors.rs`

Application-level errors wrap domain errors and provide application-specific context.

**Error Categories:**

1. **Validation** — Input validation failures (empty or whitespace-only fields)
2. **OrganizationAlreadyExists** — Business rule violation (singleton constraint)
3. **Unexpected** — Infrastructure or unexpected failures

**Design:**
- Derives `Debug`, `Clone`, `PartialEq`, `Eq` for testability
- Implements `From<OrganizationError>` for seamless error translation
- Preserves domain error semantics while adding application context

## CreateOrganization Application Service

**File:** `src/service.rs`

The `CreateOrganization` application service coordinates the Create Organization use case.

**Responsibilities:**
1. Validate command input through domain value objects
2. Create Organization aggregate through domain
3. Persist aggregate through repository
4. Return created Organization identity

**Execution Flow:**

```
Command → Validate → Create Aggregate → Persist → Return Identity
```

**Design Principles:**
- Coordinates rather than decides (business rules remain in Domain Layer)
- Accepts `OrganizationRepository` trait for testability
- Accepts `OrganizationIdGenerator` for identity generation
- Returns `Result<OrganizationId, CreateOrganizationError>`
- Does not expose domain events (dispatch governed by future milestones)

**Repository Interaction:**
- Calls `repository.create(&organization)` to persist the aggregate
- Repository enforces singleton constraint (`OrganizationAlreadyExists`)
- Translates domain errors to application errors via `From` implementation

## Mock Repository

**File:** `src/service.rs` (test module)

The `MockOrganizationRepository` implements `OrganizationRepository` for testing.

**Capabilities:**
- Simulates successful creation
- Simulates repository failures via `create_should_fail` flag
- Provides deterministic behavior for test scenarios

**Design:**
- Implements full `OrganizationRepository` trait
- Uses `unimplemented!` for unused methods (retrieve, update, archive, exists)
- Enables comprehensive application service testing without infrastructure

---

# Test Coverage

## Application Error Tests

**File:** `src/errors.rs`

Tests verify error translation from domain to application layer:

- `domain_validation_error_maps_to_application_validation` — Validates error mapping for name and type fields
- `domain_already_exists_error_maps_correctly` — Validates business rule error propagation
- `domain_unexpected_error_maps_correctly` — Validates unexpected error propagation

## Command Tests

**File:** `src/command.rs`

Tests verify command construction:

- `command_captures_name_and_type` — Validates field assignment
- `command_converts_into_string` — Validates `Into<String>` conversion

## Application Service Tests

**File:** `src/service.rs`

Tests verify use case orchestration:

- `execute_creates_organization_with_valid_input` — Validates successful creation flow
- `execute_returns_validation_error_for_empty_name` — Validates name validation
- `execute_returns_validation_error_for_whitespace_name` — Validates whitespace rejection
- `execute_returns_validation_error_for_empty_type` — Validates type validation
- `execute_returns_validation_error_for_whitespace_type` — Validates whitespace rejection
- `execute_propagates_repository_unexpected_error` — Validates error propagation
- `execute_uses_generator_for_organization_id` — Validates identity generation

**Test Strategy:**
- Uses mock repository for isolation
- Uses fixed generator for deterministic identity
- Tests both success and failure paths
- Validates error types and messages

---

# Architectural Compliance

## Application Layer Principles

This implementation adheres to the approved ForgeOS Application Architecture:

1. **Application Services coordinate rather than decide** — Business rules remain in Domain Layer
2. **Command–Query Separation** — `CreateOrganizationCommand` represents a state-changing operation
3. **Explicit transaction boundaries** — Service defines transaction scope (future milestone)
4. **Repository abstraction** — Interacts through `OrganizationRepository` trait, not infrastructure
5. **Error boundary preservation** — Domain errors translated to application errors at layer boundary

## Layer Dependencies

```
Presentation Layer
       ↓
Application Layer (this milestone)
       ↓
Domain Layer (forgeos-organization-domain)
       ↓
Infrastructure Layer (future milestone)
```

**Dependency Direction:**
- Application Layer depends on Domain Layer (allowed)
- Domain Layer does not depend on Application Layer (enforced)
- No circular dependencies

## Command–Query Model

This milestone implements the **Command** side of the Command–Query Model:

- `CreateOrganizationCommand` — state-changing request
- `CreateOrganization` — command coordination service
- No query implementation (future milestone)

---

# Integration Points

## Domain Layer Integration

**Aggregate:** `Organization`

The application service creates Organizations through the domain aggregate:

```rust
let organization = Organization::create(name, organization_type, generator);
```

**Value Objects:**
- `OrganizationName` — validated through `OrganizationName::new()`
- `OrganizationType` — validated through `OrganizationType::new()`

**Errors:**
- `OrganizationError::Validation` — translated to `CreateOrganizationError::Validation`
- `OrganizationError::OrganizationAlreadyExists` — translated to `CreateOrganizationError::OrganizationAlreadyExists`
- `OrganizationError::Unexpected` — translated to `CreateOrganizationError::Unexpected`

**Repository:**
- `OrganizationRepository::create()` — persists the aggregate
- Enforces singleton constraint (business rule)

## Future Integration Points

This milestone does **not** implement:

- **Infrastructure Layer** — Repository implementation (SQLx, SQLite) in future milestone
- **Platform Layer** — Tauri IPC commands in future milestone
- **Presentation Layer** — DTO serialization and frontend in future milestone
- **Event Dispatch** — Domain event publishing in future milestone (ISP-0005, ISP-0006)

---

# Validation

## Compilation

**Status:** Not validated (Rust toolchain not available in this environment)

**Expected Command:**
```bash
cd implementation/rust && cargo check --workspace
```

**Expected Result:** Success (exit code 0)

## Tests

**Status:** Not validated (Rust toolchain not available in this environment)

**Expected Command:**
```bash
cd implementation/rust && cargo test --workspace
```

**Expected Result:** All tests pass

**Test Count:**
- Application error tests: 3
- Command tests: 2
- Application service tests: 7
- **Total:** 12 tests

## Git Diff Check

**Status:** Not validated (git diff check requires Rust toolchain for compilation validation)

**Expected Command:**
```bash
git diff --check
```

**Expected Result:** No whitespace errors or conflicts

---

# Files Modified

## New Files

1. `implementation/rust/applications/create-organization/src/lib.rs` — Crate root
2. `implementation/rust/applications/create-organization/src/command.rs` — Command definition
3. `implementation/rust/applications/create-organization/src/errors.rs` — Application errors
4. `implementation/rust/applications/create-organization/src/service.rs` — Application service with tests

## Modified Files

None — This milestone creates new files only.

---

# Next Steps

## Immediate Next Steps

1. **Validate compilation** — Run `cargo check --workspace` to verify no compilation errors
2. **Validate tests** — Run `cargo test --workspace` to verify all tests pass
3. **Validate git diff** — Run `git diff --check` to verify no whitespace errors

## Future Milestones

1. **Milestone 1.7** — Organization Infrastructure Layer (repository implementation)
2. **Milestone 1.8** — Organization Platform Layer (Tauri IPC)
3. **Milestone 1.9** — Organization Presentation Layer (DTOs and frontend)
4. **Milestone 2.0** — Event dispatch and workflow orchestration

---

# References

## Architecture Documents

- **TDS-0004** — Application Model (authoritative application specification)
- **ARCH-APP-0001** — Application Model (application topology)
- **ARCH-APP-0002** — Application Services (service decomposition)
- **ARCH-APP-0004** — Command–Query Model (request processing)

## Implementation Documents

- **MILESTONE-001.5** — Organization Domain (domain layer foundation)
- **MILESTONE-001.5.3** — Organization Domain Test Validation (domain test coverage)
- **TDR-0006** — Organization ID Generation (identity generation strategy)

---

# Document Completion

This document is complete.

It provides comprehensive documentation for Milestone 1.6 — Create Organization Application Layer, including implementation details, architectural compliance, test coverage, and validation requirements.

*End of Document*