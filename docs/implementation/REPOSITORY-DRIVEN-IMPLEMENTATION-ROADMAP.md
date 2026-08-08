# Repository-Driven Implementation Roadmap

**Document Type:** Implementation Roadmap  
**Status:** Approved  
**Date:** 2026-08-08  
**Prepared by:** Architecture Office

---

## 1. Current Implementation Baseline

### Completed Implementation

The repository has completed the following implementation work:

**Milestone 001 — Organization Domain Foundation:**
- Rust workspace initialization (MILESTONE-001.1)
- Crate boundary plan (MILESTONE-001.2)
- Crate initialization plan (MILESTONE-001.3)
- Cargo member initialization (MILESTONE-001.4)
- Organization domain foundation (MILESTONE-001.5.2)
- Organization domain test validation (MILESTONE-001.5.3)

**Milestone 001.6–001.9 — Create Organization Vertical Slice:**
- Create Organization application layer (MILESTONE-001.6)
- Organization infrastructure layer (MILESTONE-001.7)
- Organization platform layer (MILESTONE-001.8)
- Organization presentation layer (MILESTONE-001.9)

**Milestone 002.0 — Event Dispatch and Workflow Orchestration:**
- Event publisher abstraction
- In-memory event publisher implementation
- Event dispatch after transaction commit
- Workflow orchestration pattern

**Milestone 002.1 — Transaction Coordination Refinement:**
- Transaction trait in Application Layer
- SqlxTransaction implementation in Infrastructure
- Explicit transaction lifecycle (begin, commit, rollback)
- Dependency wiring through Platform composition root

### Current Implementation Structure

```
implementation/rust/
├── Cargo.toml (workspace)
├── Cargo.lock
├── domains/
│   ├── .gitkeep
│   └── organization-domain/ (IMPLEMENTED)
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── organization.rs
│           ├── value_objects/
│           ├── org_domain_event/
│           ├── organization_created/
│           ├── organization_repository/
│           ├── id_generation/
│           └── errors/
├── applications/
│   ├── .gitkeep
│   └── create-organization/ (IMPLEMENTED)
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── service.rs
│           └── transaction.rs
├── infrastructure/
│   ├── .gitkeep
│   └── organization/ (IMPLEMENTED)
│       ├── Cargo.toml
│       ├── lib.rs
│       ├── repository.rs
│       ├── event_publisher.rs
│       └── transaction.rs
├── platform/
│   ├── .gitkeep
│   └── desktop/ (IMPLEMENTED)
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── composition.rs
│           ├── commands.rs
│           └── errors.rs
└── presentation/
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── ui.rs
        ├── ipc.rs
        ├── view_model.rs
        ├── composition.rs
        └── errors/
```

### Implementation Status Summary

**Implemented:** 1 of 7 bounded contexts (Organization)
**Not Implemented:** 6 of 7 bounded contexts (Mission, Governance, Workforce, Knowledge, Memory, Process)

**Test Status:**
- `cargo check --workspace` passes (2 non-blocking warnings)
- `cargo test --workspace -- --test-threads=1` passes (132 tests passing, 0 failures)

---

## 2. Remaining Approved Implementation Work

### Bounded Contexts Requiring Implementation

All six remaining bounded contexts are architecturally complete and ready for implementation:

1. **Mission** — Organizational execution
2. **Governance** — Organizational authority
3. **Workforce** — Organizational capability
4. **Knowledge** — Organizational knowledge
5. **Memory** — Institutional memory
6. **Process** — Organizational workflow

### Implementation Artifacts Required Per Bounded Context

Each bounded context requires implementation of:

**Domain Layer:**
- Aggregate root
- Entities (2–6 per context)
- Value objects (6–7 per context)
- Repository interface (domain-owned)
- Domain events (4–7 published, 3–4 consumed)
- Domain services (2–4 per context)
- Errors module

**Application Layer:**
- Application service(s)
- Command handlers
- Query handlers
- DTOs
- Transaction coordination

**Infrastructure Layer:**
- Repository implementation (SQLx/SQLite)
- Event publisher implementation
- Transaction implementation

**Platform Layer:**
- Dependency wiring
- Composition root updates

**Presentation Layer:**
- UI commands
- View models
- IPC handlers

### Implementation Scope Summary

| Bounded Context | Domain Crate | Application Crate | Infrastructure Crate | Estimated Entities | Estimated Events |
|-----------------|--------------|-------------------|---------------------|--------------------|------------------|
| Mission | mission-domain | execute-mission (TBD) | mission | 6 | 8 |
| Governance | governance-domain | approve-decision (TBD) | governance | 6 | 6 |
| Workforce | workforce-domain | manage-workforce (TBD) | workforce | 6 | 7 |
| Knowledge | knowledge-domain | promote-knowledge (TBD) | knowledge | 6 | 6 |
| Memory | memory-domain | record-memory (TBD) | memory | 6 | 4 |
| Process | process-domain | execute-process (TBD) | process | 6 | 7 |

**Total:** 6 domain crates, 6 application crates (estimated), 6 infrastructure crates, ~36 entities, ~38 domain events

---

## 3. Implementation Sequence

### Recommended Implementation Order

Based on architectural dependencies (TDS-0002, TDS-0003, ARCH-0002) and the dependency model defined in the Architecture Consolidation Report:

**Phase 1 — Foundation Contexts (implement first):**
1. **Governance** — Depends only on Organization; consumed by all other contexts
2. **Workforce** — Depends only on Organization; consumed by Mission

**Phase 2 — Execution Contexts (implement second):**
3. **Mission** — Depends on Organization, Workforce, Governance; consumed by Process, Knowledge, Memory

**Phase 3 — Learning Contexts (implement third):**
4. **Knowledge** — Depends on Organization; consumed by Memory
5. **Memory** — Depends on Organization, Knowledge

**Phase 4 — Process Context (implement last):**
6. **Process** — Depends on Organization, Mission, Governance

### Dependency Graph

```
                     Organization
                          │
       ┌──────────────────┼──────────────────┐
       ▼                  ▼                  ▼
  Mission            Workforce         Governance
       │                  │                  │
       └──────────────┬───┴──────────┐
                      ▼              ▼
                  Process        Knowledge
                      │              │
                      └──────┬───────┘
                             ▼
                          Memory
```

**Legend:**
- Solid arrows indicate "depends on" relationships
- Governance has no downstream dependencies (can be implemented first)
- Workforce has minimal dependencies (can be implemented early)
- Mission has multiple dependencies (must wait for Governance and Workforce)
- Process has the most dependencies (must be implemented last)

### Rationale for Sequence

1. **Governance first** — Provides approval authority consumed by all other contexts; no dependencies beyond Organization
2. **Workforce second** — Provides capability assignment consumed by Mission; minimal dependencies
3. **Mission third** — Core execution context; requires Governance and Workforce to be available
4. **Knowledge fourth** — Learning context; depends only on Organization
5. **Memory fifth** — Requires Knowledge to be available for institutional preservation
6. **Process last** — Orchestration context; requires Mission and Governance to be available

---

## 4. Dependencies Between Bounded Contexts and Implementation Slices

### Inter-Context Dependencies

| Context | Depends On | Consumed By | Authority Source |
|---------|------------|-------------|------------------|
| Organization | None | All contexts | RFC-0001, RFC-0004, TDS-0002, TDS-0003, ARCH-0002 |
| Governance | Organization | Mission, Process, Workforce, Knowledge, Memory | RFC-0007, TDS-0002, TDS-0003, ARCH-0002 |
| Workforce | Organization | Mission | RFC-0015, TDS-0002, TDS-0003, ARCH-0002 |
| Mission | Organization, Workforce, Governance | Process, Knowledge, Memory | RFC-0021, TDS-0002, TDS-0003, ARCH-0002 |
| Knowledge | Organization | Memory | RFC-0002, RFC-0003, RFC-0009, TDS-0002, TDS-0003, ARCH-0002 |
| Memory | Organization, Knowledge | None | RFC-0008, RFC-0024, TDS-0002, TDS-0003, ARCH-0002 |
| Process | Organization, Mission, Governance | None | RFC-0022, TDS-0002, TDS-0003, ARCH-0002 |

### Intra-Context Dependencies (Vertical Slice Pattern)

Each bounded context implementation follows the vertical slice pattern (ISP-0010) with layer dependencies:

**Domain Layer:**
- No internal dependencies
- Owns all business rules
- Defines repository interfaces

**Application Layer:**
- Depends on Domain Layer (interfaces only)
- Orchestrates use cases
- Coordinates transactions

**Infrastructure Layer:**
- Depends on Domain Layer (interfaces)
- Implements repository contracts
- Provides technical implementations

**Platform Layer:**
- Depends on all layers
- Wires dependencies
- Manages composition root

**Presentation Layer:**
- Depends on Application Layer (interfaces only)
- Translates user intent to application commands
- Renders responses

### Cross-Cutting Concerns

**Event System:**
- All contexts use the event publisher abstraction (ISP-0005)
- Event dispatch occurs after successful transaction commit (MILESTONE-002.0)
- Cross-context communication occurs through domain events

**Transaction Coordination:**
- All application services use the Transaction trait (MILESTONE-002.1)
- Transaction lifecycle: begin → execute → commit/rollback
- Event publication after successful commit

**Repository Pattern:**
- All domains follow repository pattern (ISP-0004)
- Repository interfaces owned by Domain Layer
- Repository implementations owned by Infrastructure Layer

---

## 5. Next Logical Implementation Milestone

### Recommended Next Milestone: MILESTONE-003.0 — GOVERNANCE DOMAIN FOUNDATION

### Milestone Objective

Implement the Governance bounded context as the second fully-implemented bounded context, establishing the organizational authority foundation required by all other bounded contexts.

### Milestone Scope

**In Scope:**
1. Governance domain crate (`governance-domain`)
   - Governance aggregate root
   - Decision entity
   - Policy entity
   - Standard entity
   - DelegatedAuthority entity
   - ApprovalRecord entity
   - GovernanceRule entity
   - Value objects (DecisionId, PolicyId, AuthorityLevel, ApprovalStatus, GovernanceScope, StandardIdentifier)
   - GovernanceRepository interface
   - Domain events (DecisionApproved, DecisionRejected, PolicyPublished, PolicyRetired, AuthorityDelegated, AuthorityRevoked)
   - Domain services (PolicyEvaluationService, GovernanceValidationService, AuthorityManagementService, DecisionEvaluationService)
   - Errors module

2. Governance application crate (`approve-decision` or similar)
   - Application service(s) for governance operations
   - Command handlers
   - Query handlers
   - DTOs
   - Transaction coordination using existing Transaction trait

3. Governance infrastructure crate (`governance`)
   - GovernanceRepository implementation (SQLx/SQLite)
   - Event publisher implementation
   - Transaction implementation

4. Platform updates
   - Wire Governance dependencies
   - Update composition root

5. Tests
   - Unit tests for domain logic
   - Integration tests for repository
   - Application service tests

**Out of Scope:**
- Presentation layer for Governance (deferred to future milestone)
- Additional bounded contexts (Mission, Workforce, Knowledge, Memory, Process)
- Event broker integration (requires future RFC/TDS approval)
- Event persistence (requires future RFC/TDS approval)
- Cross-context event consumption (deferred to future milestone)

### Authority Governing This Milestone

**Primary Authority:**
- **RFC-0007** — Decision Authority Matrix (defines decision ownership, approval authority, delegation)
- **TDS-0002** — Domain Model (defines Governance aggregate, entities, value objects, repository contract, domain events, domain services)
- **TDS-0003** — Organization Model (defines Governance Unit responsibilities, authority, ownership)
- **ARCH-0002** — Component Model (defines Governance Domain as implementation owner)
- **ARCH-0003** — Architecture Enforcement Specification (enforces dependency contracts)

**Supporting Authority:**
- **RFC-0001** — ForgeOS Genome (establishes Governance as a first-class concept)
- **RFC-0006** — Executive Meeting Protocol (defines executive approval workflows)
- **ISP-0001** — Application Service Pattern
- **ISP-0002** — Command Handler Pattern
- **ISP-0003** — Query Handler Pattern
- **ISP-0004** — Repository Pattern
- **ISP-0005** — Domain Event Pattern
- **ISP-0006** — Transaction Pattern
- **ISP-0007** — Dependency Injection Pattern
- **ISP-0008** — Error Handling Pattern
- **ISP-0009** — Testing Pattern
- **ISP-0010** — Vertical Slice Pattern

### Expected Repository Areas/Files Affected

**New Directories:**
```
implementation/rust/domains/governance-domain/
implementation/rust/applications/approve-decision/ (or similar)
implementation/rust/infrastructure/governance/
```

**Modified Files:**
- `implementation/rust/Cargo.toml` — Add new workspace members
- `implementation/rust/platform/desktop/src/composition.rs` — Wire Governance dependencies
- `implementation/rust/platform/desktop/src/commands.rs` — Add Governance commands (if presentation included)

**Estimated File Count:**
- ~20–25 new source files in governance-domain
- ~10–15 new source files in application layer
- ~5–10 new source files in infrastructure layer
- ~2–3 modified files in platform layer

---

## 6. Validation and Completion Gates

### Gate 1: Architecture Compliance

**Criteria:**
- All domain entities, value objects, and aggregates match TDS-0002 specifications
- All repository interfaces match TDS-0002 contracts
- All domain events match TDS-0002 and ARCH-0002 specifications
- All domain services match TDS-0002 specifications
- All ownership rules comply with TDS-0003 and ARCH-0002
- All dependencies comply with ARCH-0003

**Verification:**
- Architecture review against validation reports
- Cross-reference with GOVERNANCE-VALIDATION-REPORT.md
- Dependency graph validation

### Gate 2: Implementation Standards Compliance

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

### Gate 3: Transaction Coordination

**Criteria:**
- All application services use Transaction trait
- Transaction lifecycle (begin, commit, rollback) implemented correctly
- Event publication occurs after successful commit
- Rollback on errors prevents event publication

**Verification:**
- Transaction coordination tests pass
- Event dispatch tests pass
- Rollback scenarios tested

### Gate 4: Test Coverage

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

### Gate 5: Documentation

**Criteria:**
- Implementation report documents all decisions
- Milestone report documents scope, authority, and completion
- Architecture compliance documented
- Known issues documented

**Verification:**
- Implementation report complete
- Milestone report complete
- Documentation review

### Completion Criteria

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

## 7. Out-of-Scope Boundaries

### Explicitly Out of Scope for This Milestone

**Bounded Contexts:**
- Mission domain (future milestone)
- Workforce domain (future milestone)
- Knowledge domain (future milestone)
- Memory domain (future milestone)
- Process domain (future milestone)

**Presentation Layer:**
- Governance UI (deferred to future milestone)
- Governance commands in presentation layer
- Governance view models
- Governance IPC handlers

**Cross-Cutting Concerns:**
- Event broker integration (requires future RFC/TDS approval per NEXT_SESSION.md)
- Event persistence (requires future RFC/TDS approval per NEXT_SESSION.md)
- Cross-context event consumption (deferred to future milestone)
- Additional domain events beyond Governance context (deferred to future milestone)

**Architecture:**
- New RFCs (deferred until implementation experience requires it per PROJECT_STATUS.md)
- New TDSs (deferred until implementation experience requires it)
- New TDRs (deferred until implementation experience requires it)
- Architecture modifications (repository authority is complete per ARCHITECTURE-CONSOLIDATION-REPORT.md)
- Design Packages (not required; authority is complete)

**Technology:**
- Frontend framework selection (not required for domain implementation)
- AI provider integration (Phase 3 per ROADMAP.MD)
- Cloud deployment (not in scope for MVP)
- Multi-tenant architecture (not in scope for MVP)

### In-Scope Boundaries

**Only the following are in scope:**
- Governance domain crate (domain layer)
- Governance application crate (application layer)
- Governance infrastructure crate (infrastructure layer)
- Platform dependency wiring
- Unit tests, integration tests, application service tests
- Implementation documentation

---

## 8. Implementation Work vs. Architecture Work

### Architecture Work: COMPLETE

**Completed:**
- ✅ RFC-0001 through RFC-0045 (approved)
- ✅ TDS-0001 through TDS-0004 (approved)
- ✅ TDR-0001 through TDR-0006 (approved)
- ✅ ARCH-0001 through ARCH-0004 (approved)
- ✅ ISP-0001 through ISP-0010 (approved)
- ✅ Architecture Consolidation Phase complete
- ✅ All bounded contexts validated
- ✅ No architectural gaps identified
- ✅ Repository authority is complete

**Status:** No additional architecture work is required before implementation.

### Implementation Work: REQUIRED

**Remaining Implementation Work:**

1. **Governance Domain** (next milestone)
   - Domain crate: ~20–25 files
   - Application crate: ~10–15 files
   - Infrastructure crate: ~5–10 files
   - Platform updates: ~2–3 files
   - Tests: ~20–30 test files

2. **Workforce Domain** (future milestone)
   - Similar scope to Governance

3. **Mission Domain** (future milestone)
   - Similar scope to Governance

4. **Knowledge Domain** (future milestone)
   - Similar scope to Governance

5. **Memory Domain** (future milestone)
   - Similar scope to Governance

6. **Process Domain** (future milestone)
   - Similar scope to Governance

**Total Remaining Implementation:** ~120–150 source files across 6 bounded contexts

---

## 9. Dependencies and Prerequisites

### Prerequisites for Next Milestone

**Completed:**
- ✅ Organization domain implemented
- ✅ Transaction coordination pattern established (MILESTONE-002.1)
- ✅ Event dispatch pattern established (MILESTONE-002.0)
- ✅ Repository authority complete (ARCHITECTURE-CONSOLIDATION-REPORT.md)
- ✅ Governance bounded context validated (GOVERNANCE-VALIDATION-REPORT.md)
- ✅ All ISP patterns defined (ISP-0001 through ISP-0010)

**Not Required:**
- ❌ Additional RFCs (authority is sufficient)
- ❌ Additional TDSs (authority is sufficient)
- ❌ Additional TDRs (authority is sufficient)
- ❌ Design Packages (authority is sufficient)
- ❌ Other bounded contexts (Governance has no dependencies on them)

### External Dependencies

**Approved and Installed:**
- Rust/Cargo (TDR-0001)
- SQLx/SQLite (TDR-0003)
- Tauri 2.x (TDR-0002)
- Serde/JSON (TDR-0004)

**No Additional External Dependencies Required**

---

## 10. Risk Assessment

### Low Risk

**Architecture Risk:** LOW
- Repository authority is complete
- No architectural gaps identified
- All bounded contexts validated
- Clear implementation patterns established

**Dependency Risk:** LOW
- Governance has minimal dependencies (Organization only)
- No cross-context dependencies for initial implementation
- Clear implementation sequence defined

**Technical Risk:** LOW
- Transaction coordination pattern proven (MILESTONE-002.1)
- Event dispatch pattern proven (MILESTONE-002.0)
- Repository pattern proven (Organization domain)
- Testing patterns proven (132 tests passing)

### Medium Risk

**Implementation Complexity:** MEDIUM
- 6 bounded contexts remaining (~120–150 files)
- Significant implementation work remaining
- Requires disciplined adherence to architecture

**Knowledge Transfer:** MEDIUM
- Implementation team must understand DDD, aggregate boundaries, repository pattern
- Architecture Office consultation may be required

### Mitigation Strategies

1. **Follow established patterns** — Use Organization domain and Create Organization vertical slice as templates
2. **Reference validation reports** — Use bounded context validation reports to ensure compliance
3. **Incremental implementation** — Implement one bounded context at a time
4. **Test-driven development** — Follow TESTING_STANDARD.md
5. **Architecture Office consultation** — Consult Architecture Office for architectural questions

---

## 11. Recommendations

### Primary Recommendation

**Implement Governance Domain (MILESTONE-003.0) as the next milestone.**

### Rationale

1. **Minimal Dependencies** — Governance depends only on Organization (already implemented)
2. **Maximum Value** — Governance provides approval authority consumed by all other contexts
3. **Clear Authority** — RFC-0007, TDS-0002, TDS-0003, ARCH-0002 provide complete specification
4. **Proven Pattern** — Follow the same pattern as Organization domain implementation
5. **Enables Future Work** — Mission and Process depend on Governance

### Implementation Approach

1. **Follow Vertical Slice Pattern** — Implement domain, application, infrastructure, platform layers
2. **Follow Transaction Coordination Pattern** — Use Transaction trait from MILESTONE-002.1
3. **Follow Event Dispatch Pattern** — Use event publisher from MILESTONE-002.0
4. **Follow Repository Pattern** — Use Organization repository as template
5. **Follow Testing Patterns** — Use Organization tests as template
6. **Reference Validation Report** — Use GOVERNANCE-VALIDATION-REPORT.md for compliance verification

### Success Criteria

The Governance domain implementation is complete when:
1. All domain entities, value objects, and aggregates implement TDS-0002 specifications
2. All repository interfaces and implementations comply with ISP-0004
3. All domain events comply with ISP-0005
4. All application services comply with ISP-0001
5. Transaction coordination works correctly (ISP-0006)
6. All tests pass (`cargo test --workspace -- --test-threads=1`)
7. Code compiles (`cargo check --workspace`)
8. Architecture Office approves completion

---

## 12. Conclusion

The ForgeOS repository is **ready for implementation** of the remaining six bounded contexts.

**Current State:**
- Architecture: Complete ✅
- Authority: Complete ✅
- Implementation: 1/7 bounded contexts complete (Organization)
- Remaining: 6 bounded contexts (Mission, Governance, Workforce, Knowledge, Memory, Process)

**Next Step:**
- Implement Governance domain (MILESTONE-003.0)
- Follow established patterns from Organization domain
- Use existing transaction coordination and event dispatch infrastructure
- Reference GOVERNANCE-VALIDATION-REPORT.md for compliance

**No Blockers:**
- No architectural gaps
- No missing authority
- No technology decisions required
- No design packages required

The repository authority is the source of truth. Implementation shall follow the approved baseline without inventing new architecture, technology decisions, or implementation responsibilities.

---

*End of Repository-Driven Implementation Roadmap*