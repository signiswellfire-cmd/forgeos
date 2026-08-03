# MILESTONE-001.9 — Organization Presentation Layer

**Milestone ID:** MILESTONE-001.9

**Title:** Organization Presentation Layer

**Status:** Approved

**Version:** 1.0.0

**Related Milestones:**

- MILESTONE-001 — Create Organization vertical slice plan
- MILESTONE-001.5 — Organization Domain
- MILESTONE-001.6 — Create Organization Application
- MILESTONE-001.7 — Organization Infrastructure
- MILESTONE-001.8 — Organization Platform

---

# Purpose

This milestone defines the implementation contract for the Organization Presentation Layer within the Create Organization vertical slice.

It specifies the scope, ownership, crate boundaries, dependency direction, integration points, expected files, expected modules, expected public APIs, testing responsibilities, validation requirements, and architecture drift requirements for the Presentation Layer that renders the Create Organization user interface and dispatches the `createOrganization` Tauri command.

This document introduces **no new architecture**, **no new technology decisions**, **no RFC**, **no TDS**, **no TDR**, **no ARCH**, and **no ISP**.

All scope is derived exclusively from the approved authority documents listed in the Authority Coverage Matrix.

---

# Objective

Implement the Organization Presentation Layer for the Create Organization vertical slice.

The Presentation Layer shall:

- render the Create Organization user interface;
- collect user input for organization name and organization type;
- dispatch the `createOrganization` Tauri command through the IPC boundary;
- display the `CreateOrganizationResponse` DTO on success;
- display the `CreateOrganizationError` DTO on failure.

The Presentation Layer shall contain **no business logic**, **no domain logic**, **no persistence logic**, **no workflow rules**, and **no governance rules**.

---

# Scope

This milestone covers:

1. **Presentation crate creation** — a new Rust crate under `implementation/rust/presentation/` that owns the Create Organization user interface and IPC integration.

2. **UI composition** — rendering the Create Organization form with fields for organization name and organization type, consistent with the domain contract approved in `MILESTONE-001-DOMAIN-DECISIONS.md`.

3. **IPC integration** — invoking the `createOrganization` Tauri command defined by `MILESTONE-001.8` and `TDR-0004`, passing the `CreateOrganizationRequest` DTO and receiving either `CreateOrganizationResponse` or `CreateOrganizationError`.

4. **View state management** — transient UI state for form inputs, submission status, and error display, owned exclusively by the Presentation Domain.

5. **Response rendering** — displaying the result DTO fields (`organizationId`, `name`, `organizationType`, `status`, `version`) on successful creation.

6. **Error rendering** — displaying the error DTO's stable error code and safe user-facing message on failure.

7. **Presentation tests** — verification of UI behavior, IPC dispatch, and DTO handling per `ISP-0009` and `ISP-0010`.

---

# Out of Scope

This milestone does **not** cover:

1. **Frontend framework selection** — the choice of web UI framework (e.g., React, Vue, Svelte, vanilla) remains deferred per `TDR-0002` Future Considerations. No new technology decision is introduced.

2. **State management architecture** — no state management library or pattern beyond the approved DI pattern (`ISP-0007`) is introduced.

3. **UI component library** — no component library or design system is selected.

4. **Domain logic** — no business rules, aggregate behavior, or domain invariants are implemented in the Presentation Layer.

5. **Persistence** — no database access, file I/O, or storage operations are performed by the Presentation Layer.

6. **Workflow rules** — no orchestration, transaction coordination, or workflow logic is implemented in the Presentation Layer.

7. **Governance rules** — no policy evaluation or authority checks are implemented in the Presentation Layer.

8. **Domain entity serialization** — domain entities never cross the IPC boundary; only DTOs traverse the boundary per `TDR-0002`, `TDR-0004`, and `ARCH-0001` TB-2.

9. **Backend command implementation** — the `createOrganization` Tauri command, DTOs, composition root, and error translation are already implemented by `MILESTONE-001.8`.

10. **Event dispatch** — domain event publishing remains a future milestone per `ISP-0005` and `ISP-0006`.

11. **Transaction coordination** — explicit transaction boundaries remain in the Application Service per `ISP-0006`.

12. **Additional windows** — only the Create Organization view is in scope. Settings, About, and other windows remain future work per `TDR-0002`.

---

# Ownership

| Artifact | Architectural Owner | Authority |
|----------|---------------------|-----------|
| Presentation crate | Presentation Domain | ARCH-0002 |
| UI composition | Presentation Domain | ARCH-0002; TDR-0002 |
| View state | Presentation Domain | ARCH-0002 |
| IPC integration | Presentation Domain | ARCH-0002; TDR-0002; TDR-0004 |
| Response rendering | Presentation Domain | ARCH-0002; TDR-0004 |
| Error rendering | Presentation Domain | ARCH-0002; TDR-0004; ISP-0008 |
| Presentation tests | Presentation Domain | ISP-0009; ISP-0010 |

Ownership is exclusive. No artifact shall have multiple architectural owners.

---

# Crate Boundaries

## New Crate

| Crate Name | Workspace Category | Architectural Owner | Location |
|------------|--------------------|---------------------|----------|
| `forgeos-organization-presentation` | Presentation | Presentation Domain | `implementation/rust/presentation/` |

The crate name is a repository implementation decision consistent with the naming convention established by existing crates (`forgeos-organization-domain`, `forgeos-create-organization`, `forgeos-organization-infrastructure`, `forgeos-desktop-platform`).

## Existing Crates (Consumed, Not Modified)

| Crate | Workspace Category | Architectural Owner |
|-------|--------------------|---------------------|
| `forgeos-desktop-platform` | Platform | Platform Domain |
| `forgeos-create-organization` | Applications | Application Services |
| `forgeos-organization-domain` | Domains | Organization Domain |
| `forgeos-organization-infrastructure` | Infrastructure | Infrastructure Domain |

The Presentation crate depends on the Platform crate for the `createOrganization` Tauri command and DTO types. It does **not** depend on Domain, Infrastructure, or Application crates directly.

---

# Dependency Direction

## Approved Dependency Direction

```text
Presentation
    │
    ▼
Platform (Desktop Runtime)
    │
    ▼
Application Services
    │
    ▼
Implementation Domains
    │
    ▼
Infrastructure
    │
    ▼
Platform
```

## Presentation Dependency Contract

| Dependency | Status | Authority |
|------------|--------|-----------|
| Application Services | Required | ARCH-0003; TDS-0001 |
| Desktop Runtime (Tauri) | Required | ARCH-0003; TDR-0002 |
| UI Framework | Allowed | ARCH-0003; TDR-0002 |
| Domain | Forbidden | ARCH-0003; TDS-0001; ARCH-0002 |
| Repository Implementations | Forbidden | ARCH-0003; ARCH-0002 |
| Storage Providers | Forbidden | ARCH-0003; ARCH-0002 |
| AI Providers | Forbidden | ARCH-0003; ARCH-0002 |

The Presentation Layer communicates with the backend exclusively through the `createOrganization` Tauri command published by the Platform Layer. It does not import Domain, Infrastructure, or Application crates directly.

---

# Integration Points

## 1. Tauri Command IPC

| Integration Point | Authority | Description |
|-------------------|-----------|-------------|
| `createOrganization` command | TDR-0002; TDR-0004; MILESTONE-001.8 | The Presentation Layer invokes the `createOrganization` Tauri command via IPC, passing a `CreateOrganizationRequest` DTO and receiving either `CreateOrganizationResponse` or `CreateOrganizationError`. |

## 2. DTO Contract

| DTO | Fields | Authority |
|-----|--------|-----------|
| `CreateOrganizationRequest` | `name`, `organizationType` | TDR-0004; MILESTONE-001-DOMAIN-DECISIONS |
| `CreateOrganizationResponse` | `organizationId`, `name`, `organizationType`, `status`, `version` | TDR-0004; MILESTONE-001-DOMAIN-DECISIONS |
| `CreateOrganizationError` | stable error code, safe user-facing message | TDR-0004; ISP-0008 |

DTOs are immutable, contain no business behavior, are versionable, and remain independent of domain entities per `TDR-0002` and `TDR-0004`.

## 3. Trust Boundary TB-1 (User Interface)

| Boundary | Trust Level | Authority |
|----------|-------------|-----------|
| TB-1 — User Interface | Untrusted | ARCH-0001 |

User input is considered untrusted. The Presentation Layer performs structural validation of form inputs before dispatching the `createOrganization` command. Business validation remains in the Domain Layer.

## 4. Trust Boundary TB-2 (IPC Boundary)

| Boundary | Trust Level | Authority |
|----------|-------------|-----------|
| TB-2 — IPC Boundary | Validated | ARCH-0001; TDR-0002; TDR-0004 |

Only DTOs cross TB-2. Domain entities never cross the IPC boundary.

---

# Expected Files

## New Files

| File | Purpose | Authority |
|------|---------|-----------|
| `implementation/rust/presentation/Cargo.toml` | Crate manifest | ARCH-0004; TDR-0002 |
| `implementation/rust/presentation/src/lib.rs` | Crate root and public API | ARCH-0002; ISP-0010 |
| `implementation/rust/presentation/src/ui.rs` | UI composition and rendering | ARCH-0002; TDR-0002 |
| `implementation/rust/presentation/src/view_model.rs` | View model for Create Organization form | ARCH-0002; TDR-0002 |
| `implementation/rust/presentation/src/ipc.rs` | IPC integration — `createOrganization` command dispatch | ARCH-0002; TDR-0002; TDR-0004 |
| `implementation/rust/presentation/src/errors.rs` | Presentation-layer error types | ISP-0008; TDR-0004 |
| `implementation/rust/presentation/src/composition.rs` | Presentation composition root | ISP-0007; ARCH-0003 |
| `implementation/rust/presentation/tests/presentation_test.rs` | Presentation integration tests | ISP-0009; ISP-0010 |

## Existing Files (Consumed, Not Modified)

| File | Purpose | Authority |
|------|---------|-----------|
| `implementation/rust/platform/desktop/src/commands.rs` | `createOrganization` Tauri command | MILESTONE-001.8; TDR-0004 |
| `implementation/rust/platform/desktop/src/dtos.rs` | IPC DTOs | MILESTONE-001.8; TDR-0004 |
| `implementation/rust/platform/desktop/src/errors.rs` | Error translation | MILESTONE-001.8; ISP-0008 |
| `implementation/rust/platform/desktop/src/composition.rs` | Platform composition root | MILESTONE-001.8; ISP-0007 |
| `implementation/rust/platform/desktop/src/lib.rs` | Platform crate root | MILESTONE-001.8 |

---

# Expected Modules

## `forgeos-organization-presentation` Crate

### `lib.rs`

- Crate-level documentation
- Public re-exports of view model and IPC integration types
- Module declarations

### `ui.rs`

- Create Organization form rendering
- Form field binding (name, organization type)
- Submit button handler
- Response display
- Error display

### `view_model.rs`

- `CreateOrganizationViewModel` — transient UI state for the Create Organization form
- Form input state (name, organization type)
- Submission status (idle, submitting, success, error)
- Response display state
- Error display state

### `ipc.rs`

- `invoke_create_organization` — dispatches the `createOrganization` Tauri command
- Request DTO construction from view model state
- Response DTO handling
- Error DTO handling

### `errors.rs`

- `PresentationError` — Presentation-layer error type
- Error code mapping for display
- No domain or infrastructure error leakage

### `composition.rs`

- Presentation composition root
- View model construction
- IPC integration wiring

---

# Expected Public APIs

## `CreateOrganizationViewModel`

| API | Signature | Authority |
|-----|-----------|-----------|
| `new()` | Creates a new view model with default (empty) form state | ARCH-0002; TDR-0002 |
| `name()` | Returns the current organization name input | ARCH-0002; TDR-0002 |
| `set_name(name: String)` | Updates the organization name input | ARCH-0002; TDR-0002 |
| `organization_type()` | Returns the current organization type input | ARCH-0002; TDR-0002 |
| `set_organization_type(org_type: String)` | Updates the organization type input | ARCH-0002; TDR-0002 |
| `status()` | Returns the current submission status | ARCH-0002; TDR-0002 |
| `response()` | Returns the response DTO if creation succeeded | TDR-0004 |
| `error()` | Returns the error DTO if creation failed | TDR-0004; ISP-0008 |

## `invoke_create_organization`

| API | Signature | Authority |
|-----|-----------|-----------|
| `invoke_create_organization(view_model: &CreateOrganizationViewModel)` | Dispatches the `createOrganization` Tauri command with the request DTO and updates the view model with the response or error | TDR-0002; TDR-0004; ISP-0001 |

## `PresentationError`

| API | Description | Authority |
|-----|-------------|-----------|
| `PresentationError` | Presentation-layer error type with stable error codes and safe messages | ISP-0008; TDR-0004 |

---

# Testing Responsibilities

## Test Ownership

| Test Type | Owner | Authority |
|-----------|-------|-----------|
| Presentation unit tests | Presentation Domain | ISP-0009; ISP-0010 |
| Presentation integration tests | Presentation Domain | ISP-0009; ISP-0010 |
| Architecture tests | Architecture verification | ARCH-0003; ISP-0009 |

## Test Scope

### Presentation Unit Tests

- View model state transitions (idle → submitting → success/error)
- Form input validation (structural validation at TB-1)
- Response DTO field access
- Error DTO field access
- Error code mapping

### Presentation Integration Tests

- `createOrganization` command dispatch via IPC
- Request DTO construction from view model
- Response DTO handling
- Error DTO handling
- End-to-end flow from form submission to response display

### Architecture Tests

- Presentation crate depends only on Platform (not Domain, Infrastructure, or Application)
- No domain entities cross the IPC boundary
- No business logic in Presentation
- No persistence logic in Presentation

## Test Principles

- Tests shall be deterministic per `ISP-0009`
- Tests shall verify behavior at the correct architectural boundary per `ISP-0009`
- Tests shall preserve dependency boundaries per `ISP-0009`
- Tests shall not mock domain behavior per `ISP-0009`
- Tests shall verify both success and failure paths per `ISP-0009`

---

# Validation Requirements

## Compile-Time Validation

| Requirement | Authority |
|-------------|-----------|
| Presentation crate compiles without depending on Domain, Infrastructure, or Application crates | ARCH-0003; TDS-0001 |
| No domain entities are imported or referenced in Presentation | ARCH-0003; TDR-0002; TDR-0004 |
| No business logic exists in Presentation | ARCH-0003 AV-002; TDS-0001 |
| No persistence logic exists in Presentation | ARCH-0003; TDS-0001 |
| No workflow rules exist in Presentation | ARCH-0003; TDS-0001 |

## Repository-Time Validation

| Requirement | Authority |
|-------------|-----------|
| Presentation crate is registered as a workspace member | ARCH-0004 |
| Repository structure conforms to the approved workspace layout | ARCH-0004 |
| Documentation references remain consistent with approved architecture | ARCH-0003 |
| Dependency graph matches approved contracts | ARCH-0003 |

## Runtime Validation

| Requirement | Authority |
|-------------|-----------|
| `createOrganization` command is invocable from the Presentation Layer | TDR-0002; TDR-0004 |
| DTOs are correctly serialized and deserialized across IPC | TDR-0004 |
| Error DTOs do not leak domain or infrastructure internals | TDR-0004; ISP-0008 |

---

# Dependency Approval Requirements

## New Dependencies

| Dependency | Approval Required | Authority |
|------------|-------------------|-----------|
| UI framework (e.g., web UI framework for Tauri frontend) | TDR-0002 Future Considerations | TDR-0002 |
| Any external crate added to the Presentation crate | Dependency Contract approval | ARCH-0003 |

## Existing Dependencies (Consumed)

| Dependency | Source | Authority |
|------------|--------|-----------|
| `forgeos-desktop-platform` | Platform crate | MILESTONE-001.8; ARCH-0004 |
| Tauri runtime | TDR-0002 | TDR-0002 |
| Serde (for DTO serialization) | TDR-0004 | TDR-0004 |

No new technology decisions are introduced by this milestone. The frontend framework selection remains deferred per `TDR-0002`.

---

# Public API Requirements

## Stability

| Requirement | Authority |
|-------------|-----------|
| Public API names are stable identifiers | TDR-0004 |
| Backward-compatible changes may add optional fields or additive response fields | TDR-0004 |
| Renaming, removing, or changing the meaning of a required field is breaking and requires a new versioned contract | TDR-0004 |

## DTO Boundary

| Requirement | Authority |
|-------------|-----------|
| Only DTOs cross the IPC boundary | TDR-0002; TDR-0004; ARCH-0001 TB-2 |
| Domain entities never cross the IPC boundary | TDR-0002; TDR-0004; ARCH-0001 TB-2 |
| DTOs are immutable | TDR-0002; TDR-0004 |
| DTOs contain no business behavior | TDR-0002; TDR-0004 |
| DTOs are versionable | TDR-0002; TDR-0004 |
| DTOs remain independent of domain entities | TDR-0002; TDR-0004 |

## Error Handling

| Requirement | Authority |
|-------------|-----------|
| Error DTOs contain a stable error code and a safe, user-facing message | TDR-0004; ISP-0008 |
| Error DTOs must not serialize Rust error chains, database details, or domain internals | TDR-0004; ISP-0008 |
| Error translation occurs at the Platform boundary | TDR-0004; ISP-0008; MILESTONE-001.8 |

---

# Architecture Drift Requirements

## Drift Categories

| Drift Category | Architectural Invariant | Verification | Authority |
|----------------|------------------------|--------------|-----------|
| Dependency Drift | Approved dependency contracts remain unchanged | Cargo dependency graph analysis; compile-time dependency validation | ARCH-0003 |
| Ownership Drift | Every artifact has exactly one architectural owner | Ownership registry validation; repository ownership analysis | ARCH-0003 |
| Interface Drift | Published interfaces remain stable and owned by one Implementation Domain | Public API analysis; interface compatibility testing | ARCH-0003 |
| Repository Drift | Repository organization conforms to the approved Workspace Specification | Repository layout validation; workspace inventory comparison | ARCH-0003; ARCH-0004 |
| Workspace Drift | Cargo workspace structure reflects the approved architectural model | Workspace manifest validation; dependency graph verification | ARCH-0003; ARCH-0004 |
| Persistence Drift | Persistence ownership follows architectural ownership | Repository ownership analysis; aggregate ownership validation | ARCH-0003 |
| Event Drift | Domain events originate from exactly one Implementation Domain | Event registry validation; event publisher analysis | ARCH-0003 |
| Plugin Drift | Plugins communicate exclusively through approved extension contracts | Plugin registration validation; runtime compatibility verification | ARCH-0003 |

## Enforcement Priority

1. Compile-Time
2. Repository-Time
3. Runtime
4. Manual architectural review

---

# Authority Coverage Matrix

## Implementation Responsibility → Governing Authority

| Implementation Responsibility | Governing Authority |
|-------------------------------|---------------------|
| Presentation Domain ownership | ARCH-0002 — Component Model |
| Presentation Layer responsibilities | TDS-0001 — System Architecture |
| Presentation dependency contract | ARCH-0003 — Architecture Enforcement Specification |
| Presentation workspace category | ARCH-0004 — Workspace Specification |
| Frontend responsibilities | TDR-0002 — Desktop Framework |
| IPC serialization strategy | TDR-0004 — IPC Serialization Strategy |
| Frontend Runtime responsibilities | ARCH-0001 — System Context |
| Trust Boundary TB-1 (User Interface) | ARCH-0001 — System Context |
| Trust Boundary TB-2 (IPC Boundary) | ARCH-0001 — System Context |
| Application Service pattern | ISP-0001 — Application Service Pattern |
| Command Handler pattern | ISP-0002 — Command Handler Pattern |
| Error Handling pattern | ISP-0008 — Error Handling Pattern |
| Dependency Injection pattern | ISP-0007 — Dependency Injection Pattern |
| Testing pattern | ISP-0009 — Testing Pattern |
| Vertical Slice pattern | ISP-0010 — Vertical Slice Pattern |
| Create Organization domain contract | MILESTONE-001-DOMAIN-DECISIONS |
| Create Organization domain implementation | MILESTONE-001.5; MILESTONE-001.5.2; MILESTONE-001.5.3 |
| Create Organization application implementation | MILESTONE-001.6 |
| Organization infrastructure implementation | MILESTONE-001.7 |
| Organization platform implementation | MILESTONE-001.8 |
| Organization ID generation | TDR-0006 — Organization ID Generation |
| Organization type decision | ORGANIZATION-TYPE-DECISION |
| Implementation baseline | MILESTONE-001-IMPLEMENTATION-BASELINE |
| Create Organization vertical slice plan | MILESTONE-001-CREATE-ORGANIZATION |

---

# Modified File Traceability

## New Files

| File | Responsibility | Governing Authority | Reason |
|------|----------------|---------------------|--------|
| `implementation/rust/presentation/Cargo.toml` | Crate manifest | ARCH-0004; TDR-0002 | New Presentation crate workspace member |
| `implementation/rust/presentation/src/lib.rs` | Crate root and public API | ARCH-0002; ISP-0010 | Presentation Domain crate entry point |
| `implementation/rust/presentation/src/ui.rs` | UI composition and rendering | ARCH-0002; TDR-0002 | Frontend responsibilities per TDR-0002 |
| `implementation/rust/presentation/src/view_model.rs` | View model for Create Organization form | ARCH-0002; TDR-0002 | Internal components per ARCH-0002 |
| `implementation/rust/presentation/src/ipc.rs` | IPC integration — `createOrganization` dispatch | ARCH-0002; TDR-0002; TDR-0004 | IPC rules per TDR-0002; DTO contract per TDR-0004 |
| `implementation/rust/presentation/src/errors.rs` | Presentation-layer error types | ISP-0008; TDR-0004 | Error handling per ISP-0008; safe error DTOs per TDR-0004 |
| `implementation/rust/presentation/src/composition.rs` | Presentation composition root | ISP-0007; ARCH-0003 | DI pattern per ISP-0007; dependency contract per ARCH-0003 |
| `implementation/rust/presentation/tests/presentation_test.rs` | Presentation integration tests | ISP-0009; ISP-0010 | Testing pattern per ISP-0009; vertical slice per ISP-0010 |

## Existing Files (Consumed, Not Modified)

| File | Responsibility | Governing Authority | Reason |
|------|----------------|---------------------|--------|
| `implementation/rust/platform/desktop/src/commands.rs` | `createOrganization` Tauri command | MILESTONE-001.8; TDR-0004 | Presentation invokes this command via IPC |
| `implementation/rust/platform/desktop/src/dtos.rs` | IPC DTOs | MILESTONE-001.8; TDR-0004 | Presentation consumes these DTOs |
| `implementation/rust/platform/desktop/src/errors.rs` | Error translation | MILESTONE-001.8; ISP-0008 | Presentation consumes error DTOs |
| `implementation/rust/platform/desktop/src/composition.rs` | Platform composition root | MILESTONE-001.8; ISP-0007 | Presentation depends on Platform composition |
| `implementation/rust/platform/desktop/src/lib.rs` | Platform crate root | MILESTONE-001.8 | Presentation depends on Platform crate |
| `implementation/rust/Cargo.toml` | Workspace manifest | ARCH-0004 | Presentation crate registered as workspace member |

---

# Document Completion

This document is complete.

It establishes the implementation contract for MILESTONE-001.9 — Organization Presentation Layer and serves as the scope definition for the Create Organization vertical slice's Presentation Layer.

It introduces no new architecture, no new technology decisions, no RFC, no TDS, no TDR, no ARCH, and no ISP.

All scope is derived from the approved authority documents listed in the Authority Coverage Matrix.

---

# Validation

Run:

```bash
git diff --check
```

Verify:

- only the milestone document `docs/implementation/MILESTONE-001.9-ORGANIZATION-PRESENTATION.md` changed
- no whitespace issues

---

# Repository Readiness

The repository is ready for Phase 4 — Milestone 1.9 Implementation.

The following prerequisites are satisfied:

- MILESTONE-001.5 — Organization Domain: Complete
- MILESTONE-001.6 — Create Organization Application: Complete
- MILESTONE-001.7 — Organization Infrastructure: Complete
- MILESTONE-001.8 — Organization Platform: Complete

The Platform Layer (`forgeos-desktop-platform`) already provides the `createOrganization` Tauri command, IPC DTOs, error translation, and composition root. The Presentation Layer milestone builds upon these existing integration points without modifying them.

The frontend framework selection remains deferred per `TDR-0002` Future Considerations. This milestone does not introduce a frontend framework decision.
