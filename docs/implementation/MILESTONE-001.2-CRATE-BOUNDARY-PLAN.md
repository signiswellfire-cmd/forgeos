# Milestone 001.2 — Crate Boundary Plan

**Status:** Proposed — Approval Required  
**Scope:** Crate-boundary specification for the Create Organization vertical slice. This document creates no Cargo member, source file, dependency, or framework selection.

---

# Objective

Define the smallest set of Rust crate ownership boundaries needed to implement the approved Create Organization vertical slice while preserving TDS layer responsibilities, ARCH-0003 dependency enforcement, ARCH-0004 workspace topology, and ISP implementation patterns.

The plan defines **semantic crate roles**, not Cargo package identifiers. ARCH-0004 explicitly defines workspace categories rather than mandatory crate names. Package names, manifests, Rust edition, and Cargo resolver remain deferred to the approved crate-creation milestone.

---

# Minimum Crates Required

Four crates are required for the first complete slice:

| Semantic crate role | Architectural owner | Why it is required now |
|---|---|---|
| Organization Domain crate | Organization Domain | Owns the Organization aggregate, value objects, domain errors, `OrganizationCreated`, and the Domain-owned `OrganizationRepository` contract. |
| Create Organization Application crate | Application Services | Owns the command, command handler, application service, application result/error translation, and transaction coordination for one use case. |
| Organization Infrastructure crate | Infrastructure Domain | Implements the Domain-owned repository contract, SQLite/SQLx persistence mapping, migration execution support, transaction participation, and technical error translation. |
| Desktop Platform crate | Platform Domain | Owns dependency composition, runtime bootstrap, and the Tauri/DTO boundary for the approved `createOrganization` command. |

No Shared, Presentation, Plugin, AI, tooling, query, or second domain crate is required for Milestone 1. A Shared crate is specifically excluded: no approved common behavior has yet demonstrated the need for it.

---

# Proposed Workspace Structure

ARCH-0004 governs the physical topology. The semantic placeholders below are intentionally not assumed Cargo package names.

```text
implementation/
└── rust/
    ├── Cargo.toml
    ├── domains/
    │   └── <organization-domain-crate>/
    ├── applications/
    │   └── <create-organization-application-crate>/
    ├── infrastructure/
    │   └── <organization-infrastructure-crate>/
    └── platform/
        └── <desktop-platform-crate>/
```

No `crates/` directory is proposed. Adding one would duplicate the categories established by ARCH-0004 and conflict with the approved workspace layout. The root `Cargo.toml` will list these four members only after this plan is approved.

---

# Dependency Graph

```text
Desktop Platform crate
  ├── Create Organization Application crate
  │     └── Organization Domain crate
  └── Organization Infrastructure crate
        └── Organization Domain crate
```

The Platform crate composes the Application and Infrastructure crates at runtime; it does not own business behavior. The Application crate invokes only Domain-owned contracts. The Infrastructure crate implements Domain-owned contracts and is injected through the Platform composition root.

No dependency is permitted from Domain to Application, Infrastructure, Platform, Tauri, SQLx, or Serde IPC. The Infrastructure crate does not depend on the Application crate; the Platform crate connects their implementations at composition time.

This graph implements the TDS-0001 allowed dependency directions (Application → Domain, Infrastructure → Domain interfaces, Platform → Infrastructure) while also allowing the Platform boundary to invoke the Application entry point. It does not treat the illustrative dependency graph in the milestone request as a substitute for the TDS dependency contracts.

---

# Crate Contracts and Dependency Rules

## Organization Domain crate

**Owned public interfaces:**

* Organization aggregate behavior and its constructor/factory boundary;
* approved Organization value objects and domain errors;
* immutable `OrganizationCreated` event contract;
* `OrganizationRepository` repository contract.

**Allowed dependencies:** only the minimum Domain-side primitives, value objects, and event contracts required by the Organization bounded context. The initial crate should require no external library.

**Forbidden dependencies:** Application, Infrastructure, Platform, Tauri, SQLx, Serde IPC, Presentation, OS APIs, other bounded contexts, and concrete persistence types.

## Create Organization Application crate

**Owned public interfaces:**

* Create Organization command;
* command handler;
* Create Organization application service;
* application result and application-level error boundary.

**Allowed dependencies:** Organization Domain public interfaces and approved application abstractions needed for orchestration. Any transaction or event-dispatch abstraction used by the application must be a published interface, not a concrete Infrastructure type.

**Forbidden dependencies:** SQLx, concrete repositories, Tauri, desktop runtime internals, OS APIs, database types, and Organization business-rule implementation outside the aggregate.

## Organization Infrastructure crate

**Owned public interfaces:** no business-oriented public interface. It exposes only concrete implementations of contracts owned by the Organization Domain or approved application abstractions:

* SQLx-backed `OrganizationRepository` implementation;
* migration execution support;
* transaction participation and technical error translation;
* event-dispatch implementation, if required by the approved transaction/event contract.

**Allowed dependencies:** Organization Domain contracts, approved Platform bootstrap abstractions, SQLx with the SQLite and Tokio support selected by TDR-0003, and Rust standard-library facilities required by the adapter.

**Forbidden dependencies:** Application implementation, Presentation, Tauri command types, Domain implementation internals, and business-rule ownership.

## Desktop Platform crate

**Owned public interfaces:** runtime/bootstrap entry points and the Tauri `createOrganization` boundary that maps dedicated request/response/error DTOs to the Application crate's public entry point.

**Allowed dependencies:** Tauri and Serde IPC according to TDR-0002 and TDR-0004; the Create Organization Application crate; the Organization Infrastructure crate for dependency composition; and runtime configuration/diagnostics required for bootstrap.

**Forbidden dependencies:** Organization business behavior, SQL statements, direct database access, repository-contract ownership, aggregate mutation outside Application orchestration, and frontend-framework code.

---

# Relationship to the Create Organization Vertical Slice

| Slice concern | Owning crate role | Authority |
|---|---|---|
| Aggregate, value objects, singleton invariant, repository contract, domain event | Organization Domain | TDS-0002; `MILESTONE-001-DOMAIN-DECISIONS.md`; ISP-0004 and ISP-0005 |
| Command, handler, service, workflow, transaction boundary | Create Organization Application | TDS-0004; ISP-0001, ISP-0002, ISP-0006, ISP-0008 |
| SQLite repository, migrations, transaction participation | Organization Infrastructure | TDR-0003; ISP-0004, ISP-0006, ISP-0008 |
| Tauri command, DTO serialization, composition root | Desktop Platform | TDR-0002, TDR-0004; ISP-0007, ISP-0008 |
| Domain, application, infrastructure, and end-to-end tests | Tests colocated with their owning crate and workspace integration tests when needed | ISP-0009, ISP-0010 |

Tests do not justify a fifth production crate. Unit tests remain with their owning crate; integration and architectural tests are added only when the first member crates make them meaningful.

---

# Traceability Matrix

| Semantic crate role | TDS | Architecture documents | ISP documents |
|---|---|---|---|
| Organization Domain | TDS-0001 Domain Layer; TDS-0002 Organization context, aggregate, repository, event, and invariants | `component-model.md`, `domain-model.md`, `aggregate-boundaries.md`, `entity-relationships.md`, `domain-event-model.md`, `architecture-enforcement-specification.md` | ISP-0004, ISP-0005, ISP-0008, ISP-0009, ISP-0010 |
| Create Organization Application | TDS-0001 Application Layer; TDS-0004 Application Model | `application-model.md`, `application-services.md`, `command-query-model.md`, `workflow-orchestration.md`, `architecture-enforcement-specification.md` | ISP-0001, ISP-0002, ISP-0006, ISP-0007, ISP-0008, ISP-0009, ISP-0010 |
| Organization Infrastructure | TDS-0001 Infrastructure Layer; TDS-0002 persistence ownership; TDS-0004 transaction coordination | `persistence-model.md`, `component-model.md`, `architecture-enforcement-specification.md` | ISP-0004, ISP-0005, ISP-0006, ISP-0008, ISP-0009, ISP-0010 |
| Desktop Platform | TDS-0001 Platform Layer; TDS-0004 external application boundary | `system-context.md`, `component-model.md`, `architecture-enforcement-specification.md`, `workspace-specification.md` | ISP-0001, ISP-0002, ISP-0007, ISP-0008, ISP-0009, ISP-0010 |

TDR-0001 supplies Rust/Cargo, TDR-0002 supplies Tauri, TDR-0003 supplies SQLite/SQLx storage, TDR-0004 supplies Serde/JSON DTO IPC, and TDR-0005 supplies the `implementation/rust/` workspace location.

---

# Architecture Validation

The planned boundaries satisfy ARCH-0003:

* each planned crate has one architectural owner;
* Organization Domain has no Infrastructure, Presentation, or foreign-domain dependency;
* Application uses Domain contracts and has no repository-implementation or desktop-runtime dependency;
* Infrastructure implements contracts but does not define business contracts;
* Platform hosts runtime composition without acquiring Organization behavior;
* SQLx remains Infrastructure-only; Tauri and Serde IPC remain Platform-only;
* there is no circular dependency and no Shared crate used as an ownership bypass.

When Cargo becomes available and members are created, these rules must be enforced through workspace dependency inspection, compile-time restrictions, architecture tests, and the repository-time checks defined by ARCH-0003.

---

# Codex Readiness

**Can a senior Rust engineer create the first crates without inventing architecture?**

**Yes, after approval of this plan.** The minimum number of crate boundaries, their ownership, physical locations, permitted direction of dependency, and responsibility for every Create Organization layer are defined.

The following implementation details remain intentionally deferred and are not blockers to creating empty member crates:

1. Cargo package identifiers, Rust edition, and resolver setting; ARCH-0004 does not prescribe them.
2. Authentication and authorization mechanism; it must be resolved before exposing the user-facing command for operational use.
3. The Cargo toolchain must be installed before the workspace can be parsed and its dependency graph validated.

No new framework, production dependency, domain boundary, or business behavior is selected by this plan.

---

# References

* TDS-0001 — System Architecture
* TDS-0002 — Domain Model
* TDS-0004 — Application Model
* TDR-0001 through TDR-0005
* `docs/architecture/architecture-enforcement-specification.md`
* `docs/architecture/workspace-specification.md`
* ISP-0001 through ISP-0010
* `docs/implementation/MILESTONE-001-CREATE-ORGANIZATION.md`
* `docs/implementation/MILESTONE-001-DOMAIN-DECISIONS.md`
