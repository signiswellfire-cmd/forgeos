# Milestone 001.3 — Cargo Crate Initialization Plan

**Status:** Proposed — Approval Required  
**Scope:** Concrete Cargo member mapping for the approved Milestone 1.2 crate boundaries. This plan creates no Cargo member manifest, source file, dependency lockfile, or business behavior.

---

# Objective

Map the four approved semantic crate roles for the Create Organization vertical slice to concrete Cargo workspace member paths, package names, Rust crate names, edition, resolver, and minimal dependencies.

This plan is a technology-specific realization of the approved boundaries. It does not add a fifth production crate, alter ownership, create a Shared crate, or select any technology beyond the approved Rust, Tauri, SQLx/SQLite, and Serde decisions.

---

# Proposed Workspace Member Mapping

All members reside below the canonical workspace root, `implementation/rust/`, established by TDR-0005.

| Semantic role | Member path | Cargo package name | Rust crate name | Architectural owner |
|---|---|---|---|---|
| Organization Domain crate | `domains/organization-domain` | `forgeos-organization-domain` | `forgeos_organization_domain` | Organization Domain |
| Create Organization Application crate | `applications/create-organization` | `forgeos-create-organization-application` | `forgeos_create_organization_application` | Application Services |
| Organization Infrastructure crate | `infrastructure/organization` | `forgeos-organization-infrastructure` | `forgeos_organization_infrastructure` | Infrastructure Domain |
| Desktop Platform crate | `platform/desktop` | `forgeos-desktop-platform` | `forgeos_desktop_platform` | Platform Domain |

The member paths retain ARCH-0004 categories. Hyphenated package names and their underscore-equivalent Rust crate names follow Cargo conventions. These identifiers are explicit proposals; they become canonical only when this plan is approved and the member manifests are created.

---

# Workspace Configuration

## Rust edition

**Proposed:** Rust edition **2024** for all four members.

Using one edition across the initial workspace keeps compilation behavior and lint interpretation consistent. TDR-0001 selects Rust but deliberately does not name an edition; therefore this is a bounded technology detail proposed by this plan, not an implied pre-existing architecture rule.

## Cargo resolver

**Proposed:** Cargo resolver **`3`** at the workspace root.

Resolver `3` corresponds to the proposed Rust 2024 edition and keeps feature resolution centralized in the workspace manifest. No Cargo feature is introduced by this plan.

## Workspace members

When approved, the workspace root will list exactly these four paths:

```text
domains/organization-domain
applications/create-organization
infrastructure/organization
platform/desktop
```

No default member is selected. No `[workspace.dependencies]`, package metadata, feature flags, profile overrides, or workspace-wide lint policy is proposed at this stage.

---

# Initial Dependency Requirements

Dependencies are stated as capability requirements, not version pins. Exact compatible versions are selected only when the manifests are created, then locked by Cargo and verified against the approved TDRs.

| Member | Internal dependencies | External dependencies permitted initially | Explicitly excluded |
|---|---|---|---|
| `forgeos-organization-domain` | None | Rust standard library only | SQLx, Tauri, Serde, Serde JSON, database drivers, OS APIs, Application, Infrastructure, Platform |
| `forgeos-create-organization-application` | `forgeos-organization-domain` | Rust standard library only | SQLx, Tauri, Serde IPC, database drivers, concrete repository adapters, OS APIs |
| `forgeos-organization-infrastructure` | `forgeos-organization-domain` | SQLx with SQLite, migration, and Tokio-runtime support as selected by TDR-0003 | Tauri, Serde IPC, Presentation, frontend libraries, business-rule libraries |
| `forgeos-desktop-platform` | `forgeos-create-organization-application`; `forgeos-organization-infrastructure` | Tauri 2.x and Serde with derive support, as selected by TDR-0002 and TDR-0004 | SQLx, direct database access, frontend framework, business-rule libraries |

`serde_json` is not listed as a direct dependency. TDR-0004 chooses JSON over the Tauri command boundary, but this plan does not add a direct serialization implementation dependency unless the approved Tauri command implementation later requires it.

---

# Dependency Graph

```text
forgeos-desktop-platform
  ├── forgeos-create-organization-application
  │     └── forgeos-organization-domain
  └── forgeos-organization-infrastructure
        └── forgeos-organization-domain
```

Dependency rules:

* Domain has no internal or external technical dependency.
* Application depends only on Domain public interfaces and values.
* Infrastructure depends on Domain public contracts and approved SQLx functionality.
* Platform composes Application and Infrastructure and owns Tauri/Serde IPC.
* Infrastructure never depends on Platform, Application implementation, or Tauri; Platform is the only composition root.
* No cycle, direct database access from Platform, or dependency from Domain upward is permitted.

---

# Validation Against Approved Authority

| Authority | Validation result |
|---|---|
| TDR-0001 | All members are Rust/Cargo packages; business logic remains independent of Infrastructure. |
| TDR-0003 | SQLite and SQLx are isolated to the Infrastructure member; migration and transaction implementation remain there. |
| TDR-0004 | Serde and Tauri command transport are isolated to the Platform member; Domain values are not IPC payloads. |
| ARCH-0003 | Each member has one owner; Domain has no forbidden dependency; Application has no concrete repository or desktop runtime dependency; Infrastructure implements rather than owns contracts; Platform has no business ownership. |
| ARCH-0004 | All paths remain under `implementation/rust/` and within the four approved categories; no redundant `crates/` hierarchy or Shared category is introduced. |
| ISP-0001 through ISP-0010 | Application, repository, event, transaction, DI, error, test, and vertical-slice responsibilities remain assigned to their approved ownership boundaries. |

---

# Relationship to the Create Organization Vertical Slice

The four members provide exactly one ownership location for every approved slice concern:

| Slice concern | Member |
|---|---|
| Aggregate, value objects, domain errors, domain event, repository contract | `forgeos-organization-domain` |
| Command, handler, application service, application result | `forgeos-create-organization-application` |
| SQLx repository adapter, migrations, storage errors, transaction participation | `forgeos-organization-infrastructure` |
| Tauri command, request/response/error DTOs, dependency composition | `forgeos-desktop-platform` |

Tests remain colocated with the members they verify. This plan adds no dedicated test package because ISP-0009 requires tests to preserve the ownership boundary under test.

---

# Unresolved Decisions

1. **Approval of the Rust 2024 edition and resolver `3`:** TDR-0001 does not specify these values. Approval of this plan records them as the initial workspace configuration; otherwise a TDR amendment or new TDR is required before manifest creation.
2. **Transaction abstraction ownership:** TDR-0003 defines an Application-owned logical transaction with Infrastructure participation, but the exact public abstraction between those members is not yet named. It must be specified before transaction-capable source code is implemented; it does not block creation of empty members.
3. **Cargo validation environment:** Cargo is currently unavailable on PATH. It must be installed before member manifests can be evaluated with `cargo metadata` and their dependency graph can be enforced.
4. **Authentication and authorization mechanism:** this remains required before the user-facing Tauri command becomes operational, but does not block member initialization.

---

# Codex Readiness

A senior Rust engineer can create the four empty member crates without inventing architecture once this plan is approved. The only implementation choices left for member initialization are the approval-gated edition/resolver values and exact dependency versions compatible with the approved TDRs.

No source code, Cargo member manifest, dependency, crate directory, or framework was created by this plan.

---

# References

* TDS-0001 — System Architecture
* TDS-0002 — Domain Model
* TDS-0004 — Application Model
* TDR-0001 — Programming Language
* TDR-0003 — Storage Strategy
* TDR-0004 — IPC Serialization Strategy
* TDR-0005 — Workspace Location Reconciliation
* `docs/architecture/architecture-enforcement-specification.md`
* `docs/architecture/workspace-specification.md`
* ISP-0001 through ISP-0010
* `docs/implementation/MILESTONE-001.2-CRATE-BOUNDARY-PLAN.md`
