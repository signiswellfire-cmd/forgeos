# MILESTONE-004.0 — Workforce Domain Foundation

**Milestone ID:** MILESTONE-004.0

**Title:** Workforce Domain Foundation

**Status:** Proposed

**Version:** 1.0.0

---

## 1. Milestone Identity

**Milestone ID:** MILESTONE-004.0

**Title:** Workforce Domain Foundation

**Bounded Context:** Workforce

**Phase:** Foundation

**Sequence:** Second foundation milestone (after Governance)

**Dependencies:** MILESTONE-001 (Organization), MILESTONE-002.0 (Event Dispatch), MILESTONE-002.1 (Transaction Coordination), MILESTONE-003.0 (Governance)

---

## 2. Objective

Implement the Workforce bounded context as the third fully-implemented bounded context in ForgeOS, establishing the organizational capability foundation required by the Mission execution context.

The milestone shall:

1. **Workforce Domain** — implement the Workforce aggregate root with all entities, value objects, repository interface, domain events, and domain services as specified in TDS-0002 and RFC-0015
2. **Workforce Application** — implement application service(s) for workforce operations following the transaction coordination pattern established in MILESTONE-002.1
3. **Workforce Infrastructure** — implement repository and event publisher infrastructure following the Organization and Governance domain patterns
4. **Testing** — comprehensive unit tests following ISP-0009 and ISP-0010

The milestone shall demonstrate the canonical ForgeOS vertical slice pattern (ISP-0010) applied to the Workforce bounded context, following the Organization and Governance domain implementation patterns.

---

## 3. Authority Chain

### Primary Authority

| Authority | Document | Purpose |
|-----------|----------|---------|
| RFC | RFC-0015 — Digital Workforce Framework | Defines Professionals, lifecycle, team formation, workforce principles |
| TDS | TDS-0002 — Domain Model | Defines Workforce aggregate, entities, value objects, events, services |
| TDS | TDS-0003 — Organization Model | Defines Workforce Unit responsibilities, authority, ownership |
| ARCH | ARCH-0002 — Component Model | Defines Workforce Domain as implementation owner |
| ARCH | ARCH-0003 — Architecture Enforcement | Enforces dependency contracts and ownership |

### Supporting Authority

| Authority | Document | Purpose |
|-----------|----------|---------|
| RFC | RFC-0001 — ForgeOS Genome | Establishes Professional as a first-class concept |
| RFC | RFC-0028 — Competency Management | Defines competency evaluation (referenced in validation) |
| ISP | ISP-0001 — Application Service Pattern | Application service structure |
| ISP | ISP-0004 — Repository Pattern | Repository interface and implementation |
| ISP | ISP-0005 — Domain Event Pattern | Domain events and event publisher |
| ISP | ISP-0006 — Transaction Pattern | Transaction coordination |
| ISP | ISP-0007 — Dependency Injection Pattern | Dependency composition |
| ISP | ISP-0008 — Error Handling Pattern | Error types and propagation |
| ISP | ISP-0009 — Testing Pattern | Test structure and coverage |
| ISP | ISP-0010 — Vertical Slice Pattern | Complete vertical slice |
| Validation | WORKFORCE-VALIDATION-REPORT.md | Architecture validation complete |
| Milestone | MILESTONE-002.0 — Event Dispatch | Event publication pattern |
| Milestone | MILESTONE-002.1 — Transaction Coordination | Transaction abstraction pattern |
| Milestone | MILESTONE-003.0 — Governance Domain Foundation | Implementation pattern reference |

### Authority Traceability

Every implementation responsibility in this milestone traces to at least one approved authority document. No new architecture, technology decisions, RFCs, TDSs, TDRs, ARCH documents, or ISPs are introduced.

---

## 4. Why Workforce is the Next Milestone

### Dependency Order

1. **Minimal Dependencies** — Workforce depends only on Organization (already implemented in MILESTONE-001)
2. **No Governance Dependency** — Workforce does not require Governance for foundation implementation
3. **Enables Mission** — Workforce provides capability assignment consumed by Mission context
4. **Independent Implementation** — Can be implemented without waiting for other contexts

### Implementation Sequence

Per REPOSITORY-DRIVEN-IMPLEMENTATION-ROADMAP.md:

| Order | Context | Dependencies | Status |
|-------|---------|--------------|--------|
| 1 | ✅ Governance | Organization | Complete (MILESTONE-003.0) |
| 2 | ⏸️ Workforce | Organization | **NEXT** |
| 3 | ⏸️ Mission | Organization, Workforce, Governance | Waiting |
| 4 | ⏸️ Knowledge | Organization | Waiting |
| 5 | ⏸️ Memory | Organization, Knowledge | Waiting |
| 6 | ⏸️ Process | Organization, Mission, Governance | Waiting |

### Rationale

Workforce is the correct next milestone because:
- Dependencies are satisfied (Organization implemented)
- Provides maximum value (enables Mission context)
- Follows the established implementation sequence
- Authority is complete (RFC-0015, TDS-0002, TDS-0003, WORKFORCE-VALIDATION-REPORT.md)
- Pattern is proven (Organization, Governance domains)

---

## 5. Workforce Responsibilities

### Authorized Responsibilities

Per **TDS-0003**, the Workforce Unit owns:

- workforce capability;
- competency management;
- professional development;
- capability assignment;
- organizational capacity.

Per **TDS-0002**, the Workforce context owns:

- workforce identity;
- professional records;
- capability assignments;
- competency evaluations;
- team relationships.

Per **ARCH-0002**, the Workforce Domain owns:

- Professionals;
- Teams;
- Skills;
- Competencies;
- Capability Assignments;
- Workforce Metadata;
- Team Memberships.

Per **RFC-0015**, the Digital Workforce Framework defines how digital Professionals are created, governed, assigned, evaluated, evolved, and coordinated within a ForgeOS Organization.

### Responsibility Characteristics

Every workforce responsibility satisfies:

- one owner;
- explicit authority;
- traceable delegation;
- measurable accountability;
- governed execution.

Responsibility ownership remains stable throughout the lifecycle of the responsibility.

---

## 6. Domain Scope

### Workforce Aggregate Root

The Workforce aggregate is the authoritative root of the Workforce bounded context (TDS-0002, ARCH-0002).

**Ownership:**
- Workforce identity is singular
- Team membership remains internally consistent
- Competency history is append-only
- Capability ownership remains explicit
- Workforce aggregates never modify foreign aggregates

**Aggregate Lifecycle:**
- Created
- Initialized
- Active
- Modified
- Archived

### Entities

| Entity | Authority | Description |
|--------|-----------|-------------|
| Professional | RFC-0015, TDS-0002 | Organizational role with responsibilities, competencies, authority level |
| Team | RFC-0015, TDS-0002 | Temporary collection of Professionals assembled around Missions |
| Competency | TDS-0002, RFC-0028 | Measure of professional capability |
| Skill | TDS-0002 | Specific professional capability |
| CapabilityAssignment | TDS-0002 | Assignment of capabilities to professionals or teams |
| TeamMembership | TDS-0002, ARCH-0002 | Team membership governance |

### Value Objects

| Value Object | Authority | Description |
|--------------|-----------|-------------|
| ProfessionalId | TDS-0002 | Unique professional identifier |
| TeamId | TDS-0002 | Unique team identifier |
| CompetencyLevel | TDS-0002 | Competency proficiency level |
| SkillIdentifier | TDS-0002 | Unique skill identifier |
| WorkforceStatus | TDS-0002 | Workforce status (active, inactive, etc.) |
| CapabilityReference | TDS-0002 | Reference to organizational capability |

### Repository Interface

**WorkforceRepository** (TDS-0002, ISP-0004)

Domain-owned repository contract for persisting and retrieving Workforce aggregates.

### Domain Events (Published)

Per **TDS-0002** and **ARCH-0002**:

| Event | Trigger | Authority |
|-------|---------|-----------|
| ProfessionalCreated | Professional registered | TDS-0002, ISP-0005 |
| ProfessionalUpdated | Professional modified | TDS-0002, ISP-0005 |
| TeamCreated | Team formed | TDS-0002, ISP-0005 |
| TeamMembershipChanged | Team membership changed | TDS-0002, ISP-0005 |
| SkillRegistered | Skill registered | TDS-0002, ISP-0005 |
| CompetencyEvaluated | Competency evaluated | TDS-0002, ISP-0005 |
| CapabilityAssigned | Capability assigned | TDS-0002, ISP-0005 |

### Domain Events (Consumed)

Per **TDS-0002** and **ARCH-0002**:

| Event | Purpose | Authority |
|-------|---------|-----------|
| MissionAssigned | Workforce planning | TDS-0002, ARCH-0002 |
| LearningCompleted | Competency update | TDS-0002, ARCH-0002 |
| OrganizationUpdated | Workforce alignment | TDS-0002, ARCH-0002 |

### Domain Services

Per **TDS-0002**:

| Service | Responsibility | Authority |
|---------|----------------|-----------|
| CompetencyEvaluationService | Evaluates professional competency | TDS-0002 |
| WorkforcePlanningService | Plans workforce capacity | TDS-0002 |
| CapabilityAssignmentService | Assigns capabilities to professionals/teams | TDS-0002 |
| TeamFormationService | Forms teams for missions | TDS-0002 |

### Errors Module

Per **ISP-0008**:

- WorkforceError enum
- Error conversion implementations
- Error propagation patterns

---

## 7. Application Scope

### Application Service(s)

Per **TDS-0004** and **ISP-0001**:

| Service | Responsibility | Authority |
|---------|----------------|-----------|
| ManageWorkforceService | Orchestrates workforce operations | TDS-0004, ISP-0001 |

**Responsibilities:**
- Coordinate workforce use cases
- Inject transaction and repository dependencies
- Orchestrate domain operations
- Publish events after successful commit

### Transaction Coordination

Per **TDS-0004**, **ISP-0006**, and **MILESTONE-002.1**:

- Use Transaction trait from Application Layer
- Transaction lifecycle: begin → execute → commit/rollback
- Event publication after successful commit
- Rollback prevents event publication

### Command/Query Handlers

**Foundation Milestone Scope:**
- Command handlers: Deferred to future milestone
- Query handlers: Deferred to future milestone
- DTOs: Deferred to future milestone

**Rationale:** Foundation milestone follows Organization and Governance pattern (minimal application layer).

---

## 8. Infrastructure Scope

### Repository Implementation

Per **TDS-0004**, **ISP-0004**, and **TDR-0003**:

- **Foundation Milestone:** In-memory stub implementation
- **Future Milestone:** SQLx/SQLite implementation

**Rationale:** Foundation milestone pattern established in MILESTONE-001.5 and MILESTONE-003.0.

### Event Publisher Implementation

Per **TDS-0004**, **ISP-0005**, and **MILESTONE-002.0**:

- **Foundation Milestone:** In-memory stub implementation
- **Future Milestone:** Event bus integration (requires future RFC/TDS)

**Rationale:** Foundation milestone pattern established in MILESTONE-002.0 and MILESTONE-003.0.

### Transaction Implementation

Per **TDS-0004**, **ISP-0006**, and **MILESTONE-002.1**:

- **Foundation Milestone:** Stub implementation
- **Future Milestone:** Full SQLx transaction implementation

**Rationale:** Foundation milestone pattern established in MILESTONE-002.1 and MILESTONE-003.0.

---

## 9. Platform/Composition Scope

### Foundation Milestone

**NOT IN SCOPE for MILESTONE-004.0**

Per the approved scope pattern from MILESTONE-003.0, platform composition wiring is deferred to a future milestone.

**Rationale:** Foundation milestone focuses on domain, application, and infrastructure layers. Platform wiring is not required for domain validation.

### Future Milestone

Platform composition root updates will be required in a future milestone to wire Workforce dependencies into the desktop platform (ISP-0007).

---

## 10. Expected Crates

### New Crates

| Crate Name | Category | Location | Authority |
|------------|----------|----------|-----------|
| `forgeos-workforce-domain` | Domains | `implementation/rust/domains/workforce-domain/` | ARCH-0002 |
| `forgeos-manage-workforce-application` | Applications | `implementation/rust/applications/manage-workforce/` | ARCH-0002 |
| `forgeos-workforce-infrastructure` | Infrastructure | `implementation/rust/infrastructure/workforce/` | ARCH-0002 |

### Modified Crates

| Crate Name | Change | Authority |
|------------|--------|-----------|
| `forgeos-desktop-platform` | Add Workforce dependencies (future milestone) | ISP-0007 |

---

## 11. Expected Implementation Areas/Files

### Domain Layer (~20–25 files)

**Crate Root:**
- `implementation/rust/domains/workforce-domain/Cargo.toml`
- `implementation/rust/domains/workforce-domain/src/lib.rs`

**Aggregate:**
- `implementation/rust/domains/workforce-domain/src/workforce.rs`

**Value Objects (~6 files):**
- `implementation/rust/domains/workforce-domain/src/value_objects.rs` (or individual files per pattern)

**Entities (~6 modules):**
- `implementation/rust/domains/workforce-domain/src/professional/`
- `implementation/rust/domains/workforce-domain/src/team/`
- `implementation/rust/domains/workforce-domain/src/competency/`
- `implementation/rust/domains/workforce-domain/src/skill/`
- `implementation/rust/domains/workforce-domain/src/capability_assignment/`
- `implementation/rust/domains/workforce-domain/src/team_membership/`

**Repository:**
- `implementation/rust/domains/workforce-domain/src/workforce_repository.rs`

**Domain Events (~7 files):**
- `implementation/rust/domains/workforce-domain/src/workforce_domain_event.rs`

**Domain Services (~4 files):**
- `implementation/rust/domains/workforce-domain/src/domain_services/competency_evaluation_service.rs`
- `implementation/rust/domains/workforce-domain/src/domain_services/workforce_planning_service.rs`
- `implementation/rust/domains/workforce-domain/src/domain_services/capability_assignment_service.rs`
- `implementation/rust/domains/workforce-domain/src/domain_services/team_formation_service.rs`

**Errors:**
- `implementation/rust/domains/workforce-domain/src/errors.rs`

### Application Layer (~4–6 files)

**Crate Root:**
- `implementation/rust/applications/manage-workforce/Cargo.toml`
- `implementation/rust/applications/manage-workforce/src/lib.rs`

**Service:**
- `implementation/rust/applications/manage-workforce/src/service.rs`

**Transaction:**
- `implementation/rust/applications/manage-workforce/src/transaction.rs`

**Future (deferred):**
- `commands/` — Command handlers
- `queries/` — Query handlers
- `dto/` — Data transfer objects
- `errors.rs` — Application error types

### Infrastructure Layer (~4 files)

**Crate Root:**
- `implementation/rust/infrastructure/workforce/Cargo.toml`
- `implementation/rust/infrastructure/workforce/src/lib.rs`

**Repository:**
- `implementation/rust/infrastructure/workforce/src/repository.rs` (stub)

**Event Publisher:**
- `implementation/rust/infrastructure/workforce/src/event_publisher.rs` (stub)

**Future (deferred):**
- `transaction.rs` — Transaction implementation
- `errors.rs` — Infrastructure error types

### Workspace Configuration

**Modified:**
- `implementation/rust/Cargo.toml` — Add 3 new workspace members

---

## 12. Dependencies

### Required Dependencies (Satisfied)

| Dependency | Status | Evidence |
|------------|--------|----------|
| Organization domain | ✅ Satisfied | MILESTONE-001 complete |
| Transaction coordination pattern | ✅ Satisfied | MILESTONE-002.1 complete |
| Event dispatch pattern | ✅ Satisfied | MILESTONE-002.0 complete |
| Governance domain | ✅ Satisfied | MILESTONE-003.0 complete (not required but available) |
| All ISP patterns | ✅ Satisfied | ISP-0001 through ISP-0010 approved |
| Workforce validation | ✅ Satisfied | WORKFORCE-VALIDATION-REPORT.md complete |
| Workforce authority | ✅ Satisfied | RFC-0015, TDS-0002, TDS-0003 approved |

### Not Required

| Dependency | Required? | Rationale |
|------------|-----------|-----------|
| Mission domain | ❌ NO | Workforce does not depend on Mission |
| Knowledge domain | ❌ NO | Workforce does not depend on Knowledge |
| Memory domain | ❌ NO | Workforce does not depend on Memory |
| Process domain | ❌ NO | Workforce does not depend on Process |
| Event broker | ❌ NO | Requires future RFC/TDS approval |
| Event persistence | ❌ NO | Requires future RFC/TDS approval |

**Determination:** All required dependencies are satisfied. No unsatisfied dependencies block MILESTONE-004.0.

---

## 13. Workforce Entities, Aggregates, Value Objects, Repositories, Events, and Services

### Aggregate Root

**Workforce** (TDS-0002, ARCH-0002)

- Authoritative root of Workforce bounded context
- Owns professional records, capability assignments, competency evaluations, team relationships
- Aggregate consistency boundary
- Never modifies foreign aggregates

### Entities

| Entity | Authority | Description |
|--------|-----------|-------------|
| Professional | RFC-0015, TDS-0002 | Permanent organizational responsibility with title, competencies, authority level, supported capabilities |
| Team | RFC-0015, TDS-0002 | Temporary collection of Professionals assembled around Missions |
| Competency | TDS-0002, RFC-0028 | Measure of professional capability |
| Skill | TDS-0002 | Specific professional capability |
| CapabilityAssignment | TDS-0002 | Assignment of capabilities to professionals or teams |
| TeamMembership | TDS-0002, ARCH-0002 | Team membership governance |

### Value Objects

| Value Object | Authority | Description |
|--------------|-----------|-------------|
| ProfessionalId | TDS-0002 | Unique professional identifier (UUID v4) |
| TeamId | TDS-0002 | Unique team identifier (UUID v4) |
| CompetencyLevel | TDS-0002 | Competency proficiency level |
| SkillIdentifier | TDS-0002 | Unique skill identifier (UUID v4) |
| WorkforceStatus | TDS-0002 | Workforce status (active, inactive, etc.) |
| CapabilityReference | TDS-0002 | Reference to organizational capability |

### Repository Interface

**WorkforceRepository** (TDS-0002, ISP-0004)

Domain-owned repository contract:

- `save(workforce: &Workforce)` — Persist workforce aggregate
- `find_by_id(id: ProfessionalId)` — Retrieve workforce by ID
- `exists(id: ProfessionalId)` — Check existence
- `delete(id: ProfessionalId)` — Archive workforce

### Domain Events (Published)

Per **TDS-0002** and **ARCH-0002**:

| Event | Trigger | Authority |
|-------|---------|-----------|
| ProfessionalCreated | Professional registered | TDS-0002, ISP-0005 |
| ProfessionalUpdated | Professional modified | TDS-0002, ISP-0005 |
| TeamCreated | Team formed | TDS-0002, ISP-0005 |
| TeamMembershipChanged | Team membership changed | TDS-0002, ISP-0005 |
| SkillRegistered | Skill registered | TDS-0002, ISP-0005 |
| CompetencyEvaluated | Competency evaluated | TDS-0002, ISP-0005 |
| CapabilityAssigned | Capability assigned | TDS-0002, ISP-0005 |

### Domain Events (Consumed)

Per **TDS-0002** and **ARCH-0002**:

| Event | Purpose | Authority |
|-------|---------|-----------|
| MissionAssigned | Workforce planning | TDS-0002, ARCH-0002 |
| LearningCompleted | Competency update | TDS-0002, ARCH-0002 |
| OrganizationUpdated | Workforce alignment | TDS-0002, ARCH-0002 |

### Domain Services

Per **TDS-0002**:

| Service | Responsibility | Authority |
|---------|----------------|-----------|
| CompetencyEvaluationService | Evaluates professional competency | TDS-0002 |
| WorkforcePlanningService | Plans workforce capacity | TDS-0002 |
| CapabilityAssignmentService | Assigns capabilities to professionals/teams | TDS-0002 |
| TeamFormationService | Forms teams for missions | TDS-0002 |

---

## 14. Testing Requirements

### Test Ownership

Per **ISP-0009** and **ISP-0010**:

| Test Type | Owner | Authority |
|-----------|-------|-----------|
| Domain logic tests | Workforce Domain | ISP-0009, ISP-0010 |
| Repository integration tests | Infrastructure Domain | ISP-0009, ISP-0010 |
| Application service tests | Application Services | ISP-0009, ISP-0010 |

### Test Scope (Foundation Milestone)

**Domain Logic Tests:**
- Workforce aggregate creation
- Professional registration
- Team creation
- Competency evaluation
- Capability assignment
- Aggregate invariants enforcement
- Domain event generation
- Error handling

**Repository Integration Tests:**
- Deferred to future milestone (foundation pattern)

**Application Service Tests:**
- Deferred to future milestone (foundation pattern)

### Test Principles

Per **ISP-0009**:

- Tests shall be deterministic
- Tests shall verify behavior at the correct architectural boundary
- Tests shall preserve dependency boundaries
- Tests shall verify both success and failure paths

---

## 15. Validation Gates

### Gate 1: Architecture Compliance

**Criteria:**
- All domain entities, value objects, and aggregates match TDS-0002 specifications
- All repository interfaces match TDS-0002 contracts
- All domain events match TDS-0002 and ARCH-0002 specifications
- All domain services match TDS-0002 specifications
- All ownership rules comply with TDS-0003 and ARCH-0002
- All dependencies comply with ARCH-0003

**Verification:**
- Architecture review against WORKFORCE-VALIDATION-REPORT.md
- Cross-reference with RFC-0015, TDS-0002, TDS-0003
- Dependency graph validation

### Gate 2: Implementation Standards Compliance

**Criteria:**
- Code follows CODING_STANDARD.md
- Documentation follows DOCUMENTATION_STANDARD.md
- Tests follow TESTING_STANDARD.md
- Naming follows NAMING_STANDARD.md
- All ISP patterns implemented correctly (ISP-0001, ISP-0004, ISP-0005, ISP-0006, ISP-0008, ISP-0009, ISP-0010)

**Verification:**
- `cargo check --workspace` passes
- `cargo test --workspace -- --test-threads=1` passes
- Code review

### Gate 3: Transaction Coordination

**Criteria:**
- Application services use Transaction trait (reuse from MILESTONE-002.1)
- Transaction lifecycle (begin, commit, rollback) implemented correctly
- Event publication occurs after successful commit
- Rollback on errors prevents event publication

**Verification:**
- Transaction coordination tests pass
- Event dispatch tests pass

### Gate 4: Test Coverage

**Criteria:**
- Unit tests for all domain logic
- Tests follow ISP-0009 and ISP-0010

**Verification:**
- `cargo test --workspace -- --test-threads=1` passes
- Test coverage meets TESTING_STANDARD.md requirements

### Gate 5: Documentation

**Criteria:**
- Implementation report documents all decisions
- Milestone report documents scope, authority, and completion
- Architecture compliance documented

**Verification:**
- Implementation report complete
- Documentation review

---

## 16. Completion Criteria

The milestone is complete when:

1. All domain entities, value objects, and aggregates implement TDS-0002 and RFC-0015 specifications
2. All repository interfaces comply with ISP-0004
3. All domain events comply with ISP-0005
4. All application services comply with ISP-0001
5. Transaction coordination works correctly (ISP-0006, MILESTONE-002.1)
6. Event publication works correctly (ISP-0005, MILESTONE-002.0)
7. All tests pass (`cargo test --workspace -- --test-threads=1`)
8. Code compiles (`cargo check --workspace`)
9. Architecture compliance verified against WORKFORCE-VALIDATION-REPORT.md
10. Architecture Office approves completion

---

## 17. Explicit Out-of-Scope Boundaries

### Bounded Contexts

**Out of Scope:**
- Mission domain (future milestone)
- Knowledge domain (future milestone)
- Memory domain (future milestone)
- Process domain (future milestone)
- Governance domain (already implemented)

**In Scope:**
- Workforce bounded context only

### Presentation Layer

**Out of Scope:**
- Workforce UI
- Workforce commands in presentation layer
- Workforce view models
- Workforce IPC handlers

**In Scope:**
- None (presentation layer not modified)

### Cross-Cutting Concerns

**Out of Scope:**
- Event broker integration (requires future RFC/TDS approval per NEXT_SESSION.md)
- Event persistence (requires future RFC/TDS approval per NEXT_SESSION.md)
- Cross-context event consumption (deferred to future milestone)
- Additional domain events beyond Workforce context (deferred to future milestone)

**In Scope:**
- Event publication within Workforce context only
- In-process event dispatch (InMemoryEventPublisher)

### Application Layer

**Out of Scope:**
- Command handlers (deferred to future milestone)
- Query handlers (deferred to future milestone)
- DTOs (deferred to future milestone)

**In Scope:**
- Application service orchestration only

### Infrastructure Layer

**Out of Scope:**
- SQLx/SQLite repository implementation (deferred to future milestone)
- Full transaction implementation (deferred to future milestone)
- Event bus integration (deferred to future milestone)

**In Scope:**
- In-memory stub implementations (foundation milestone pattern)

### Platform Layer

**Out of Scope:**
- Platform composition wiring (deferred to future milestone)

**In Scope:**
- None (platform layer not modified)

### Architecture

**Out of Scope:**
- New RFCs (deferred until implementation experience requires it per PROJECT_STATUS.md)
- New TDSs (deferred until implementation experience requires it)
- New TDRs (deferred until implementation experience requires it)
- Architecture modifications (repository authority is complete per ARCHITECTURE-CONSOLIDATION-REPORT.md)
- Design Packages (not required; authority is complete)

**In Scope:**
- Implementation of existing approved authority only

### Technology

**Out of Scope:**
- Frontend framework selection (not required for domain implementation)
- AI provider integration (Phase 3 per ROADMAP.MD)
- Cloud deployment (not in scope for MVP)
- Multi-tenant architecture (not in scope for MVP)

**In Scope:**
- Rust/Cargo (TDR-0001)
- SQLx/SQLite (TDR-0003) — future milestone
- Tauri 2.x (TDR-0002) — platform only
- Serde/JSON (TDR-0004) — IPC only

---

## 18. Authority Traceability Matrix

### Authority → Responsibility → Implementation → Validation

| Authority | Responsibility | Expected Implementation Area/File | Validation |
|-----------|----------------|-----------------------------------|------------|
| **RFC-0015** — Digital Workforce Framework | Professional definition and characteristics | `workforce-domain/src/professional/` | WORKFORCE-VALIDATION-REPORT.md |
| **RFC-0015** — Digital Workforce Framework | Professional lifecycle (7 stages) | `workforce-domain/src/professional/` | WORKFORCE-VALIDATION-REPORT.md |
| **RFC-0015** — Digital Workforce Framework | Team formation model | `workforce-domain/src/team/` | WORKFORCE-VALIDATION-REPORT.md |
| **RFC-0015** — Digital Workforce Framework | Workforce principles | `workforce-domain/src/` | WORKFORCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | Workforce aggregate root | `workforce-domain/src/workforce.rs` | WORKFORCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | Professional entity | `workforce-domain/src/professional/` | WORKFORCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | Team entity | `workforce-domain/src/team/` | WORKFORCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | Competency entity | `workforce-domain/src/competency/` | WORKFORCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | Skill entity | `workforce-domain/src/skill/` | WORKFORCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | CapabilityAssignment entity | `workforce-domain/src/capability_assignment/` | WORKFORCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | TeamMembership entity | `workforce-domain/src/team_membership/` | WORKFORCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | Value objects (6 types) | `workforce-domain/src/value_objects/` | WORKFORCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | WorkforceRepository interface | `workforce-domain/src/workforce_repository/` | WORKFORCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | Domain events (7 published, 3 consumed) | `workforce-domain/src/workforce_domain_event/` | WORKFORCE-VALIDATION-REPORT.md |
| **TDS-0002** — Domain Model | Domain services (4 types) | `workforce-domain/src/domain_services/` | WORKFORCE-VALIDATION-REPORT.md |
| **TDS-0003** — Organization Model | Workforce Unit responsibilities | All workforce domain files | WORKFORCE-VALIDATION-REPORT.md |
| **TDS-0003** — Organization Model | Workforce authority and ownership | `workforce-domain/src/workforce.rs` | WORKFORCE-VALIDATION-REPORT.md |
| **TDS-0004** — Application Model | Application service orchestration | `manage-workforce/src/service.rs` | MILESTONE-002.1 |
| **TDS-0004** — Application Model | Transaction coordination | `manage-workforce/src/transaction.rs` | MILESTONE-002.1 |
| **ARCH-0002** — Component Model | Workforce Domain ownership | `workforce-domain/` crate | WORKFORCE-VALIDATION-REPORT.md |
| **ARCH-0002** — Component Model | Application Services ownership | `manage-workforce/` crate | MILESTONE-002.1 |
| **ARCH-0002** — Component Model | Infrastructure Domain ownership | `workforce/` infrastructure crate | MILESTONE-002.1 |
| **ARCH-0003** — Architecture Enforcement | Dependency contracts | All crate Cargo.toml files | ARCH-0003 |
| **ISP-0001** — Application Service Pattern | Application service structure | `manage-workforce/src/service.rs` | MILESTONE-002.1 |
| **ISP-0004** — Repository Pattern | Repository interface and implementation | `workforce-domain/src/workforce_repository/`, `workforce/src/repository.rs` | MILESTONE-002.1 |
| **ISP-0005** — Domain Event Pattern | Domain events and event publisher | `workforce-domain/src/workforce_domain_event/`, `workforce/src/event_publisher.rs` | MILESTONE-002.0 |
| **ISP-0006** — Transaction Pattern | Transaction trait and implementation | `manage-workforce/src/transaction.rs`, `workforce/src/transaction.rs` | MILESTONE-002.1 |
| **ISP-0007** — Dependency Injection Pattern | Dependency composition | `platform/desktop/src/composition.rs` (future milestone) | MILESTONE-001.8 |
| **ISP-0008** — Error Handling Pattern | Error types and propagation | All `errors.rs` files | MILESTONE-002.1 |
| **ISP-0009** — Testing Pattern | Test structure and coverage | All test files | MILESTONE-002.1 |
| **ISP-0010** — Vertical Slice Pattern | Complete vertical slice | All Workforce crate files | MILESTONE-002.1 |
| **TDR-0001** — Programming Language | Rust/Cargo toolchain | All `Cargo.toml` files | MILESTONE-001.1 |
| **TDR-0003** — Storage Strategy | SQLx/SQLite persistence | `workforce/src/repository.rs` (future milestone) | MILESTONE-001.7 |
| **ARCH-0004** — Workspace Specification | Crate organization | All new crate directories | MILESTONE-001.4 |
| **MILESTONE-002.0** — Event Dispatch | Event publication pattern | `workforce/src/event_publisher.rs` | MILESTONE-002.0 |
| **MILESTONE-002.1** — Transaction Coordination | Transaction abstraction pattern | `manage-workforce/src/transaction.rs`, `workforce/src/transaction.rs` | MILESTONE-002.1 |

---

## 19. Implementation Stop Boundaries

## STOP if Missing Authority

The following responsibilities require additional approved authority before implementation:

1. **Presentation layer for Workforce** — No RFC, TDS, or ARCH document specifies Workforce UI, commands, view models, or IPC handlers for this milestone. **STOP.** Do not implement presentation layer for Workforce without approved authority.

2. **Event broker technology** — No RFC, TDS, TDR, or ARCH document approves a message broker, event bus, or external messaging system. **STOP.** Do not introduce RabbitMQ, Kafka, Redis Streams, or any external event infrastructure.

3. **Event persistence** — No approved TDS or RFC defines an event store or event sourcing mechanism. **STOP.** Do not implement event log persistence without approved authority.

4. **Cross-context event consumption** — No bounded contexts beyond Workforce are implemented. **STOP.** Do not implement event consumers for Mission, Knowledge, Memory, or other contexts without approved authority.

5. **Distributed transactions** — No RFC, TDS, or ARCH document approves two-phase commit, saga pattern, or cross-service transaction coordination. **STOP.** Do not implement distributed transactions without approved authority.

6. **New bounded contexts** — Only Workforce bounded context is in scope. **STOP.** Do not implement additional bounded contexts without approved RFC and TDS.

7. **New RFCs/TDSs/TDRs** — No new architecture documents are required. **STOP.** Do not create new RFCs, TDSs, TDRs, ARCH documents, or ISPs for this milestone.

## STOP if Architecture Violation

The following conditions require immediate cessation and architectural review:

1. Domain entities crossing IPC boundary — **STOP** per ARCH-0001 TB-2, TDR-0004
2. Business logic in Infrastructure — **STOP** per ARCH-0003 AV-001
3. Domain layer depending on Infrastructure — **STOP** per ARCH-0003 Dependency Contract
4. Application Service bypassing aggregate boundaries — **STOP** per TDS-0002, ARCH-0003
5. Event publication before transaction commit — **STOP** per ISP-0005, ISP-0006
6. Multiple architectural owners for one artifact — **STOP** per ARCH-0002, ARCH-0003 AV-007
7. Workforce aggregate modifying foreign aggregates — **STOP** per TDS-0002, ARCH-0003
8. Transaction ownership moving to Infrastructure — **STOP** per TDS-0004, ISP-0006

---

## 20. Verification Against Prior Milestones

### No Contradictions with MILESTONE-003.0 (Governance)

| Aspect | MILESTONE-003.0 | MILESTONE-004.0 | Consistent? |
|--------|-----------------|-----------------|-------------|
| Foundation milestone pattern | ✅ | ✅ (reuses pattern) | Yes |
| Event publication after commit | ✅ | ✅ (reuses pattern) | Yes |
| Transaction abstraction in Application Layer | ✅ | ✅ (reuses pattern) | Yes |
| In-memory event publisher | ✅ | ✅ (reuses pattern) | Yes |
| In-memory repository stub | ✅ | ✅ (reuses pattern) | Yes |
| Dependency injection | ✅ | ✅ (reuses pattern) | Yes |
| Domain layer independence | ✅ | ✅ | Yes |
| No business logic in Infrastructure | ✅ | ✅ | Yes |
| Architecture enforcement | ✅ | ✅ | Yes |

**Conclusion:** MILESTONE-004.0 is fully consistent with MILESTONE-003.0. No contradictions exist.

### No Contradictions with MILESTONE-002.0 and MILESTONE-002.1

| Aspect | MILESTONE-002.0/002.1 | MILESTONE-004.0 | Consistent? |
|--------|------------------------|-----------------|-------------|
| Event publication pattern | ✅ | ✅ (reuses pattern) | Yes |
| Transaction coordination pattern | ✅ | ✅ (reuses pattern) | Yes |
| Dependency direction | ✅ | ✅ | Yes |
| Architecture enforcement | ✅ | ✅ | Yes |

**Conclusion:** MILESTONE-004.0 is fully consistent with MILESTONE-002.0 and MILESTONE-002.1. No contradictions exist.

### No Contradictions with Organization Implementation

| Aspect | Organization | MILESTONE-004.0 | Consistent? |
|--------|--------------|-----------------|-------------|
| Vertical slice pattern | ✅ | ✅ (reuses pattern) | Yes |
| Domain layer structure | ✅ | ✅ (reuses pattern) | Yes |
| Application layer structure | ✅ | ✅ (reuses pattern) | Yes |
| Infrastructure layer structure | ✅ | ✅ (reuses pattern) | Yes |
| Testing pattern | ✅ | ✅ (reuses pattern) | Yes |
| Dependency direction | ✅ | ✅ | Yes |

**Conclusion:** MILESTONE-004.0 is fully consistent with Organization implementation. No contradictions exist.

---

## 21. Authority Coverage Summary

Every implementation responsibility in this milestone traces to at least one approved authority document:

- **Workforce aggregate structure** → RFC-0015, TDS-0002, ARCH-0002
- **Workforce entities** → RFC-0015, TDS-0002, RFC-0028
- **Workforce value objects** → TDS-0002
- **Workforce repository** → TDS-0002, ISP-0004
- **Workforce domain events** → TDS-0002, ISP-0005
- **Workforce domain services** → TDS-0002
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

## 22. Document Completion

This document is complete.

It establishes the **Implementation Contract** for MILESTONE-004.0 — Workforce Domain Foundation, including:

- Milestone identity and objective
- Authority chain and traceability
- Rationale for implementation sequence
- Workforce responsibilities authorized by existing authority
- Domain scope (aggregate, entities, value objects, repository, events, services)
- Application scope (service, transaction coordination)
- Infrastructure scope (repository, event publisher, transaction stubs)
- Platform/composition scope (deferred)
- Expected crates and files
- Dependencies (all satisfied)
- Testing requirements
- Validation gates
- Completion criteria
- Explicit out-of-scope boundaries
- Authority traceability matrix
- Implementation stop boundaries
- Verification against prior milestones

This document introduces **no new architecture**, **no new technology decisions**, **no RFC**, **no TDS**, **no TDR**, **no ARCH**, and **no ISP**.

Every responsibility traces to one or more approved authority documents in the ForgeOS authority chain.

---

## 23. Authority Documents Inspected

### RFC Series
- RFC-0001 — ForgeOS Genome
- RFC-0015 — Digital Workforce Framework
- RFC-0028 — Competency Management (referenced)

### TDS Series
- TDS-0002 — Domain Model
- TDS-0003 — Organization Model
- TDS-0004 — Application Model

### TDR Series
- TDR-0001 — Programming Language (Rust/Cargo)
- TDR-0003 — Storage Strategy (SQLite/SQLx)
- TDR-0004 — IPC Serialization Strategy (Serde/JSON)

### Architecture Documents
- ARCH-0001 — System Context
- ARCH-0002 — Component Model
- ARCH-0003 — Architecture Enforcement Specification
- ARCH-0004 — Workspace Specification

### Implementation Specifications
- ISP-0001 — Application Service Pattern
- ISP-0004 — Repository Pattern
- ISP-0005 — Domain Event Pattern
- ISP-0006 — Transaction Pattern
- ISP-0007 — Dependency Injection Pattern
- ISP-0008 — Error Handling Pattern
- ISP-0009 — Testing Pattern
- ISP-0010 — Vertical Slice Pattern

### Implementation Documents
- MILESTONE-001 — Create Organization Vertical Slice
- MILESTONE-002.0 — Event Dispatch and Workflow Orchestration
- MILESTONE-002.1 — Transaction Coordination Refinement
- MILESTONE-003.0 — Governance Domain Foundation

### Validation Reports
- WORKFORCE-VALIDATION-REPORT.md
- GOVERNANCE-VALIDATION-REPORT.md
- ARCHITECTURE-CONSOLIDATION-REPORT.md
- REPOSITORY-DRIVEN-IMPLEMENTATION-ROADMAP.md

### Architecture Handbook
- Architecture Handbook — Workforce Bounded Context (explanatory only)

---

## 24. Milestone Objective

Implement the Workforce bounded context as the third fully-implemented bounded context in ForgeOS, establishing the organizational capability foundation required by the Mission execution context.

---

## 25. Scope Summary

This milestone implements:

1. **Workforce domain crate** — complete domain layer with aggregate, entities, value objects, repository interface, domain events, domain services (~20–25 files)
2. **Workforce application crate** — application service with transaction coordination (~4–6 files)
3. **Workforce infrastructure crate** — repository implementation, event publisher, transaction implementation (~4 files)
4. **Workspace updates** — add 3 new workspace members (1 file modified)
5. **Tests** — unit tests for domain logic (~10–20 test files)

**Total:** ~40–60 new source files, ~1 modified file

---

## 26. Completion Criteria

The milestone is complete when:

1. ✅ All domain entities, value objects, and aggregates implement RFC-0015, TDS-0002 specifications
2. ✅ All repository interfaces comply with ISP-0004
3. ✅ All domain events comply with ISP-0005
4. ✅ All application services comply with ISP-0001
5. ✅ Transaction coordination works correctly (ISP-0006, MILESTONE-002.1)
6. ✅ Event publication works correctly (ISP-0005, MILESTONE-002.0)
7. ✅ All tests pass (`cargo test --workspace -- --test-threads=1`)
8. ✅ Code compiles (`cargo check --workspace`)
9. ✅ Architecture compliance verified against WORKFORCE-VALIDATION-REPORT.md
10. ✅ Architecture Office approves completion

---

*End of Document*