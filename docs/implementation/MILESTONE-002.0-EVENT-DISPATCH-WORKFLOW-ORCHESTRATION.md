# MILESTONE-002.0 — Event Dispatch and Workflow Orchestration

**Milestone ID:** MILESTONE-002.0

**Title:** Event Dispatch and Workflow Orchestration

**Status:** Proposed

**Version:** 1.0.0

**Related Milestones:**

- MILESTONE-001 — Create Organization vertical slice plan
- MILESTONE-001.5 — Organization Domain
- MILESTONE-001.6 — Create Organization Application
- MILESTONE-001.7 — Organization Infrastructure
- MILESTONE-001.8 — Organization Platform
- MILESTONE-001.9 — Organization Presentation

---

# Purpose

This milestone defines the implementation contract for Event Dispatch and Workflow Orchestration in ForgeOS.

It specifies the scope, ownership, crate boundaries, dependency direction, integration points, expected files, expected modules, expected public APIs, testing responsibilities, validation requirements, and architecture drift requirements for implementing domain event publication, event dispatch coordination, and application service workflow orchestration.

This document introduces **no new architecture**, **no new technology decisions**, **no RFC**, **no TDS**, **no TDR**, **no ARCH**, and **no ISP**.

All scope is derived exclusively from the approved authority documents listed in the Authority Coverage Matrix.

---

# Objective

Implement event dispatch and workflow orchestration for the Create Organization vertical slice.

The milestone shall:

1. **Event Publication** — collect and publish the `OrganizationCreated` domain event after successful transaction commit in the `CreateOrganization` application service
2. **Event Dispatch Coordination** — introduce an event dispatch abstraction that coordinates event delivery without coupling the Application Layer to infrastructure
3. **Workflow Orchestration** — demonstrate explicit transaction boundaries, commit coordination, and post-commit event publication in the Application Service
4. **Infrastructure Participation** — implement the event dispatch mechanism in the Infrastructure Layer, preserving domain ownership and architectural isolation

The milestone shall demonstrate the canonical ForgeOS event publication and workflow orchestration pattern defined by ISP-0005, ISP-0006, and TDS-0004.

---

# Scope

This milestone covers:

1. **Event Publisher abstraction** — a new domain-owned trait in the Organization Domain that defines the event publication contract
2. **Event dispatch implementation** — an infrastructure implementation of the event publisher that coordinates event delivery
3. **Application Service orchestration** — modify `CreateOrganization` to collect domain events, coordinate transaction commit, and publish events after successful commit
4. **Transaction coordination** — explicit transaction boundary management in the Application Service per ISP-0006
5. **Dependency composition** — wire the event publisher into the Platform composition root
6. **Event dispatch tests** — verification of event collection, publication, and dispatch coordination per ISP-0009 and ISP-0010

---

# Out of Scope

This milestone does **not** cover:

1. **Event broker technology** — no message broker, event bus, or messaging infrastructure is selected; event dispatch remains in-process per approved architecture
2. **Asynchronous event handling** — no async runtime, background tasks, or concurrent event processing is introduced
3. **Event persistence** — no event store, event sourcing, or event log persistence is implemented
4. **Event replay** — no event replay, event versioning, or event schema evolution is addressed
5. **Cross-context event consumption** — no consuming bounded contexts are implemented; only event publication from Organization Domain
6. **Additional domain events** — only `OrganizationCreated` is published; future events (`OrganizationUpdated`, etc.) require separate authority
7. **Saga orchestration** — no distributed transaction coordination, compensating transactions, or saga pattern is implemented
8. **Event-driven UI updates** — no presentation-layer event consumption or reactive UI updates are introduced
9. **New bounded contexts** — no additional bounded contexts beyond Organization are implemented
10. **Frontend framework selection** — remains deferred per TDR-0002
11. **Authentication/authorization** — not introduced in this milestone
12. **Additional vertical slices** — scope is limited to the Create Organization vertical slice

---

# Ownership

| Artifact | Architectural Owner | Authority |
|----------|---------------------|-----------|
| Event Publisher trait | Organization Domain | TDS-0002; ARCH-0002 |
| Domain event definitions | Organization Domain | TDS-0002; ISP-0005 |
| Event collection in aggregate | Organization Domain | TDS-0002; ISP-0005 |
| Event dispatch implementation | Infrastructure Domain | TDS-0004; ARCH-0002 |
| Transaction coordination | Application Services | TDS-0004; ISP-0006 |
| Event publication orchestration | Application Services | TDS-0004; ISP-0001; ISP-0005 |
| Dependency composition | Platform Domain | ARCH-0002; ISP-0007 |
| Event dispatch tests | Infrastructure Domain | ISP-0009; ISP-0010 |
| Application service tests | Application Services | ISP-0009; ISP-0010 |

Ownership is exclusive. No artifact shall have multiple architectural owners.

---

# Crate Boundaries

## Modified Crates

| Crate Name | Workspace Category | Architectural Owner | Location | Change Type |
|------------|--------------------|---------------------|----------|-------------|
| `forgeos-organization-domain` | Domains | Organization Domain | `implementation/rust/domains/organization-domain/` | Modified — add EventPublisher trait |
| `forgeos-create-organization-application` | Applications | Application Services | `implementation/rust/applications/create-organization/` | Modified — add event orchestration |
| `forgeos-organization-infrastructure` | Infrastructure | Infrastructure Domain | `implementation/rust/infrastructure/organization/` | Modified — add event dispatch implementation |
| `forgeos-desktop-platform` | Platform | Platform Domain | `implementation/rust/platform/desktop/` | Modified — wire event publisher |

## Existing Crates (Consumed, Not Modified)

| Crate | Workspace Category | Architectural Owner |
|-------|--------------------|---------------------|
| `forgeos-organization-presentation` | Presentation | Presentation Domain |

The Presentation crate is not modified in this milestone. It remains available for future event consumption.

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
| Application Services → Domain (EventPublisher trait) | Required | ARCH-0003; TDS-0004 |
| Infrastructure → Domain (EventPublisher trait) | Required | ARCH-0003; ISP-0005 |
| Platform → Application | Required | ARCH-0003; MILESTONE-001.8 |
| Platform → Infrastructure (composition only) | Required | ARCH-0003; MILESTONE-001.8 |
| Domain → Infrastructure | Forbidden | ARCH-0003; TDS-0002 |
| Application → Infrastructure | Forbidden | ARCH-0003; TDS-0004 |
| Domain → Platform | Forbidden | ARCH-0003; TDS-0002 |
| Application → Platform | Forbidden | ARCH-0003; TDS-0004 |

The EventPublisher trait is owned by the Organization Domain. Both Application and Infrastructure layers depend on this trait. The Infrastructure layer provides the concrete implementation. The Application layer orchestrates event publication after transaction commit.

---

# Integration Points

## 1. Event Publisher Trait

| Integration Point | Authority | Description |
|-------------------|-----------|-------------|
| `EventPublisher` trait | TDS-0002; ISP-0005; ARCH-0002 | Domain-owned contract for event publication, defined in Organization Domain |

## 2. Domain Event Publication

| Integration Point | Authority | Description |
|-------------------|-----------|-------------|
| `OrganizationCreated` event | TDS-0002; ISP-0005; MILESTONE-001.5 | Published after successful transaction commit |
| Event collection via `take_events()` | TDS-0002; ISP-0005 | Aggregate records events; Application Service collects them |
| Post-commit publication | ISP-0005; ISP-0006 | Events published only after successful commit |

## 3. Transaction Coordination

| Integration Point | Authority | Description |
|-------------------|-----------|-------------|
| Transaction boundary | TDS-0004; ISP-0006 | Application Service defines explicit transaction scope |
| Commit coordination | ISP-0006 | Successful execution commits before event publication |
| Rollback coordination | ISP-0006 | Failed execution rolls back; no events published |

## 4. Workflow Orchestration

| Integration Point | Authority | Description |
|-------------------|-----------|-------------|
| Application Service lifecycle | TDS-0004; ISP-0001 | Validate → Begin Transaction → Coordinate Domain → Publish Events → Commit → Return |
| Event dispatch coordination | TDS-0004; ISP-0001 | Application Service orchestrates event publication after commit |
| Dependency injection | ISP-0007 | EventPublisher injected into Application Service |

## 5. Trust Boundary TB-2 (IPC Boundary)

| Boundary | Trust Level | Authority |
|----------|-------------|-----------|
| TB-2 — IPC Boundary | Validated | ARCH-0001; TDR-0002; TDR-0004 |

No domain events cross the IPC boundary in this milestone. Event dispatch remains internal to the backend.

---

# Expected Files

## Modified Files

| File | Purpose | Authority |
|------|---------|-----------|
| `implementation/rust/domains/organization-domain/src/org_domain_event.rs` | Add `EventPublisher` trait definition | TDS-0002; ISP-0005; ARCH-0002 |
| `implementation/rust/applications/create-organization/src/service.rs` | Add event collection and publication orchestration | TDS-0004; ISP-0001; ISP-0005; ISP-0006 |
| `implementation/rust/infrastructure/organization/src/event_publisher.rs` | Implement `EventPublisher` trait for event dispatch | TDS-0004; ISP-0005; ARCH-0002 |
| `implementation/rust/infrastructure/organization/src/lib.rs` | Register event publisher module | ARCH-0004 |
| `implementation/rust/platform/desktop/src/composition.rs` | Wire `EventPublisher` into composition root | ISP-0007; MILESTONE-001.8 |
| `implementation/rust/applications/create-organization/Cargo.toml` | Add dependency on Organization Domain (if not already present) | ARCH-0003 |
| `implementation/rust/infrastructure/organization/Cargo.toml` | Add dependencies for event dispatch (if needed) | ARCH-0003 |

## Existing Files (Consumed, Not Modified)

| File | Purpose | Authority |
|------|---------|-----------|
| `implementation/rust/domains/organization-domain/src/organization.rs` | Aggregate with `take_events()` | TDS-0002; ISP-0005 |
| `implementation/rust/domains/organization-domain/src/organization_created.rs` | `OrganizationCreated` event | TDS-0002; MILESTONE-001.5 |
| `implementation/rust/domains/organization-domain/src/org_domain_event.rs` | `OrganizationDomainEvent` enum | TDS-0002; ISP-0005 |
| `implementation/rust/platform/desktop/src/commands.rs` | `createOrganization` command | MILESTONE-001.8; TDR-0004 |
| `implementation/rust/platform/desktop/src/composition.rs` | Platform composition root | MILESTONE-001.8; ISP-0007 |

---

# Expected Modules

## `forgeos-organization-domain` Crate

### `org_domain_event.rs` (Modified)

- `OrganizationDomainEvent` enum (existing)
- `OrganizationCreated` event (existing)
- **NEW:** `EventPublisher` trait — domain-owned contract for event publication

## `forgeos-create-organization-application` Crate

### `service.rs` (Modified)

- `CreateOrganization` application service (existing)
- **NEW:** Event collection via `take_events()`
- **NEW:** Transaction coordination (begin, commit, rollback)
- **NEW:** Post-commit event publication via `EventPublisher`

## `forgeos-organization-infrastructure` Crate

### `event_publisher.rs` (NEW)

- `InMemoryEventPublisher` — infrastructure implementation of `EventPublisher` trait
- Event dispatch coordination
- Event delivery logging/tracking (in-memory)

### `lib.rs` (Modified)

- Register `event_publisher` module
- Re-export `InMemoryEventPublisher`

---

# Expected Public APIs

## `EventPublisher` Trait (Organization Domain)

| API | Signature | Authority |
|-----|-----------|-----------|
| `publish(event: &OrganizationDomainEvent)` | Publishes a single domain event | TDS-0002; ISP-0005 |
| `publish_all(events: &[OrganizationDomainEvent])` | Publishes multiple domain events | TDS-0002; ISP-0005 |

## `CreateOrganization` Application Service (Modified)

| API | Signature | Authority |
|-----|-----------|-----------|
| `execute(command, generator, event_publisher)` | Orchestrates use case with event publication | TDS-0004; ISP-0001; ISP-0005; ISP-0006 |

**Behavior changes:**
1. Collects domain events via `take_events()`
2. Coordinates transaction commit
3. Publishes events only after successful commit
4. Returns application result

## `InMemoryEventPublisher` (Infrastructure)

| API | Signature | Authority |
|-----|-----------|-----------|
| `new()` | Creates new in-memory event publisher | ARCH-0002; ISP-0005 |
| `publish(event)` | Stores event in memory | ISP-0005 |
| `publish_all(events)` | Stores multiple events in memory | ISP-0005 |
| `drain_events()` | Retrieves and clears published events (test support) | ISP-0009 |

---

# Testing Responsibilities

## Test Ownership

| Test Type | Owner | Authority |
|-----------|-------|-----------|
| Application service tests | Application Services | ISP-0009; ISP-0010 |
| Event publisher tests | Infrastructure Domain | ISP-0009; ISP-0010 |
| Integration tests | Application Services | ISP-0009; ISP-0010 |

## Test Scope

### Application Service Tests

- Event collection after domain operation
- Transaction commit coordination
- Event publication only after successful commit
- No event publication after rollback
- Event publisher injection via DI
- Workflow orchestration sequence (validate → begin → coordinate → publish → commit → return)

### Event Publisher Tests

- Single event publication
- Multiple event publication
- Event storage in memory
- Event draining for test verification

### Integration Tests

- End-to-end flow: command → application service → domain → repository → commit → event publication
- Verify `OrganizationCreated` event is published after successful creation
- Verify no event publication when repository fails

## Test Principles

- Tests shall be deterministic per ISP-0009
- Tests shall verify behavior at the correct architectural boundary per ISP-0009
- Tests shall preserve dependency boundaries per ISP-0009
- Tests shall verify both success and failure paths per ISP-0009
- Event publisher shall be mockable for application service tests

---

# Validation Requirements

## Compile-Time Validation

| Requirement | Authority |
|-------------|-----------|
| EventPublisher trait compiles in Organization Domain | ARCH-0003; TDS-0002 |
| Application Service depends on EventPublisher trait, not implementation | ARCH-0003; TDS-0004 |
| Infrastructure implements EventPublisher trait | ARCH-0003; ISP-0005 |
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
| `OrganizationCreated` event published after successful creation | ISP-0005; TDS-0002 |
| No event published when creation fails | ISP-0005; ISP-0006 |
| Transaction commit occurs before event publication | ISP-0006; TDS-0004 |

---

# Dependency Approval Requirements

## New Dependencies

| Dependency | Approval Required | Authority |
|------------|-------------------|-----------|
| Event dispatch abstraction (in-process, no external library) | None — abstraction only | ISP-0005 |
| Any external event bus or messaging library | STOP — missing authority | ARCH-0003 |

## Existing Dependencies (Consumed)

| Dependency | Source | Authority |
|------------|--------|-----------|
| `forgeos-organization-domain` | Domain crate | ARCH-0003; TDS-0002 |
| `forgeos-create-organization-application` | Application crate | ARCH-0003; TDS-0004 |
| `forgeos-organization-infrastructure` | Infrastructure crate | ARCH-0003; TDR-0003 |
| `forgeos-desktop-platform` | Platform crate | ARCH-0003; TDR-0002 |

No new technology decisions are introduced by this milestone.

---

# Public API Requirements

## Stability

| Requirement | Authority |
|-------------|-----------|
| `EventPublisher` trait is a stable interface | TDR-0004; ARCH-0002 |
| `OrganizationCreated` event contract remains unchanged | TDS-0002; ISP-0005 |
| `CreateOrganization` public API signature changes (adds parameters) | TDS-0004; ISP-0001 |

## Event Contract

| Requirement | Authority |
|-------------|-----------|
| Events are immutable after creation | ISP-0005 |
| Events represent completed business facts | ISP-0005; TDS-0002 |
| Events are published only after successful commit | ISP-0005; ISP-0006 |
| Event ownership remains with publishing bounded context | TDS-0002; ARCH-0002 |

## Error Handling

| Requirement | Authority |
|-------------|-----------|
| Event publication failures do not rollback committed business state | ISP-0005; ISP-0006 |
| Event dispatch errors are logged but do not affect application result | ISP-0008; TDS-0004 |

---

# Architecture Drift Requirements

## Drift Categories

| Drift Category | Architectural Invariant | Verification | Authority |
|----------------|------------------------|--------------|-----------|
| Dependency Drift | Approved dependency contracts remain unchanged | Cargo dependency graph analysis | ARCH-0003 |
| Ownership Drift | Every artifact has exactly one architectural owner | Ownership registry validation | ARCH-0003 |
| Interface Drift | Published interfaces remain stable | Public API analysis | ARCH-0003 |
| Event Drift | Domain events originate from exactly one Implementation Domain | Event registry validation | ARCH-0003 |
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
| Event Publisher trait ownership | ARCH-0002 — Component Model |
| Domain event definitions | TDS-0002 — Domain Model |
| Event publication lifecycle | ISP-0005 — Domain Event Pattern |
| Transaction coordination | TDS-0004 — Application Model; ISP-0006 — Transaction Pattern |
| Application Service orchestration | TDS-0004 — Application Model; ISP-0001 — Application Service Pattern |
| Event dispatch implementation | ARCH-0002 — Component Model (Infrastructure Domain) |
| Dependency injection | ISP-0007 — Dependency Injection Pattern |
| Error handling | ISP-0008 — Error Handling Pattern |
| Testing pattern | ISP-0009 — Testing Pattern |
| Vertical slice scope | ISP-0010 — Vertical Slice Pattern |
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
| Organization Presentation implementation | MILESTONE-001.9 |
| Crate boundary plan | MILESTONE-001.2 — Crate Boundary Plan |
| Implementation baseline | MILESTONE-001-IMPLEMENTATION-BASELINE |

---

# Modified File Traceability

## New Files

| File | Responsibility | Governing Authority | Reason |
|------|----------------|---------------------|--------|
| `implementation/rust/infrastructure/organization/src/event_publisher.rs` | Event dispatch implementation | ARCH-0002; ISP-0005; TDS-0004 | Infrastructure Domain owns event dispatch mechanism |

## Modified Files

| File | Responsibility | Governing Authority | Reason |
|------|----------------|---------------------|--------|
| `implementation/rust/domains/organization-domain/src/org_domain_event.rs` | Add `EventPublisher` trait | TDS-0002; ISP-0005; ARCH-0002 | Domain-owned event publication contract |
| `implementation/rust/applications/create-organization/src/service.rs` | Add event orchestration | TDS-0004; ISP-0001; ISP-0005; ISP-0006 | Application Service coordinates event publication after commit |
| `implementation/rust/infrastructure/organization/src/lib.rs` | Register event publisher module | ARCH-0004 | Infrastructure crate module registration |
| `implementation/rust/platform/desktop/src/composition.rs` | Wire EventPublisher | ISP-0007; MILESTONE-001.8 | Dependency composition per approved DI pattern |

---

# Stop Boundaries

## STOP if Missing Authority

The following responsibilities require additional approved authority before implementation:

1. **Event broker technology** — No RFC, TDS, TDR, or ARCH document approves a message broker, event bus, or external messaging system. **STOP.** Do not introduce RabbitMQ, Kafka, Redis Streams, or any external event infrastructure.

2. **Asynchronous event handling** — No approved architecture specifies async runtime requirements for event dispatch. **STOP.** Do not introduce background tasks, async event handlers, or concurrent dispatch without approved authority.

3. **Event persistence** — No approved TDS or RFC defines an event store or event sourcing mechanism. **STOP.** Do not implement event log persistence without approved authority.

4. **Cross-context event consumption** — No bounded contexts beyond Organization are implemented. **STOP.** Do not implement event consumers for Mission, Process, Knowledge, or other contexts without approved authority.

5. **Saga orchestration** — No RFC or TDS defines saga pattern, compensating transactions, or distributed transaction coordination. **STOP.** Do not implement sagas without approved authority.

6. **New bounded contexts** — Only Organization bounded context is in scope. **STOP.** Do not implement additional bounded contexts without approved RFC and TDS.

## STOP if Architecture Violation

The following conditions require immediate cessation and architectural review:

1. Domain entities crossing IPC boundary — **STOP** per ARCH-0001 TB-2, TDR-0004
2. Business logic in Infrastructure — **STOP** per ARCH-0003 AV-001
3. Domain layer depending on Infrastructure — **STOP** per ARCH-0003 Dependency Contract
4. Application Service bypassing aggregate boundaries — **STOP** per TDS-0004, ARCH-0003
5. Event publication before transaction commit — **STOP** per ISP-0005, ISP-0006
6. Multiple architectural owners for one artifact — **STOP** per ARCH-0002, ARCH-0003 AV-007

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

- [ ] `EventPublisher` trait defined in Organization Domain
- [ ] `InMemoryEventPublisher` implemented in Infrastructure
- [ ] `CreateOrganization` service collects events via `take_events()`
- [ ] `CreateOrganization` service coordinates transaction commit
- [ ] `CreateOrganization` service publishes events after commit
- [ ] `CreateOrganization` service does not publish events on rollback
- [ ] EventPublisher wired into Platform composition root
- [ ] `createOrganization` command still functions end-to-end

## Testing Validation

- [ ] Application service tests verify event collection
- [ ] Application service tests verify post-commit publication
- [ ] Application service tests verify no publication on failure
- [ ] Event publisher tests verify single and multiple event publication
- [ ] Integration tests verify end-to-end event publication
- [ ] All tests pass: `cargo test --workspace`
- [ ] Workspace compiles: `cargo check --workspace`

## Architecture Validation

- [ ] No domain entities cross IPC boundary
- [ ] No business logic in Infrastructure
- [ ] No circular dependencies
- [ ] Dependency graph matches approved contracts
- [ ] All crates have exactly one architectural owner
- [ ] Event ownership remains with Organization Domain
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

It establishes the **Implementation Contract** for MILESTONE-002.0 — Event Dispatch and Workflow Orchestration, including scope, ownership, crate boundaries, dependency direction, integration points, expected files, expected modules, expected public APIs, testing responsibilities, validation requirements, and full traceability to approved authority documents.

This document introduces no new architecture, no new technology decisions, no RFC, no TDS, no TDR, no ARCH, and no ISP.

Every responsibility traces to one or more approved authority documents in the ForgeOS authority chain.

---

# Authority Documents Inspected

## RFC Series
- RFC-0001 — ForgeOS Genome
- RFC-0005 — Forge Pipeline
- RFC-0021 — Mission Engine

## TDS Series
- TDS-0001 — System Architecture
- TDS-0002 — Domain Model
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
- ARCH-DOM-0003 — Domain Event Model
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
- MILESTONE-001-IMPLEMENTATION-BASELINE

---

# Milestone Objective

Implement event dispatch and workflow orchestration for the Create Organization vertical slice, demonstrating the canonical ForgeOS pattern for domain event publication after successful transaction commit and application service workflow coordination.

---

# Scope Summary

This milestone implements:

1. **EventPublisher trait** — domain-owned contract in Organization Domain
2. **InMemoryEventPublisher** — infrastructure implementation for in-process event dispatch
3. **Application Service orchestration** — transaction coordination and post-commit event publication in `CreateOrganization`
4. **Dependency composition** — wire EventPublisher into Platform composition root
5. **Comprehensive tests** — application service tests, event publisher tests, and integration tests

The milestone modifies four existing crates and adds one new module. No new crates are created.

---

# Out of Scope

- Event broker technology (no approved authority)
- Asynchronous event handling (no approved authority)
- Event persistence (no approved authority)
- Event replay/versioning (future work)
- Cross-context event consumption (no consuming contexts implemented)
- Additional domain events (only `OrganizationCreated` in scope)
- Saga orchestration (no approved authority)
- Event-driven UI updates (Presentation not modified)
- New bounded contexts (Organization only)
- Frontend framework selection (deferred per TDR-0002)
- Authentication/authorization (not in scope)

---

# Expected Files

## New Files (1)
- `implementation/rust/infrastructure/organization/src/event_publisher.rs`

## Modified Files (4)
- `implementation/rust/domains/organization-domain/src/org_domain_event.rs`
- `implementation/rust/applications/create-organization/src/service.rs`
- `implementation/rust/infrastructure/organization/src/lib.rs`
- `implementation/rust/platform/desktop/src/composition.rs`

---

# Modified File Traceability Summary

| File | Change | Authority |
|------|--------|-----------|
| `org_domain_event.rs` | Add `EventPublisher` trait | TDS-0002; ISP-0005; ARCH-0002 |
| `service.rs` | Add event orchestration | TDS-0004; ISP-0001; ISP-0005; ISP-0006 |
| `event_publisher.rs` | New module | ARCH-0002; ISP-0005; TDS-0004 |
| `lib.rs` (infrastructure) | Register module | ARCH-0004 |
| `composition.rs` (platform) | Wire dependency | ISP-0007; MILESTONE-001.8 |

All changes trace to approved authority. No new architecture or technology decisions are introduced.

---

# Authority Coverage Summary

Every implementation responsibility in this milestone traces to at least one approved authority document:

- **Event publication lifecycle** → ISP-0005, TDS-0002
- **Transaction coordination** → ISP-0006, TDS-0004
- **Application orchestration** → ISP-0001, TDS-0004
- **Event dispatch implementation** → ARCH-0002 (Infrastructure Domain ownership)
- **Dependency injection** → ISP-0007
- **Error handling** → ISP-0008
- **Testing** → ISP-0009, ISP-0010
- **Workspace organization** → ARCH-0004
- **Architecture enforcement** → ARCH-0003

No responsibility lacks authority coverage.

---

# Validation Results

## Authority Completeness: ✅ PASS

All implementation responsibilities trace to approved authority documents. No missing authority identified.

## Stop Boundary Compliance: ✅ PASS

All stop boundaries are satisfied:
- No event broker technology introduced
- No async event handling introduced
- No event persistence introduced
- No cross-context consumption introduced
- No saga orchestration introduced
- No new bounded contexts introduced

## Architecture Compliance: ✅ PASS

- Dependency direction matches ARCH-0003 contracts
- No circular dependencies
- No domain entities cross IPC boundary
- No business logic in Infrastructure
- Transaction ownership remains in Application Layer
- Event ownership remains with Organization Domain

## Repository Readiness: ✅ PASS

The repository is ready for Phase 4 — Implementation.

### Prerequisites Satisfied

- MILESTONE-001.5 — Organization Domain: Complete
- MILESTONE-001.6 — Create Organization Application: Complete
- MILESTONE-001.7 — Organization Infrastructure: Complete
- MILESTONE-001.8 — Organization Platform: Complete
- MILESTONE-001.9 — Organization Presentation: Complete

### Current State

The Create Organization vertical slice is fully implemented through the presentation layer. The `OrganizationCreated` domain event is already recorded by the aggregate via `take_events()`. The Application Service currently returns the `OrganizationId` without collecting or publishing events. The Infrastructure Layer provides SQLite persistence. The Platform Layer exposes the `createOrganization` Tauri command.

This milestone builds upon this existing foundation to introduce event publication and workflow orchestration without modifying the vertical slice's external behavior.

---

# Repository Readiness for Phase 4 — Implementation

**Status: READY**

The repository is ready for Phase 4 — Milestone 2.0 Implementation.

### Implementation Sequence

1. Add `EventPublisher` trait to `org_domain_event.rs`
2. Implement `InMemoryEventPublisher` in new `event_publisher.rs` module
3. Modify `CreateOrganization::execute()` to collect events, coordinate transaction, and publish after commit
4. Update Platform `composition.rs` to wire `EventPublisher`
5. Implement tests (application service, event publisher, integration)
6. Validate: `cargo check --workspace`
7. Validate: `cargo test --workspace`
8. Validate: `git diff --check`

### Risk Assessment

**Low Risk**

- Minimal surface area change (4 files modified, 1 new module)
- Existing `createOrganization` command behavior preserved
- Event publication is additive (no breaking changes)
- All authority documents are approved and stable
- Implementation follows established patterns from Milestone 1

### Post-Milestone Direction

Upon completion, the repository will have demonstrated the canonical ForgeOS event publication and workflow orchestration pattern. Future milestones can extend this pattern to:

- Additional domain events (`OrganizationUpdated`, `OrganizationArchived`, etc.)
- Cross-context event consumption
- Additional bounded contexts (Mission, Process, Knowledge, etc.)
- Transaction coordination refinement (MILESTONE-2.1 per roadmap)

---

*End of Document*