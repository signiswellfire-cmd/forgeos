# MILESTONE-003.0 — Governance Domain Foundation

**Milestone ID:** MILESTONE-003.0

**Title:** Governance Domain Foundation

**Status:** Proposed

**Version:** 1.0.0

**Related Milestones:**
- MILESTONE-001 — Create Organization vertical slice plan
- MILESTONE-001.5 — Organization Domain Foundation
- MILESTONE-001.6 — Create Organization Application
- MILESTONE-001.7 — Organization Infrastructure
- MILESTONE-001.8 — Organization Platform
- MILESTONE-001.9 — Organization Presentation
- MILESTONE-002.0 — Event Dispatch and Workflow Orchestration
- MILESTONE-002.1 — Transaction Coordination Refinement

---

# Purpose

This milestone defines the implementation contract for Governance Domain Foundation in ForgeOS.

It specifies the scope, ownership, crate boundaries, dependency direction, integration points, expected files, expected modules, expected public APIs, testing responsibilities, validation requirements, and architecture drift requirements for implementing the Governance bounded context as the second fully-implemented bounded context in ForgeOS.

This document introduces **no new architecture**, **no new technology decisions**, **no RFC**, **no TDS**, **no TDR**, **no ARCH**, and **no ISP**.

All scope is derived exclusively from the approved authority documents listed in the Authority Coverage Matrix.

---

# Objective

Implement the Governance bounded context as the second fully-implemented bounded context, establishing the organizational authority foundation required by all other bounded contexts.

The milestone shall:

1. **Governance Domain** — implement the Governance aggregate root with all entities, value objects, repository interface, domain events, and domain services as specified in TDS-0002
2. **Governance Application** — implement application service(s) for governance operations following the transaction coordination pattern established in MILESTONE-002.1
3. **Governance Infrastructure** — implement repository and event publisher infrastructure following the Organization domain pattern
4. **Platform Integration** — wire Governance dependencies into the Platform composition root
5. **Testing** — comprehensive unit tests, integration tests, and application service tests following ISP-0009 and ISP-0010

The milestone shall demonstrate the canonical ForgeOS vertical slice pattern (ISP-0010) applied to a new bounded context, following the Organization domain implementation pattern.

---

# Scope

This milestone covers:

1. **Governance domain crate** — complete domain layer implementation per TDS-0002
2. **Governance application crate** — application service(s) with transaction coordination per TDS-0004, ISP-0001, ISP-0006
3. **Governance infrastructure crate** — repository implementation, event publisher implementation per TDS-0004, ISP-0004, ISP-0005
4. **Platform updates** — wire Governance dependencies into composition root per ISP-0007
5. **Comprehensive tests** — unit tests, integration tests, application service tests per ISP-0009, ISP-0010

---

# Out of Scope

This milestone does **not** cover:

1. **Presentation layer** — no Governance UI, commands, view models, or IPC handlers (deferred to future milestone)
2. **Cross-context event consumption** — no consuming bounded contexts beyond Governance (deferred to future milestone)
3. **Event broker integration** — no message broker, event bus, or external messaging infrastructure (requires future RFC/TDS approval per NEXT_SESSION.md)
4. **Event persistence** — no event store or event sourcing (requires future RFC/TDS approval per NEXT_SESSION.md)
5. **Additional bounded contexts** — only Governance bounded context is implemented (Mission, Workforce, Knowledge, Memory, Process deferred to future milestones)
6. **Frontend framework selection** — remains deferred per TDR-0002
7. **Authentication/authorization** — not introduced in this milestone
8. **AI provider integration** — Phase 3 per ROADMAP.MD
9. **Cloud deployment** — not in scope for MVP
10. **Multi-tenant architecture** — not in scope for MVP

---

# Ownership

| Artifact | Architectural Owner | Authority |
|----------|---------------------|-----------|
| Governance aggregate | Governance Domain | TDS-0002; ARCH-0002 |
| Governance entities | Governance Domain | TDS-0002; ARCH-0002 |
| Governance value objects | Governance Domain | TDS-0002; ARCH-0002 |
| GovernanceRepository interface | Governance Domain | TDS-0002; ARCH-0002 |
| Governance domain events | Governance Domain | TDS-0002; ISP-0005; ARCH-0002 |
| Governance domain services | Governance Domain | TDS-0002; ARCH-0002 |
| Governance application services | Application Services | TDS-0004; ISP-0001; ARCH-0002 |
| Governance repository implementation | Infrastructure Domain | TDS-0004; ISP-0004; ARCH-0002 |
| Governance event publisher implementation | Infrastructure Domain | TDS-0004; ISP-0005; ARCH-0002 |
| Governance transaction implementation | Infrastructure Domain | TDS-0004; ISP-0006; ARCH-0002 |
| Dependency composition | Platform Domain | ARCH-0002; ISP-0007 |
| Governance tests | Application Services | ISP-0009; ISP-0010 |

Ownership is exclusive. No artifact shall have multiple architectural owners.

---

# Crate Boundaries

## New Crates

| Crate Name | Workspace Category | Architectural Owner | Location | Change Type |
|------------|--------------------|---------------------|----------|-------------|
| `forgeos-governance-domain` | Domains | Governance Domain | `implementation/rust/domains/governance-domain/` | New |
| `forgeos-approve-decision-application` | Applications | Application Services | `implementation/rust/applications/approve-decision/` | New |
| `forgeos-governance-infrastructure` | Infrastructure | Infrastructure Domain | `implementation/rust/infrastructure/governance/` | New |

## Modified Crates

| Crate Name | Workspace Category | Architectural Owner | Location | Change Type |
|------------|--------------------|---------------------|----------|-------------|
| `forgeos-desktop-platform` | Platform | Platform Domain | `implementation/rust/platform/desktop/` | Modified — wire Governance dependencies |

## Existing Crates (Consumed, Not Modified)

| Crate | Workspace Category | Architectural Owner |
|-------|--------------------|---------------------|
| `forgeos-organization-domain` | Domains | Organization Domain |
| `forgeos-create-organization-application` | Applications | Application Services |
| `forgeos-organization-infrastructure` | Infrastructure | Infrastructure Domain |
| `forgeos-organization-presentation` | Presentation | Presentation Domain |

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
| Governance Application → Governance Domain | Required | ARCH-0003; TDS-0004 |
| Governance Infrastructure → Governance Domain | Required | ARCH-0003; ISP-0004, ISP-0005, ISP-0006 |
| Platform → Governance Application | Required | ARCH-0003; ISP-0007 |
| Platform → Governance Infrastructure (composition only) | Required | ARCH-0003; ISP-0007 |
| Governance Domain → Organization Domain | Allowed (event consumption) | ARCH-0003; TDS-0002 |
| Governance Domain → Infrastructure | Forbidden | ARCH-0003; TDS-0002 |
| Governance Application → Infrastructure | Forbidden | ARCH-0003; TDS-0004 |
| Governance Domain → Platform | Forbidden | ARCH-0003; TDS-0002 |
| Governance Application → Platform | Forbidden | ARCH-0003; TDS-0004 |

---

# Integration Points

## 1. Governance Aggregate

| Integration Point | Authority | Description |
|-------------------|-----------|-------------|
| Governance aggregate root | TDS-0002; ARCH-0002 | Authoritative root of Governance bounded context |
| Decision entity | TDS-0002; RFC-0007 | Organizational decision with ownership and lifecycle |
| Policy entity | TDS-0002 | Organizational policy owned by Governance Unit |
| Standard entity | TDS-0002 | Engineering or organizational standard |
| DelegatedAuthority entity | TDS-0002 | Delegated execution authority |
| ApprovalRecord entity | TDS-0002 | Approval history and audit trail |
| GovernanceRule entity | TDS-0002 | Governance rule definition |

## 2. Governance Value Objects

| Integration Point | Authority | Description |
|-------------------|-----------|-------------|
| DecisionId | TDS-0002 | Unique decision identifier |
| PolicyId | TDS-0002 | Unique policy identifier |
| AuthorityLevel | TDS-0002; RFC-0007 | Decision authority level (1-5) |
| ApprovalStatus | TDS-0002 | Approval state (Proposed, Approved, Rejected) |
| GovernanceScope | TDS-0002 | Scope of governance authority |
| StandardIdentifier | TDS-0002 | Unique standard identifier |

## 3. Governance Repository

| Integration Point | Authority | Description |
|-------------------|-----------|-------------|
| GovernanceRepository interface | TDS-0002; ISP-0004 | Domain-owned repository contract |
| Repository implementation | TDS-0004; ISP-0004; TDR-0003 | SQLx/SQLite implementation in Infrastructure |

## 4. Governance Domain Events

| Integration Point | Authority | Description |
|-------------------|-----------|-------------|
| DecisionApproved | TDS-0002; ISP-0005 | Published when decision is approved |
| DecisionRejected | TDS-0002; ISP-0005 | Published when decision is rejected |
| PolicyPublished | TDS-0002; ISP-0005 | Published when policy is published |
| PolicyRetired | TDS-0002; ISP-0005 | Published when policy is retired |
| AuthorityDelegated | TDS-0002; ISP-0005 | Published when authority is delegated |
| AuthorityRevoked | TDS-0002; ISP-0005 | Published when authority is revoked |

## 5. Governance Domain Services

| Integration Point | Authority | Description |
|-------------------|-----------|-------------|
| PolicyEvaluationService | TDS-0002 | Evaluates policy compliance |
| GovernanceValidationService | TDS-0002 | Validates governance rules |
| AuthorityManagementService | TDS-0002 | Manages delegated authority |
| DecisionEvaluationService | TDS-0002 | Evaluates decision proposals |

## 6. Transaction Coordination

| Integration Point | Authority | Description |
|-------------------|-----------|-------------|
| Transaction trait | TDS-0004; ISP-0006; MILESTONE-002.1 | Application-owned transaction lifecycle |
| Transaction implementation | TDS-0004; ISP-0006; MILESTONE-002.1 | Infrastructure implementation (reuse pattern) |
| Post-commit event publication | ISP-0005; ISP-0006; MILESTONE-002.0 | Events published after successful commit |

## 7. Trust Boundary TB-2 (IPC Boundary)

| Boundary | Trust Level | Authority |
|----------|-------------|-----------|
| TB-2 — IPC Boundary | Validated | ARCH-0001; TDR-0002; TDR-0004 |

No domain entities or sensitive governance data cross the IPC boundary in this milestone. Governance operations remain internal to the backend.

---

# Expected Files

## New Files

### Governance Domain Crate (~20–25 files)

| File | Purpose | Authority |
|------|---------|-----------|
| `implementation/rust/domains/governance-domain/src/lib.rs` | Crate root | ARCH-0004 |
| `implementation/rust/domains/governance-domain/src/governance.rs` | Governance aggregate root | TDS-0002; ARCH-0002 |
| `implementation/rust/domains/governance-domain/src/value_objects/decision_id.rs` | Decision identifier | TDS-0002 |
| `implementation/rust/domains/governance-domain/src/value_objects/policy_id.rs` | Policy identifier | TDS-0002 |
| `implementation/rust/domains/governance-domain/src/value_objects/authority_level.rs` | Authority level | TDS-0002; RFC-0007 |
| `implementation/rust/domains/governance-domain/src/value_objects/approval_status.rs` | Approval status | TDS-0002 |
| `implementation/rust/domains/governance-domain/src/value_objects/governance_scope.rs` | Governance scope | TDS-0002 |
| `implementation/rust/domains/governance-domain/src/value_objects/standard_identifier.rs` | Standard identifier | TDS-0002 |
| `implementation/rust/domains/governance-domain/src/decision/` | Decision entity | TDS-0002; RFC-0007 |
| `implementation/rust/domains/governance-domain/src/policy/` | Policy entity | TDS-0002 |
| `implementation/rust/domains/governance-domain/src/standard/` | Standard entity | TDS-0002 |
| `implementation/rust/domains/governance-domain/src/delegated_authority/` | DelegatedAuthority entity | TDS-0002 |
| `implementation/rust/domains/governance-domain/src/approval_record/` | ApprovalRecord entity | TDS-0002 |
| `implementation/rust/domains/governance-domain/src/governance_rule/` | GovernanceRule entity | TDS-0002 |
| `implementation/rust/domains/governance-domain/src/governance_repository/` | GovernanceRepository interface | TDS-0002; ISP-0004 |
| `implementation/rust/domains/governance-domain/src/governance_domain_event/` | Domain events | TDS-0002; ISP-0005 |
| `implementation/rust/domains/governance-domain/src/domain_services/` | Domain services | TDS-0002 |
| `implementation/rust/domains/governance-domain/src/errors.rs` | Error types | ISP-0008 |

### Governance Application Crate (~10–15 files)

| File | Purpose | Authority |
|------|---------|-----------|
| `implementation/rust/applications/approve-decision/src/lib.rs` | Crate root | ARCH-0004 |
| `implementation/rust/applications/approve-decision/src/service.rs` | Application service(s) | TDS-0004; ISP-0001 |
| `implementation/rust/applications/approve-decision/src/transaction.rs` | Transaction trait usage | TDS-0004; ISP-0006; MILESTONE-002.1 |
| `implementation/rust/applications/approve-decision/src/commands/` | Command handlers | TDS-0004; ISP-0002 |
| `implementation/rust/applications/approve-decision/src/queries/` | Query handlers | TDS-0004; ISP-0003 |
| `implementation/rust/applications/approve-decision/src/dto/` | Data transfer objects | TDS-0004 |
| `implementation/rust/applications/approve-decision/src/errors.rs` | Error types | ISP-0008 |

### Governance Infrastructure Crate (~5–10 files)

| File | Purpose | Authority |
|------|---------|-----------|
| `implementation/rust/infrastructure/governance/src/lib.rs` | Crate root | ARCH-0004 |
| `implementation/rust/infrastructure/governance/src/repository.rs` | GovernanceRepository implementation | TDS-0004; ISP-0004; TDR-0003 |
| `implementation/rust/infrastructure/governance/src/event_publisher.rs` | Event publisher implementation | TDS-0004; ISP-0005 |
| `implementation/rust/infrastructure/governance/src/transaction.rs` | Transaction implementation | TDS-0004; ISP-0006; MILESTONE-002.1 |
| `implementation/rust/infrastructure/governance/src/errors.rs` | Error types | ISP-0008 |

### Modified Files

| File | Purpose | Authority |
|------|---------|-----------|
| `implementation/rust/Cargo.toml` | Add new workspace members | ARCH-0004 |
| `implementation/rust/platform/desktop/src/composition.rs` | Wire Governance dependencies | ISP-0007; MILESTONE-001.8 |

---

# Expected Modules

## `forgeos-governance-domain` Crate

### `governance.rs` (NEW)
- Governance aggregate root
- Aggregate identity
- Aggregate lifecycle (Created, Initialized, Active, Modified, Archived)
- Aggregate invariants

### Value Objects (~6 files)
- DecisionId — unique decision identifier
- PolicyId — unique policy identifier
- AuthorityLevel — decision authority level (1-5 per RFC-0007)
- ApprovalStatus — approval state machine
- GovernanceScope — scope of governance authority
- StandardIdentifier — unique standard identifier

### Entities (~6 modules)
- `decision/` — Decision entity with lifecycle, ownership, approval workflow
- `policy/` — Policy entity with versioning, publication, retirement
- `standard/` — Standard entity with versioning, enforcement
- `delegated_authority/` — DelegatedAuthority entity with delegation lifecycle
- `approval_record/` — ApprovalRecord entity with audit trail
- `governance_rule/` — GovernanceRule entity with rule definition

### `governance_repository/` (NEW)
- GovernanceRepository trait — domain-owned repository contract
- CRUD operations for Governance aggregate
- Existence checks
- Optimistic concurrency (if specified)

### `governance_domain_event/` (NEW)
- GovernanceDomainEvent enum
- DecisionApproved event
- DecisionRejected event
- PolicyPublished event
- PolicyRetired event
- AuthorityDelegated event
- AuthorityRevoked event

### `domain_services/` (NEW)
- PolicyEvaluationService — evaluates policy compliance
- GovernanceValidationService — validates governance rules
- AuthorityManagementService — manages delegated authority
- DecisionEvaluationService — evaluates decision proposals

### `errors.rs` (NEW)
- GovernanceError enum
- Error conversion implementations

## `forgeos-approve-decision-application` Crate

### `service.rs` (NEW)
- ApproveDecision application service (or similar)
- Transaction coordination
- Event publication after commit
- Workflow orchestration

### `transaction.rs` (NEW)
- Transaction trait usage (reuse pattern from MILESTONE-002.1)

### `commands/` (NEW)
- ApproveDecision command handler
- RejectDecision command handler
- PublishPolicy command handler
- DelegateAuthority command handler

### `queries/` (NEW)
- GetDecision query handler
- GetPolicy query handler
- ListDecisions query handler

### `dto/` (NEW)
- DecisionDto
- PolicyDto
- Command DTOs
- Query DTOs

### `errors.rs` (NEW)
- Application error types

## `forgeos-governance-infrastructure` Crate

### `repository.rs` (NEW)
- GovernanceRepository implementation
- SQLx/SQLite persistence
- Transaction participation

### `event_publisher.rs` (NEW)
- InMemoryEventPublisher implementation (reuse pattern from MILESTONE-002.0)

### `transaction.rs` (NEW)
- SqlxTransaction implementation (reuse pattern from MILESTONE-002.1)

### `lib.rs` (NEW)
- Crate root
- Module registration
- Public API exports

### `errors.rs` (NEW)
- Infrastructure error types

---

# Expected Public APIs

## Governance Aggregate (Domain)

| API | Signature | Authority |
|-----|-----------|-----------|
| `Governance::new(id, ...)` | Creates new Governance aggregate | TDS-0002 |
| `governance.approve_decision(decision_id, approver)` | Approves a decision | TDS-0002; RFC-0007 |
| `governance.reject_decision(decision_id, reason)` | Rejects a decision | TDS-0002; RFC-0007 |
| `governance.publish_policy(policy)` | Publishes a new policy | TDS-0002 |
| `governance.retire_policy(policy_id)` | Retires a policy | TDS-0002 |
| `governance.delegate_authority(...)` | Delegates authority | TDS-0002; RFC-0007 |
| `governance.revoke_authority(...)` | Revokes delegated authority | TDS-0002; RFC-0007 |
| `governance.take_events()` | Collects domain events | ISP-0005 |

## GovernanceRepository (Domain Interface)

| API | Signature | Authority |
|-----|-----------|-----------|
| `save(governance: &Governance)` | Persists governance aggregate | TDS-0002; ISP-0004 |
| `find_by_id(id: DecisionId)` | Retrieves governance by ID | TDS-0002; ISP-0004 |
| `exists(id: DecisionId)` | Checks existence | TDS-0002; ISP-0004 |
| `delete(id: DecisionId)` | Archives governance | TDS-0002; ISP-0004 |

## Application Services (Application)

| API | Signature | Authority |
|-----|-----------|-----------|
| `ApproveDecision::execute(cmd, tx, repo, event_publisher)` | Approves decision with transaction coordination | TDS-0004; ISP-0001; ISP-0006 |
| `RejectDecision::execute(cmd, tx, repo, event_publisher)` | Rejects decision with transaction coordination | TDS-0004; ISP-0001; ISP-0006 |
| `PublishPolicy::execute(cmd, tx, repo, event_publisher)` | Publishes policy with transaction coordination | TDS-0004; ISP-0001; ISP-0006 |

## Infrastructure Implementations

| API | Signature | Authority |
|-----|-----------|-----------|
| `SqlxGovernanceRepository` | SQLx/SQLite implementation | TDS-0004; ISP-0004; TDR-0003 |
| `InMemoryEventPublisher` | Event dispatch implementation | TDS-0004; ISP-0005 |
| `SqlxTransaction` | Transaction implementation | TDS-0004; ISP-0006; TDR-0003 |

---

# Testing Responsibilities

## Test Ownership

| Test Type | Owner | Authority |
|-----------|-------|-----------|
| Domain logic tests | Governance Domain | ISP-0009; ISP-0010 |
| Repository integration tests | Infrastructure Domain | ISP-0009; ISP-0010 |
| Application service tests | Application Services | ISP-0009; ISP-0010 |
| Transaction coordination tests | Application Services | ISP-0009; ISP-0010 |
| Event publication tests | Application Services | ISP-0009; ISP-0010 |

## Test Scope

### Domain Logic Tests

- Governance aggregate creation
- Decision approval workflow
- Decision rejection workflow
- Policy publication
- Policy retirement
- Authority delegation
- Authority revocation
- Aggregate invariants enforcement
- Domain event generation
- Error handling

### Repository Integration Tests

- Save and retrieve Governance aggregate
- Existence checks
- Update and delete operations
- Transaction participation
- Error handling and propagation

### Application Service Tests

- ApproveDecision command execution
- RejectDecision command execution
- PublishPolicy command execution
- Transaction coordination (begin, commit, rollback)
- Event publication after successful commit
- No event publication on rollback
- Dependency injection
- Error handling and propagation

### Integration Tests

- End-to-end flow: command → application service → transaction → domain → repository → commit → event publication
- Verify Governance aggregate lifecycle
- Verify transaction boundaries
- Verify event publication
- Verify rollback behavior

## Test Principles

- Tests shall be deterministic per ISP-0009
- Tests shall verify behavior at the correct architectural boundary per ISP-0009
- Tests shall preserve dependency boundaries per ISP-0009
- Tests shall verify both success and failure paths per ISP-0009
- Transaction abstraction shall be mockable for application service tests

---

# Validation and Completion Gates

## Gate 1: Architecture Compliance

**Criteria:**
- All domain entities, value objects, and aggregates match TDS-0002 specifications
- All repository interfaces match TDS-0002 contracts
- All domain events match TDS-0002 and ARCH-0002 specifications
- All domain services match TDS-0002 specifications
- All ownership rules comply with TDS-0003 and ARCH-0002
- All dependencies comply with ARCH-0003

**Verification:**
- Architecture review against GOVERNANCE-VALIDATION-REPORT.md
- Cross-reference with TDS-0002, TDS-0003, ARCH-0002
- Dependency graph validation

## Gate 2: Implementation Standards Compliance

**Criteria:**
- Code follows CODING_STANDARD.md
- Documentation follows DOCUMENTATION_STANDARD.md
- Tests follow TESTING_STANDARD.md
- Naming follows NAMING_STANDARD.md
- Git commits follow GIT_STANDARD.md
- All ISP patterns implemented correctly (ISP-0001 through ISP-0010)

**Verification:**
- `cargo check --workspace` passes
- `cargo test --workspace -- --test-threads=1` passes
- Clippy passes (if applicable)
- Code review

## Gate 3: Transaction Coordination

**Criteria:**
- All application services use Transaction trait (reuse from MILESTONE-002.1)
- Transaction lifecycle (begin, commit, rollback) implemented correctly
- Event publication occurs after successful commit
- Rollback on errors prevents event publication

**Verification:**
- Transaction coordination tests pass
- Event dispatch tests pass
- Rollback scenarios tested

## Gate 4: Test Coverage

**Criteria:**
- Unit tests for all domain logic
- Integration tests for repository
- Application service tests
- Transaction coordination tests
- Event publication tests

**Verification:**
- `cargo test --workspace -- --test-threads=1` passes
- Test coverage meets TESTING_STANDARD.md requirements
- All tests pass consistently

## Gate 5: Documentation

**Criteria:**
- Implementation report documents all decisions
- Milestone report documents scope, authority, and completion
- Architecture compliance documented
- Known issues documented

**Verification:**
- Implementation report complete
- Milestone report complete
- Documentation review

## Completion Criteria

The milestone is complete when:
1. All architecture compliance gates pass
2. All implementation standards gates pass
3. All transaction coordination gates pass
4. All test coverage gates pass
5. All documentation gates pass
6. `cargo check --workspace` passes
7. `cargo test --workspace -- --test-threads=1` passes with 0 failures
8. Architecture Office approves completion

---

# Explicit Out-of-Scope Boundaries

## Bounded Contexts

**Out of Scope:**
- Mission domain (future milestone)
- Workforce domain (future milestone)
- Knowledge domain (future milestone)
- Memory domain (future milestone)
- Process domain (future milestone)

**In Scope:**
- Governance bounded context only

## Presentation Layer

**Out of Scope:**
- Governance UI
- Governance commands in presentation layer
- Governance view models
- Governance IPC handlers

**In Scope:**
- None (presentation layer not modified)

## Cross-Cutting Concerns

**Out of Scope:**
- Event broker integration (requires future RFC/TDS approval per NEXT_SESSION.md)
- Event persistence (requires future RFC/TDS approval per NEXT_SESSION.md)
- Cross-context event consumption (deferred to future milestone)
- Additional domain events beyond Governance context (deferred to future milestone)

**In Scope:**
- Event publication within Governance context only
- In-process event dispatch (InMemoryEventPublisher)

## Architecture

**Out of Scope:**
- New RFCs (deferred until implementation experience requires it per PROJECT_STATUS.md)
- New TDSs (deferred until implementation experience requires it)
- New TDRs (deferred until implementation experience requires it)
- Architecture modifications (repository authority is complete per ARCHITECTURE-CONSOLIDATION-REPORT.md)
- Design Packages (not required; authority is complete)

**In Scope:**
- Implementation of existing approved authority only

## Technology

**Out of Scope:**
- Frontend framework selection (not required for domain implementation)
- AI provider integration (Phase 3 per ROADMAP.MD)
- Cloud deployment (not in scope for MVP)
- Multi-tenant architecture (not in scope for MVP)

**In Scope:**
- Rust/Cargo (TDR-0001)
- SQLx/SQLite (TDR-0003)
- Tauri 2.x (TDR-0002) — platform only
- Serde/JSON (TDR-0004) — IPC only

---

# Traceability Matrix

## Authority → Responsibility → Implementation → Validation

| Authority | Responsibility | Expected Implementation Area/File | Validation |
|-----------|----------------|-----------------------------------|------------|
| **RFC-0007** — Decision Authority Matrix | Decision ownership and approval authority | `governance-domain/src/decision/` | GOVERNANCE-VALIDATION-REPORT.md |
| **RFC-0007** — Decision Authority Matrix | Authority levels (1-5) | `governance-domain/src/value_objects/authority_level.rs` | GOVERNANCE-VALIDATION-REPORT.md |
| **RFC-0007** — Decision Authority Matrix | Delegation model | `governance-domain/src/delegated_authority/` | GOVERNANCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | Governance aggregate root | `governance-domain/src/governance.rs` | GOVERNANCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | Decision entity | `governance-domain/src/decision/` | GOVERNANCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | Policy entity | `governance-domain/src/policy/` | GOVERNANCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | Standard entity | `governance-domain/src/standard/` | GOVERNANCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | DelegatedAuthority entity | `governance-domain/src/delegated_authority/` | GOVERNANCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | ApprovalRecord entity | `governance-domain/src/approval_record/` | GOVERNANCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | GovernanceRule entity | `governance-domain/src/governance_rule/` | GOVERNANCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | Value objects (6 types) | `governance-domain/src/value_objects/` | GOVERNANCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | GovernanceRepository interface | `governance-domain/src/governance_repository/` | GOVERNANCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | Domain events (6 types) | `governance-domain/src/governance_domain_event/` | GOVERNANCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | Domain services (4 types) | `governance-domain/src/domain_services/` | GOVERNANCE-VALIDATION-REPORT.md |
| **TDS-0003** — Organization Model | Governance Unit responsibilities | All governance domain files | GOVERNANCE-VALIDATION-REPORT.md |
| **TDS-0003** — Organization Model | Governance authority and ownership | `governance-domain/src/governance.rs` | GOVERNANCE-VALIDATION-REPORT.md |
| **TDS-0004** — Application Model | Application service orchestration | `approve-decision/src/service.rs` | MILESTONE-002.1 |
| **TDS-0004** — Application Model | Transaction coordination | `approve-decision/src/transaction.rs` | MILESTONE-002.1 |
| **ARCH-0002** — Component Model | Governance Domain ownership | `governance-domain/` crate | GOVERNANCE-VALIDATION-REPORT.md |
| **ARCH-0002** — Component Model | Application Services ownership | `approve-decision/` crate | MILESTONE-002.1 |
| **ARCH-0002** — Component Model | Infrastructure Domain ownership | `governance/` infrastructure crate | MILESTONE-002.1 |
| **ARCH-0003** — Architecture Enforcement | Dependency contracts | All crate Cargo.toml files | ARCH-0003 |
| **ISP-0001** — Application Service Pattern | Application service structure | `approve-decision/src/service.rs` | MILESTONE-002.1 |
| **ISP-0002** — Command Handler Pattern | Command handlers | `approve-decision/src/commands/` | MILESTONE-002.1 |
| **ISP-0003** — Query Handler Pattern | Query handlers | `approve-decision/src/queries/` | MILESTONE-002.1 |
| **ISP-0004** — Repository Pattern | Repository interface and implementation | `governance-domain/src/governance_repository/`, `governance/src/repository.rs` | MILESTONE-002.1 |
| **ISP-0005** — Domain Event Pattern | Domain events and event publisher | `governance-domain/src/governance_domain_event/`, `governance/src/event_publisher.rs` | MILESTONE-002.0 |
| **ISP-0006** — Transaction Pattern | Transaction trait and implementation | `approve-decision/src/transaction.rs`, `governance/src/transaction.rs` | MILESTONE-002.1 |
| **ISP-0007** — Dependency Injection Pattern | Dependency composition | `platform/desktop/src/composition.rs` | MILESTONE-001.8 |
| **ISP-0008** — Error Handling Pattern | Error types and propagation | All `errors.rs` files | MILESTONE-002.1 |
| **ISP-0009** — Testing Pattern | Test structure and coverage | All test files | MILESTONE-002.1 |
| **ISP-0010** — Vertical Slice Pattern | Complete vertical slice | All Governance crate files | MILESTONE-002.1 |
| **TDR-0001** — Programming Language | Rust/Cargo toolchain | All `Cargo.toml` files | MILESTONE-001.1 |
| **TDR-0002** — Desktop Framework | Tauri 2.x runtime | `platform/desktop/` | MILESTONE-001.8 |
| **TDR-0003** — Storage Strategy | SQLx/SQLite persistence | `governance/src/repository.rs` | MILESTONE-001.7 |
| **TDR-0004** — IPC Serialization | Serde/JSON serialization | IPC boundary (not modified) | MILESTONE-001.8 |
| **TDR-0005** — Workspace Location | Workspace structure | All crate locations | MILESTONE-001.4 |
| **TDR-0006** — Organization ID Generation | UUID v4 (not used in Governance) | N/A | MILESTONE-001.5 |
| **ARCH-0004** — Workspace Specification | Crate organization | All new crate directories | MILESTONE-001.4 |
| **MILESTONE-002.0** — Event Dispatch | Event publication pattern | `governance/src/event_publisher.rs` | MILESTONE-002.0 |
| **MILESTONE-002.1** — Transaction Coordination | Transaction abstraction pattern | `approve-decision/src/transaction.rs`, `governance/src/transaction.rs` | MILESTONE-002.1 |

---

# Implementation Stop Boundaries

## STOP if Missing Authority

The following responsibilities require additional approved authority before implementation:

1. **Presentation layer for Governance** — No RFC, TDS, or ARCH document specifies Governance UI, commands, view models, or IPC handlers for this milestone. **STOP.** Do not implement presentation layer for Governance without approved authority.

2. **Event broker technology** — No RFC, TDS, TDR, or ARCH document approves a message broker, event bus, or external messaging system. **STOP.** Do not introduce RabbitMQ, Kafka, Redis Streams, or any external event infrastructure.

3. **Event persistence** — No approved TDS or RFC defines an event store or event sourcing mechanism. **STOP.** Do not implement event log persistence without approved authority.

4. **Cross-context event consumption** — No bounded contexts beyond Governance are implemented. **STOP.** Do not implement event consumers for Mission, Process, Knowledge, or other contexts without approved authority.

5. **Distributed transactions** — No RFC, TDS, or ARCH document approves two-phase commit, saga pattern, or cross-service transaction coordination. **STOP.** Do not implement distributed transactions without approved authority.

6. **New bounded contexts** — Only Governance bounded context is in scope. **STOP.** Do not implement additional bounded contexts without approved RFC and TDS.

7. **New RFCs/TDSs/TDRs** — No new architecture documents are required. **STOP.** Do not create new RFCs, TDSs, TDRs, or ARCH documents for this milestone.

## STOP if Architecture Violation

The following conditions require immediate cessation and architectural review:

1. Domain entities crossing IPC boundary — **STOP** per ARCH-0001 TB-2, TDR-0004
2. Business logic in Infrastructure — **STOP** per ARCH-0003 AV-001
3. Domain layer depending on Infrastructure — **STOP** per ARCH-0003 Dependency Contract
4. Application Service bypassing aggregate boundaries — **STOP** per TDS-0004, ARCH-0003
5. Event publication before transaction commit — **STOP** per ISP-0005, ISP-0006
6. Multiple architectural owners for one artifact — **STOP** per ARCH-0002, ARCH-0003 AV-007
7. Governance aggregate modifying foreign aggregates — **STOP** per TDS-0002, ARCH-0003
8. Transaction ownership moving to Infrastructure — **STOP** per TDS-0004, ISP-0006

---

# Verification Against MILESTONE-002.0 and MILESTONE-002.1

## No Contradictions Identified

MILESTONE-003.0 does not contradict MILESTONE-002.0 or MILESTONE-002.1:

### Event Publication Pattern

- **MILESTONE-002.0** established event publication pattern for Organization context
- **MILESTONE-003.0** reuses the same pattern for Governance context
- **No contradiction:** Pattern is reusable across bounded contexts

### Transaction Coordination Pattern

- **MILESTONE-002.1** established transaction abstraction in Application Layer
- **MILESTONE-003.0** reuses the same transaction abstraction for Governance application services
- **No contradiction:** Transaction abstraction is designed for reuse across bounded contexts

### Dependency Direction

- **MILESTONE-002.0/002.1** established dependency contracts for Organization vertical slice
- **MILESTONE-003.0** follows the same dependency contracts for Governance bounded context
- **No contradiction:** Dependency contracts are consistent

### Architecture Enforcement

- **MILESTONE-002.0/002.1** enforced ARCH-0003 rules for Organization context
- **MILESTONE-003.0** enforces the same ARCH-0003 rules for Governance context
- **No contradiction:** Architecture enforcement is consistent

### Vertical Slice Pattern

- **MILESTONE-002.0/002.1** demonstrated vertical slice pattern for Organization
- **MILESTONE-003.0** applies the same vertical slice pattern to Governance
- **No contradiction:** Vertical slice pattern is a general implementation pattern (ISP-0010)

## Consistency Verification

| Aspect | MILESTONE-002.0/002.1 | MILESTONE-003.0 | Consistent? |
|--------|------------------------|-----------------|-------------|
| Event publication after commit | ✅ | ✅ (reuses pattern) | Yes |
| Transaction abstraction in Application Layer | ✅ | ✅ (reuses pattern) | Yes |
| In-memory event publisher | ✅ | ✅ (reuses pattern) | Yes |
| SQLx transaction implementation | ✅ | ✅ (reuses pattern) | Yes |
| Dependency injection | ✅ | ✅ (reuses pattern) | Yes |
| Domain layer independence | ✅ | ✅ | Yes |
| No business logic in Infrastructure | ✅ | ✅ | Yes |
| Architecture enforcement | ✅ | ✅ | Yes |

**Conclusion:** MILESTONE-003.0 is fully consistent with MILESTONE-002.0 and MILESTONE-002.1. No contradictions exist.

---

# Authority Coverage Summary

Every implementation responsibility in this milestone traces to at least one approved authority document:

- **Governance aggregate structure** → TDS-0002, ARCH-0002
- **Governance entities** → TDS-0002, RFC-0007
- **Governance value objects** → TDS-0002
- **Governance repository** → TDS-0002, ISP-0004
- **Governance domain events** → TDS-0002, ISP-0005
- **Governance domain services** → TDS-0002
- **Application service orchestration** → TDS-0004, ISP-0001
- **Transaction coordination** → TDS-0004, ISP-0006, MILESTONE-002.1
- **Event publication** → ISP-0005, MILESTONE-002.0
- **Repository implementation** → TDS-0004, ISP-0004, TDR-0003
- **Dependency injection** → ISP-0007
- **Error handling** → ISP-0008
- **Testing** → ISP-0009, ISP-0010
- **Workspace organization** → ARCH-0004
- **Architecture enforcement** → ARCH-0003

No responsibility lacks authority coverage.

---

# Document Completion

This document is complete.

It establishes the **Implementation Contract** for MILESTONE-003.0 — Governance Domain Foundation, including scope, ownership, crate boundaries, dependency direction, integration points, expected files, expected modules, expected public APIs, testing responsibilities, validation requirements, stop boundaries, and full traceability to approved authority documents.

This document introduces no new architecture, no new technology decisions, no RFC, no TDS, no TDR, no ARCH, and no ISP.

Every responsibility traces to one or more approved authority documents in the ForgeOS authority chain.

---

# Authority Documents Inspected

## RFC Series
- RFC-0001 — ForgeOS Genome
- RFC-0007 — Decision Authority Matrix
- RFC-0006 — Executive Meeting Protocol

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

## Implementation Specifications
- ISP-0001 — Application Service Pattern
- ISP-0002 — Command Handler Pattern
- ISP-0003 — Query Handler Pattern
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
- MILESTONE-002.1 — Transaction Coordination Refinement
- MILESTONE-001-IMPLEMENTATION-BASELINE

## Validation Reports
- GOVERNANCE-VALIDATION-REPORT.md
- ARCHITECTURE-CONSOLIDATION-REPORT.md
- REPOSITORY-DRIVEN-IMPLEMENTATION-ROADMAP.md

---

# Milestone Objective

Implement the Governance bounded context as the second fully-implemented bounded context in ForgeOS, establishing the organizational authority foundation required by all other bounded contexts.

---

# Scope Summary

This milestone implements:

1. **Governance domain crate** — complete domain layer with aggregate, entities, value objects, repository interface, domain events, domain services (~20–25 files)
2. **Governance application crate** — application services with transaction coordination (~10–15 files)
3. **Governance infrastructure crate** — repository implementation, event publisher, transaction implementation (~5–10 files)
4. **Platform updates** — wire Governance dependencies (~2–3 files)
5. **Comprehensive tests** — unit tests, integration tests, application service tests (~20–30 test files)

**Total:** ~60–80 new source files, ~2–3 modified files

---

# Completion Criteria

The milestone is complete when:

1. ✅ All domain entities, value objects, and aggregates implement TDS-0002 specifications
2. ✅ All repository interfaces and implementations comply with ISP-0004
3. ✅ All domain events comply with ISP-0005
4. ✅ All application services comply with ISP-0001
5. ✅ Transaction coordination works correctly (ISP-0006, MILESTONE-002.1)
6. ✅ Event publication works correctly (ISP-0005, MILESTONE-002.0)
7. ✅ All tests pass (`cargo test --workspace -- --test-threads=1`)
8. ✅ Code compiles (`cargo check --workspace`)
9. ✅ Architecture compliance verified against GOVERNANCE-VALIDATION-REPORT.md
10. ✅ Architecture Office approves completion

---

*End of Document*