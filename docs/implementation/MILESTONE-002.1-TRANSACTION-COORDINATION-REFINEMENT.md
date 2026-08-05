# MILESTONE-002.1 — Transaction Coordination Refinement

**Milestone ID:** MILESTONE-002.1

**Title:** Transaction Coordination Refinement

**Status:** Proposed

**Version:** 1.0.0

**Related Milestones:**

- MILESTONE-002.0 — Event Dispatch and Workflow Orchestration
- MILESTONE-001.6 — Create Organization Application
- MILESTONE-001.7 — Organization Infrastructure

---

# Purpose

This milestone defines the implementation contract for Transaction Coordination Refinement in ForgeOS.

It specifies the scope, ownership, crate boundaries, dependency direction, integration points, expected files, expected modules, expected public APIs, testing responsibilities, validation requirements, and architecture drift requirements for formalizing transaction coordination abstractions in the Application Layer.

This document introduces **no new architecture**, **no new technology decisions**, **no RFC**, **no TDS**, **no TDR**, **no ARCH**, and **no ISP**.

All scope is derived exclusively from the approved authority documents listed in the Authority Coverage Matrix.

---

# Objective

Formalize transaction coordination abstractions for the Create Organization vertical slice by introducing a reusable transaction abstraction that standardizes transaction lifecycle management across ForgeOS Application Services.

The milestone shall:

1. **Transaction Abstraction** — introduce a transaction trait/interface that defines the canonical transaction lifecycle (begin, commit, rollback)
2. **Application Service Integration** — refactor `CreateOrganization` to use the transaction abstraction instead of implicit transaction coordination
3. **Infrastructure Implementation** — provide a concrete transaction implementation that coordinates with the repository persistence mechanism
4. **Testing Infrastructure** — enable deterministic transaction testing through the abstraction
5. **Documentation** — document the transaction coordination pattern for future Application Services

The milestone shall demonstrate the canonical ForgeOS transaction coordination pattern defined by ISP-0006 and TDS-0004.

---

# Scope

This milestone covers:

1. **Transaction trait definition** — a new Application Layer trait that defines the transaction coordination contract
2. **Transaction implementation** — an infrastructure implementation of the transaction trait that coordinates with SQLx/SQLite
3. **Application Service refactoring** — modify `CreateOrganization` to use explicit transaction abstraction
4. **Dependency composition** — wire the transaction abstraction into the Platform composition root
5. **Transaction tests** — verification of transaction lifecycle, commit, and rollback behavior per ISP-0009 and ISP-0010

---

# Out of Scope

This milestone does **not** cover:

1. **Distributed transactions** — no two-phase commit, saga pattern, or cross-service transaction coordination
2. **Transaction middleware** — no decorator-based transaction interception or AOP transaction management
3. **Async transaction support** — no async/await transaction APIs (synchronous only per current architecture)
4. **Transaction events** — no transaction lifecycle events or transaction event sourcing
5. **Transaction monitoring** — no transaction metrics, tracing, or observability infrastructure
6. **Retry logic** — no automatic retry, circuit breaker, or resilience patterns for transactions
7. **Nested transactions** — no savepoints, nested transaction scopes, or transaction hierarchies
8. **Additional vertical slices** — scope is limited to the Create Organization vertical slice
9. **Additional bounded contexts** — no new bounded contexts beyond Organization
10. **Frontend framework selection** — remains deferred per TDR-0002
11. **Authentication/authorization** — not introduced in this milestone

---

# Ownership

| Artifact | Architectural Owner | Authority |
|----------|---------------------|-----------|
| Transaction trait | Application Services | TDS-0004; ISP-0006; ARCH-0002 |
| Transaction implementation | Infrastructure Domain | TDS-0004; ISP-0006; ARCH-0002 |
| Transaction coordination logic | Application Services | TDS-0004; ISP-0006 |
| Dependency composition | Platform Domain | ARCH-0002; ISP-0007 |
| Transaction tests | Application Services | ISP-0009; ISP-0010 |

Ownership is exclusive. No artifact shall have multiple architectural owners.

---

# Crate Boundaries

## Modified Crates

| Crate Name | Workspace Category | Architectural Owner | Location | Change Type |
|------------|--------------------|---------------------|----------|-------------|
| `forgeos-create-organization-application` | Applications | Application Services | `implementation/rust/applications/create-organization/` | Modified — add transaction trait and usage |
| `forgeos-organization-infrastructure` | Infrastructure | Infrastructure Domain | `implementation/rust/infrastructure/organization/` | Modified — add transaction implementation |
| `forgeos-desktop-platform` | Platform | Platform Domain | `implementation/rust/platform/desktop/` | Modified — wire transaction abstraction |

## Existing Crates (Consumed, Not Modified)

| Crate | Workspace Category | Architectural Owner |
|-------|--------------------|---------------------|
| `forgeos-organization-domain` | Domains | Organization Domain |
| `forgeos-organization-presentation` | Presentation | Presentation Domain |

The Domain and Presentation crates are not modified in this milestone. They remain available for future transaction participation.

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

## Milestone Dependency Contracts

| Dependency | Status | Authority |
|------------|--------|-----------|
| Application Services → Infrastructure (Transaction trait usage) | Required | ARCH-0003; TDS-0004 |
| Infrastructure → Application Services (Transaction trait implementation) | Required | ARCH-0003; ISP-0006 |
| Platform → Application | Required | ARCH-0003; MILESTONE-001.8 |
| Platform → Infrastructure (composition only) | Required | ARCH-0003; MILESTONE-001.8 |
| Domain → Application | Forbidden | ARCH-0003; TDS-0002 |
| Domain → Infrastructure | Forbidden | ARCH-0003; TDS-0002 |
| Domain → Platform | Forbidden | ARCH-0003; TDS-0002 |
| Application → Platform | Forbidden | ARCH-0003; TDS-0004 |

The Transaction trait is owned by the Application Layer (conceptually). The Infrastructure Layer provides the concrete implementation. The Application Layer uses the trait through dependency injection.

---

# Integration Points

## 1. Transaction Trait

| Integration Point | Authority | Description |
|-------------------|-----------|-------------|
| `Transaction` trait | TDS-0004; ISP-0006; ARCH-0002 | Application-owned contract for transaction lifecycle management |

## 2. Transaction Lifecycle

| Integration Point | Authority | Description |
|-------------------|-----------|-------------|
| `begin()` | ISP-0006 | Begin transaction boundary |
| `commit()` | ISP-0006 | Commit transaction after successful execution |
| `rollback()` | ISP-0006 | Rollback transaction on failure |
| `is_active()` | ISP-0006 | Query transaction state (optional) |

## 3. Application Service Integration

| Integration Point | Authority | Description |
|-------------------|-----------|-------------|
| Transaction injection | ISP-0007 | Transaction abstraction injected into Application Service |
| Transaction coordination | TDS-0004; ISP-0006 | Application Service orchestrates transaction lifecycle |
| Post-commit event publication | ISP-0005; ISP-0006 | Events published only after successful commit |

## 4. Infrastructure Implementation

| Integration Point | Authority | Description |
|-------------------|-----------|-------------|
| SQLx transaction support | TDR-0003; ISP-0006 | Concrete implementation uses SQLx transaction APIs |
| Repository participation | ISP-0004; ISP-0006 | Repositories participate in transaction without owning it |
| Connection management | TDR-0003 | Transaction manages database connection lifetime |

## 5. Trust Boundary TB-2 (IPC Boundary)

| Boundary | Trust Level | Authority |
|----------|-------------|-----------|
| TB-2 — IPC Boundary | Validated | ARCH-0001; TDR-0002; TDR-0004 |

No transaction details cross the IPC boundary. Transaction coordination remains internal to the backend.

---

# Expected Files

## Modified Files

| File | Purpose | Authority |
|------|---------|-----------|
| `implementation/rust/applications/create-organization/src/transaction.rs` | Add `Transaction` trait definition | TDS-0004; ISP-0006; ARCH-0002 |
| `implementation/rust/applications/create-organization/src/service.rs` | Refactor to use transaction abstraction | TDS-0004; ISP-0001; ISP-0006 |
| `implementation/rust/infrastructure/organization/src/transaction.rs` | Implement `Transaction` trait for SQLx/SQLite | TDS-0004; ISP-0006; ARCH-0002 |
| `implementation/rust/infrastructure/organization/src/lib.rs` | Register transaction module | ARCH-0004 |
| `implementation/rust/platform/desktop/src/composition.rs` | Wire transaction abstraction into composition root | ISP-0007; MILESTONE-001.8 |
| `implementation/rust/applications/create-organization/Cargo.toml` | Add dependency on Infrastructure (if not already present) | ARCH-0003 |

## Existing Files (Consumed, Not Modified)

| File | Purpose | Authority |
|------|---------|-----------|
| `implementation/rust/applications/create-organization/src/service.rs` | `CreateOrganization` application service (existing) | TDS-0004; MILESTONE-001.6 |
| `implementation/rust/infrastructure/organization/src/repository.rs` | Repository implementation | ISP-0004; MILESTONE-001.7 |
| `implementation/rust/platform/desktop/src/composition.rs` | Platform composition root | MILESTONE-001.8; ISP-0007 |
| `implementation/rust/domains/organization-domain/src/organization.rs` | Aggregate with `take_events()` | TDS-0002; ISP-0005 |

---

# Expected Modules

## `forgeos-create-organization-application` Crate

### `transaction.rs` (NEW)

- `Transaction` trait — Application-owned contract for transaction lifecycle
- `begin()` — begin transaction
- `commit()` — commit transaction
- `rollback()` — rollback transaction
- `is_active()` — query transaction state (optional)

### `service.rs` (Modified)

- `CreateOrganization` application service (existing)
- **MODIFIED:** Use `Transaction` trait instead of implicit transaction coordination
- **MODIFIED:** Inject `Transaction` via dependency injection

## `forgeos-organization-infrastructure` Crate

### `transaction.rs` (NEW)

- `SqlxTransaction` — infrastructure implementation of `Transaction` trait
- SQLx transaction coordination
- Connection lifecycle management

### `lib.rs` (Modified)

- Register `transaction` module
- Re-export `SqlxTransaction`

---

# Expected Public APIs

## `Transaction` Trait (Application Layer)

| API | Signature | Authority |
|-----|-----------|-----------|
| `begin(&mut self) -> Result<(), TransactionError>` | Begin transaction boundary | TDS-0004; ISP-0006 |
| `commit(&mut self) -> Result<(), TransactionError>` | Commit transaction | TDS-0004; ISP-0006 |
| `rollback(&mut self) -> Result<(), TransactionError>` | Rollback transaction | TDS-0004; ISP-0006 |
| `is_active(&self) -> bool` | Query if transaction is active (optional) | ISP-0006 |

## `CreateOrganization` Application Service (Modified)

| API | Signature | Authority |
|-----|-----------|-----------|
| `execute(command, generator, repository, transaction, event_publisher)` | Orchestrates use case with explicit transaction | TDS-0004; ISP-0001; ISP-0006 |

**Behavior changes:**
1. Accepts `&mut dyn Transaction` parameter
2. Calls `transaction.begin()` before domain operations
3. Calls `transaction.commit()` after successful persistence
4. Calls `transaction.rollback()` on failure
5. Publishes events only after successful commit

## `SqlxTransaction` (Infrastructure)

| API | Signature | Authority |
|-----|-----------|-----------|
| `new(connection: DbConnection)` | Creates new transaction wrapper | TDR-0003; ISP-0006 |
| `begin()` | Begins SQLx transaction | ISP-0006; TDR-0003 |
| `commit()` | Commits SQLx transaction | ISP-0006; TDR-0003 |
| `rollback()` | Rolls back SQLx transaction | ISP-0006; TDR-0003 |

---

# Testing Responsibilities

## Test Ownership

| Test Type | Owner | Authority |
|-----------|-------|-----------|
| Transaction trait tests | Application Services | ISP-0009; ISP-0010 |
| Transaction implementation tests | Infrastructure Domain | ISP-0009; ISP-0010 |
| Application service tests | Application Services | ISP-0009; ISP-0010 |
| Integration tests | Application Services | ISP-0009; ISP-0010 |

## Test Scope

### Transaction Trait Tests

- Trait interface contract verification
- Mock transaction implementation for testing

### Transaction Implementation Tests

- Begin transaction
- Commit transaction after successful operations
- Rollback transaction on failure
- Transaction state queries
- Connection lifecycle management
- Error handling and propagation

### Application Service Tests

- Transaction begin before domain operations
- Transaction commit after successful execution
- Transaction rollback on repository failure
- Event publication only after successful commit
- Transaction abstraction injection via DI
- Workflow orchestration sequence (begin → coordinate → commit → publish → return)

### Integration Tests

- End-to-end flow: command → application service → transaction → domain → repository → commit → event publication
- Verify transaction boundaries are respected
- Verify rollback behavior on failure
- Verify no event publication on rollback

## Test Principles

- Tests shall be deterministic per ISP-0009
- Tests shall verify behavior at the correct architectural boundary per ISP-0009
- Tests shall preserve dependency boundaries per ISP-0009
- Tests shall verify both success and failure paths per ISP-0009
- Transaction abstraction shall be mockable for application service tests

---

# Validation Requirements

## Compile-Time Validation

| Requirement | Authority |
|-------------|-----------|
| Transaction trait compiles in Application Layer | ARCH-0003; TDS-0004 |
| Application Service depends on Transaction trait, not implementation | ARCH-0003; TDS-0004 |
| Infrastructure implements Transaction trait | ARCH-0003; ISP-0006 |
| No domain entities cross architectural boundaries | ARCH-0003 AV-005; TDR-0004 |
| No business logic in Infrastructure | ARCH-0003 AV-001; TDS-0001 |

## Repository-Time Validation

| Requirement | Authority |
|-------------|-----------|
| `cargo check --workspace` passes | ARCH-0004 |
| `cargo test --workspace` passes | ISP-0009 |
| Dependency graph matches approved contracts | ARCH-0003 |
| No architectural drift detected | ARCH-0003 |

## Runtime Validation

| Requirement | Authority |
|-------------|-----------|
| `createOrganization` command still functions | TDR-0002; TDR-0004 |
| Transaction begins before domain operations | ISP-0006; TDS-0004 |
| Transaction commits after successful execution | ISP-0006; TDS-0004 |
| Transaction rolls back on failure | ISP-0006; TDS-0004 |
| Events published only after successful commit | ISP-0005; ISP-0006 |

---

# Dependency Approval Requirements

## New Dependencies

| Dependency | Approval Required | Authority |
|------------|-------------------|-----------|
| Transaction abstraction (trait definition) | None — abstraction only | ISP-0006 |
| SQLx transaction APIs (existing dependency) | None — already approved | TDR-0003 |

## Existing Dependencies (Consumed)

| Dependency | Source | Authority |
|------------|--------|-----------|
| `forgeos-organization-domain` | Domain crate | ARCH-0003; TDS-0002 |
| `forgeos-create-organization-application` | Application crate | ARCH-0003; TDS-0004 |
| `forgeos-organization-infrastructure` | Infrastructure crate | ARCH-0003; TDR-0003 |
| `forgeos-desktop-platform` | Platform crate | ARCH-0003; TDR-0002 |
| `sqlx` | Persistence library | TDR-0003 |

No new technology decisions are introduced by this milestone.

---

# Public API Requirements

## Stability

| Requirement | Authority |
|-------------|-----------|
| `Transaction` trait is a stable interface | TDR-0004; ARCH-0002 |
| `CreateOrganization` public API signature changes (adds parameters) | TDS-0004; ISP-0001 |
| `SqlxTransaction` is an internal implementation detail | ARCH-0002; ISP-0006 |

## Transaction Contract

| Requirement | Authority |
|-------------|-----------|
| Transaction lifecycle is explicit (begin, commit, rollback) | ISP-0006 |
| Transaction ownership remains in Application Layer | TDS-0004; ISP-0006 |
| Commit and rollback are mutually exclusive | ISP-0006 |
| Transaction implementation is replaceable | ISP-0006; ARCH-0002 |

## Error Handling

| Requirement | Authority |
|-------------|-----------|
| Transaction errors propagate to Application Service | ISP-0006; ISP-0008 |
| Transaction rollback errors are logged but do not prevent error propagation | ISP-0008 |
| Transaction failures result in application failure outcome | ISP-0006; TDS-0004 |

---

# Architecture Drift Requirements

## Drift Categories

| Drift Category | Architectural Invariant | Verification | Authority |
|----------------|------------------------|--------------|-----------|
| Dependency Drift | Approved dependency contracts remain unchanged | Cargo dependency graph analysis | ARCH-0003 |
| Ownership Drift | Every artifact has exactly one architectural owner | Ownership registry validation | ARCH-0003 |
| Interface Drift | Published interfaces remain stable | Public API analysis | ARCH-0003 |
| Transaction Drift | Transaction boundaries remain in Application Layer | Transaction ownership verification | ARCH-0003 |
| Repository Drift | Repository organization conforms to Workspace Specification | Repository layout validation | ARCH-0003; ARCH-0004 |

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
| Transaction trait ownership | ARCH-0002 — Component Model |
| Transaction lifecycle semantics | TDS-0004 — Application Model; ISP-0006 — Transaction Pattern |
| Application Service transaction coordination | TDS-0004 — Application Model; ISP-0001 — Application Service Pattern |
| Transaction implementation | ARCH-0002 — Component Model (Infrastructure Domain) |
| SQLx transaction APIs | TDR-0003 — Storage Strategy |
| Dependency injection | ISP-0007 — Dependency Injection Pattern |
| Error handling | ISP-0008 — Error Handling Pattern |
| Testing pattern | ISP-0009 — Testing Pattern; ISP-0010 — Vertical Slice Pattern |
| Workspace organization | ARCH-0004 — Workspace Specification |
| Architecture enforcement | ARCH-0003 — Architecture Enforcement Specification |
| Rust/Cargo toolchain | TDR-0001 — Programming Language |
| Desktop runtime | TDR-0002 — Desktop Framework |
| IPC serialization | TDR-0004 — IPC Serialization Strategy |
| Workspace location | TDR-0005 — Workspace Location Reconciliation |
| Organization ID generation | TDR-0006 — Organization ID Generation |
| Create Organization domain contract | MILESTONE-001-DOMAIN-DECISIONS |
| Organization Domain implementation | MILESTONE-001.5; MILESTONE-001.5.2; MILESTONE-001.5.3 |
| Create Organization Application implementation | MILESTONE-001.6 |
| Organization Infrastructure implementation | MILESTONE-001.7 |
| Organization Platform implementation | MILESTONE-001.8 |
| Event Dispatch and Workflow Orchestration | MILESTONE-002.0 |
| Crate boundary plan | MILESTONE-001.2 — Crate Boundary Plan |
| Implementation baseline | MILESTONE-001-IMPLEMENTATION-BASELINE |

---

# Modified File Traceability

## New Files

| File | Responsibility | Governing Authority | Reason |
|------|----------------|---------------------|--------|
| `implementation/rust/applications/create-organization/src/transaction.rs` | Transaction trait definition | TDS-0004; ISP-0006; ARCH-0002 | Application-owned transaction contract |
| `implementation/rust/infrastructure/organization/src/transaction.rs` | Transaction implementation | TDS-0004; ISP-0006; ARCH-0002 | Infrastructure Domain owns persistence mechanisms |

## Modified Files

| File | Responsibility | Governing Authority | Reason |
|------|----------------|---------------------|--------|
| `implementation/rust/applications/create-organization/src/service.rs` | Use transaction abstraction | TDS-0004; ISP-0001; ISP-0006 | Application Service coordinates transaction lifecycle |
| `implementation/rust/infrastructure/organization/src/lib.rs` | Register transaction module | ARCH-0004 | Infrastructure crate module registration |
| `implementation/rust/platform/desktop/src/composition.rs` | Wire transaction abstraction | ISP-0007; MILESTONE-001.8 | Dependency composition per approved DI pattern |

---

# Stop Boundaries

## STOP if Missing Authority

The following responsibilities require additional approved authority before implementation:

1. **Distributed transactions** — No RFC, TDS, or ARCH document approves two-phase commit, saga pattern, or cross-service transaction coordination. **STOP.** Do not implement distributed transactions without approved authority.

2. **Transaction middleware** — No approved architecture specifies decorator-based transaction interception or AOP patterns. **STOP.** Do not introduce transaction middleware without approved authority.

3. **Async transaction support** — No approved architecture specifies async/await transaction APIs. **STOP.** Do not introduce async transaction APIs without approved authority.

4. **Transaction events** — No approved RFC or TDS defines transaction lifecycle events or transaction event sourcing. **STOP.** Do not implement transaction events without approved authority.

5. **Nested transactions** — No approved architecture specifies savepoints or nested transaction scopes. **STOP.** Do not implement nested transactions without approved authority.

6. **New bounded contexts** — Only Organization bounded context is in scope. **STOP.** Do not implement additional bounded contexts without approved RFC and TDS.

## STOP if Architecture Violation

The following conditions require immediate cessation and architectural review:

1. Domain entities depending on transaction abstraction — **STOP** per ARCH-0003 Dependency Contract
2. Business logic in transaction implementation — **STOP** per ARCH-0003 AV-001
3. Transaction ownership moving to Infrastructure — **STOP** per TDS-0004, ISP-0006
4. Application Service bypassing transaction abstraction — **STOP** per TDS-0004, ISP-0006
5. Domain layer depending on transaction abstraction — **STOP** per ARCH-0003 Dependency Contract
6. Multiple architectural owners for transaction abstraction — **STOP** per ARCH-0002, ARCH-0003 AV-007

---

# Validation Checklist

## Pre-Implementation Validation

- [ ] All authority documents inspected and understood
- [ ] No missing authority identified for any responsibility
- [ ] Stop boundaries reviewed and accepted
- [ ] Dependency direction verified against ARCH-0003
- [ ] Crate boundaries verified against MILESTONE-001.2
- [ ] Workspace structure verified against ARCH-0004

## Implementation Validation

- [ ] `Transaction` trait defined in Application Layer
- [ ] `SqlxTransaction` implemented in Infrastructure
- [ ] `CreateOrganization` service uses transaction abstraction
- [ ] Transaction begin called before domain operations
- [ ] Transaction commit called after successful execution
- [ ] Transaction rollback called on failure
- [ ] Transaction wired into Platform composition root
- [ ] `createOrganization` command still functions end-to-end

## Testing Validation

- [ ] Transaction trait tests verify interface contract
- [ ] Transaction implementation tests verify lifecycle
- [ ] Transaction implementation tests verify commit behavior
- [ ] Transaction implementation tests verify rollback behavior
- [ ] Application service tests verify transaction coordination
- [ ] Application service tests verify post-commit event publication
- [ ] Application service tests verify no event publication on rollback
- [ ] Integration tests verify end-to-end transaction flow
- [ ] All tests pass: `cargo test --workspace`
- [ ] Workspace compiles: `cargo check --workspace`

## Architecture Validation

- [ ] No domain entities cross architectural boundaries
- [ ] No business logic in Infrastructure
- [ ] No circular dependencies
- [ ] Dependency graph matches approved contracts
- [ ] All crates have exactly one architectural owner
- [ ] Transaction ownership remains in Application Layer
- [ ] No new technology decisions introduced

## Documentation Validation

- [ ] Code comments reference governing authority
- [ ] Module documentation updated
- [ ] No undocumented architectural assumptions
- [ ] Milestone scope document completed

---

# Document Completion

This document is complete.

It establishes the **Implementation Contract** for MILESTONE-002.1 — Transaction Coordination Refinement, including scope, ownership, crate boundaries, dependency direction, integration points, expected files, expected modules, expected public APIs, testing responsibilities, validation requirements, and full traceability to approved authority documents.

This document introduces no new architecture, no new technology decisions, no RFC, no TDS, no TDR, no ARCH, and no ISP.

Every responsibility traces to one or more approved authority documents in the ForgeOS authority chain.

---

# Authority Documents Inspected

## RFC Series

- RFC-0001 — ForgeOS Genome
- RFC-0005 — Forge Pipeline
- RFC-0021 — Mission Engine
- RFC-0045 — Autonomous Organization Framework

## TDS Series

- TDS-0001 — System Architecture
- TDS-0002 — Domain Model
- TDS-0003 — Organization Model
- TDS-0004 — Application Model

## TDR Series

- TDR-0001 — Programming Language (Rust/Cargo)
- TDR-0002 — Desktop Framework (Tauri 2.x)
- TDR-0003 — Storage Strategy (SQLite/SQLx)
- TDR-0004 — IPC Serialization Strategy (Serde/JSON)
- TDR-0005 — Workspace Location Reconciliation
- TDR-0006 — Organization ID Generation (UUID v4)

## Architecture Documents

- ARCH-0001 — System Context
- ARCH-0002 — Component Model
- ARCH-0003 — Architecture Enforcement Specification
- ARCH-0004 — Workspace Specification
- ARCH-APP-0003 — Workflow Orchestration

## Implementation Specifications

- ISP-0001 — Application Service Pattern
- ISP-0002 — Command Handler Pattern
- ISP-0004 — Repository Pattern
- ISP-0005 — Domain Event Pattern
- ISP-0006 — Transaction Pattern
- ISP-0007 — Dependency Injection Pattern
- ISP-0008 — Error Handling Pattern
- ISP-0009 — Testing Pattern
- ISP-0010 — Vertical Slice Pattern

## Implementation Documents

- MILESTONE-001 — Create Organization Vertical Slice
- MILESTONE-001-DOMAIN-DECISIONS — Create Organization domain contract
- MILESTONE-001.2 — Crate Boundary Plan
- MILESTONE-001.5 — Organization Domain Foundation
- MILESTONE-001.5.2 — Organization Domain Implementation
- MILESTONE-001.5.3 — Organization Domain Test Validation
- MILESTONE-001.6 — Create Organization Application Layer
- MILESTONE-001.7 — Organization Infrastructure Layer
- MILESTONE-001.8 — Organization Platform Layer
- MILESTONE-001.9 — Organization Presentation Layer
- MILESTONE-002.0 — Event Dispatch and Workflow Orchestration
- MILESTONE-001-IMPLEMENTATION-BASELINE

---

# Milestone Objective

Formalize transaction coordination abstractions for the Create Organization vertical slice by introducing a reusable transaction abstraction that standardizes transaction lifecycle management across ForgeOS Application Services.

---

# Scope Summary

This milestone implements:

1. **Transaction trait** — Application-owned contract defining transaction lifecycle (begin, commit, rollback)
2. **SqlxTransaction implementation** — Infrastructure implementation coordinating with SQLx/SQLite
3. **Application Service refactoring** — `CreateOrganization` uses explicit transaction abstraction
4. **Dependency composition** — wire transaction abstraction into Platform composition root
5. **Comprehensive tests** — transaction tests, application service tests, and integration tests

The milestone modifies three existing crates and adds two new modules. No new crates are created.

---

# Out of Scope

- Distributed transactions (no approved authority)
- Transaction middleware (no approved authority)
- Async transaction support (no approved authority)
- Transaction events (no approved authority)
- Transaction monitoring/observability (no approved authority)
- Retry logic and resilience patterns (no approved authority)
- Nested transactions/savepoints (no approved authority)
- Additional vertical slices (Create Organization only)
- Additional bounded contexts (Organization only)
- Frontend framework selection (deferred per TDR-0002)
- Authentication/authorization (not in scope)

---

# Expected Files

## New Files (2)
- `implementation/rust/applications/create-organization/src/transaction.rs`
- `implementation/rust/infrastructure/organization/src/transaction.rs`

## Modified Files (3)
- `implementation/rust/applications/create-organization/src/service.rs`
- `implementation/rust/infrastructure/organization/src/lib.rs`
- `implementation/rust/platform/desktop/src/composition.rs`

---

# Modified File Traceability Summary

| File | Change | Authority |
|------|--------|-----------|
| `transaction.rs` (application) | New module — Transaction trait | TDS-0004; ISP-0006; ARCH-0002 |
| `transaction.rs` (infrastructure) | New module — SqlxTransaction implementation | TDS-0004; ISP-0006; ARCH-0002 |
| `service.rs` | Refactor to use transaction abstraction | TDS-0004; ISP-0001; ISP-0006 |
| `lib.rs` (infrastructure) | Register transaction module | ARCH-0004 |
| `composition.rs` (platform) | Wire transaction dependency | ISP-0007; MILESTONE-001.8 |

All changes trace to approved authority. No new architecture or technology decisions are introduced.

---

# Authority Coverage Summary

Every implementation responsibility in this milestone traces to at least one approved authority document:

- **Transaction lifecycle semantics** → ISP-0006, TDS-0004
- **Application Service orchestration** → ISP-0001, TDS-0004
- **Transaction implementation ownership** → ARCH-0002 (Infrastructure Domain)
- **SQLx transaction APIs** → TDR-0003
- **Dependency injection** → ISP-0007
- **Error handling** → ISP-0008
- **Testing** → ISP-0009, ISP-0010
- **Workspace organization** → ARCH-0004
- **Architecture enforcement** → ARCH-0003

No responsibility lacks authority coverage.

---

# Validation

## Authority Completeness: ✅ PASS

All implementation responsibilities trace to approved authority documents. No missing authority identified.

## Stop Boundary Compliance: ✅ PASS

All stop boundaries are satisfied:
- No distributed transactions introduced
- No transaction middleware introduced
- No async transaction support introduced
- No transaction events introduced
- No nested transactions introduced
- No new bounded contexts introduced

## Architecture Compliance: ✅ PASS

- Dependency direction matches ARCH-0003 contracts
- No circular dependencies
- No domain entities cross architectural boundaries
- No business logic in Infrastructure
- Transaction ownership remains in Application Layer
- Transaction abstraction is replaceable

## Repository Readiness: ✅ PASS

The repository is ready for Phase 4 — Implementation.

### Prerequisites Satisfied

- MILESTONE-001.5 — Organization Domain: Complete
- MILESTONE-001.6 — Create Organization Application: Complete
- MILESTONE-001.7 — Organization Infrastructure: Complete
- MILESTONE-001.8 — Organization Platform: Complete
- MILESTONE-002.0 — Event Dispatch and Workflow Orchestration: Complete

### Current State

The Create Organization vertical slice demonstrates event publication and workflow orchestration. The Application Service currently coordinates transaction implicitly through the repository. This milestone introduces an explicit transaction abstraction to standardize transaction lifecycle management and improve testability.

---

# Repository Readiness for Phase 4

**Status: READY**

The repository is ready for Phase 4 — Milestone 2.1 Implementation.

### Implementation Sequence

1. Add `Transaction` trait to `transaction.rs` in Application Layer
2. Implement `SqlxTransaction` in new `transaction.rs` module in Infrastructure
3. Refactor `CreateOrganization::execute()` to use transaction abstraction
4. Update Platform `composition.rs` to wire transaction abstraction
5. Implement tests (transaction tests, application service tests, integration tests)
6. Validate: `cargo check --workspace`
7. Validate: `cargo test --workspace`
8. Validate: `git diff --check`

### Risk Assessment

**Low Risk**

- Minimal surface area change (3 files modified, 2 new modules)
- Existing `createOrganization` command behavior preserved
- Transaction abstraction is additive (no breaking changes)
- All authority documents are approved and stable
- Implementation follows established patterns from Milestones 1 and 2

### Post-Milestone Direction

Upon completion, the repository will have a reusable transaction abstraction that standardizes transaction coordination across ForgeOS Application Services. Future milestones can extend this pattern to:

- Additional Application Services (Mission, Process, Knowledge contexts)
- Transaction coordination refinement for complex workflows
- Additional bounded contexts adopting the transaction pattern

---

*End of Document*