# Milestone 001 — Implementation Baseline

**Status:** Approved  
**Version:** 1.0.0  
**Purpose:** Record the transition from the documentation/bootstrap phase into the implementation phase of ForgeOS. This document creates no source code, Cargo member, dependency, or architecture change.

---

# 1. Purpose

This record establishes the canonical implementation baseline that the Create Organization capability will be built against. It consolidates the completed architectural preparation, the approved workspace and crate boundaries, the approved domain decisions, and the first implementation target. It also records the toolchain limitation that currently blocks Cargo-level validation.

---

# 2. Completed Architecture Preparation

The bootstrap phase is complete. The repository contains the approved engineering baseline:

| Package | Contents | Status |
|---|---|---|
| Philosophy | Vision, Mission, Philosophy, Constitution, Core Values, Engineering Principles | Complete |
| Genome | ForgeOS Genome | Complete |
| RFC Foundation | RFC-0001 through RFC-0045 | Approved |
| Engineering Standards | Architecture, Coding, Documentation, Git, Naming, Testing | Complete |
| Design Package 1 | TDS-0001 (System Architecture), ARCH-0001 through ARCH-0004 | Approved |
| Design Package 2 | TDS-0002 (Domain Model) and derived architecture views | Approved |
| Design Package 3 | TDS-0003 (Organization Model) and derived architecture views | Approved |
| Design Package 4 | TDS-0004 (Application Model) and derived architecture views | Approved |
| Implementation Specification Package | ISP-0001 through ISP-0010 | Approved |

---

# 3. Completed Implementation Specifications

The following implementation specifications are approved and in force for all implementation work:

* ISP-0001 — Application Service Pattern
* ISP-0002 — Command Handler Pattern
* ISP-0003 — Query Handler Pattern
* ISP-0004 — Repository Pattern
* ISP-0005 — Domain Event Pattern
* ISP-0006 — Transaction Pattern
* ISP-0007 — Dependency Injection Pattern
* ISP-0008 — Error Handling Pattern
* ISP-0009 — Testing Pattern
* ISP-0010 — Vertical Slice Pattern

---

# 4. Approved Workspace and Crate Boundaries

## Workspace

The canonical Rust Cargo workspace resides at:

```text
implementation/rust/
```

with resolver `3` and Rust edition 2024 across all members. This placement is governed by TDR-0005 and ARCH-0004.

## Approved members

| Member path | Cargo package | Rust crate | Ownership |
|---|---|---|---|
| `domains/organization-domain` | `forgeos-organization-domain` | `forgeos_organization_domain` | Organization Domain |
| `applications/create-organization` | `forgeos-create-organization-application` | `forgeos_create_organization_application` | Application Services |
| `infrastructure/organization` | `forgeos-organization-infrastructure` | `forgeos_organization_infrastructure` | Infrastructure Domain |
| `platform/desktop` | `forgeos-desktop-platform` | `forgeos_desktop_platform` | Platform Domain |

Crate boundaries are defined by the approved MILESTONE-001.2 (Crate Boundary Plan) and realized by the approved MILESTONE-001.3 (Crate Initialization Plan) and MILESTONE-001.4 (Cargo Member Initialization).

## Dependency contract (ARCH-0003)

```text
Platform → Application → Domain
Infrastructure → Domain interfaces
```

No Domain dependency on Application, Infrastructure, Platform, Tauri, SQLx, or Serde IPC is permitted.

---

# 5. Approved Technology Decisions

| TDR | Decision |
|---|---|
| TDR-0001 | Programming Language — Rust/Cargo |
| TDR-0002 | Desktop Framework — Tauri 2.x |
| TDR-0003 | Storage Strategy — SQLite via SQLx (Infrastructure-only) |
| TDR-0004 | IPC Serialization Strategy — Serde/JSON DTOs over Tauri command IPC |
| TDR-0005 | Workspace Location Reconciliation — `implementation/rust/` |
| TDR-0006 | OrganizationId Generation — UUID v4 through a Domain-owned generator contract |

---

# 6. Approved Domain Decisions

| Decision | Resolution |
|---|---|
| MILESTONE-001-DOMAIN-DECISIONS | Create Organization domain contract: caller supplies `name` and `organization_type`; aggregate generates identity; initial status `Active`; initial version `1`; singleton rule; validation rules; result and error contracts |
| OrganizationType Decision | `OrganizationType` is a String-backed immutable value object; preserved as supplied; no enumeration; non-whitespace requirement; taxonomy deferred |
| TDR-0006 | `OrganizationId` generation via UUID v4; `OrganizationIdGenerator` contract owned by the Organization Domain; `uuid` crate (`v4` feature) is the approved single dependency exception for the Domain crate |

---

# 7. First Implementation Target

**Implementation Milestone 001.5 — Organization Domain Foundation**

Scope: implement the Organization Domain Foundation inside the `forgeos-organization-domain` crate:

* Organization aggregate with approved creation semantics;
* approved value objects (`OrganizationId`, `OrganizationName`, `OrganizationType`, `OrganizationStatus`, `OrganizationVersion`);
* internal entities required by the approved creation semantics;
* Domain-owned `OrganizationRepository` contract (create, retrieve, update, archive, existence verification, optimistic concurrency);
* `OrganizationCreated` domain event;
* approved domain error model (Validation, `OrganizationAlreadyExists`, Unexpected);
* deterministic domain tests per ISP-0009 and ISP-0010.

Authority: `docs/implementation/MILESTONE-001.5-ORGANIZATION-DOMAIN.md`.

Out of scope for this milestone: persistence/infrastructure, application/transaction layer, Tauri/IPC boundary, authentication/authorization, frontend, non-Organization domains, Shared crate, OrganizationType taxonomy.

---

# 8. Validation Limitations

The following validation limitation is recorded as of this baseline:

* **Cargo is not installed** on the current environment PATH.
* `cargo metadata`, `cargo check --workspace`, and Cargo-enforced dependency-graph validation (ARCH-0003 Layer 1) cannot yet run.
* The Rust toolchain must be installed and `cargo metadata` must succeed before the workspace can be compiled and its dependency graph verified.
* Until then, manifest conformance is validated by static review only, as recorded by MILESTONE-001.1 and MILESTONE-001.4.

---

# 9. Transition Statement

With this baseline, ForgeOS transitions from the documentation/bootstrap phase into the implementation phase. Implementation shall:

* follow the authority order RFC → TDS → TDR → Architecture Package → ISP → source code;
* introduce no undocumented technology or architectural decision;
* stop at any boundary whose required authority has not been recorded;
* preserve Git as permanent engineering memory.

No source code is created by this document.

---

# 10. References

* RFC-0004 — Organization Model
* TDS-0001 — System Architecture
* TDS-0002 — Domain Model
* TDS-0003 — Organization Model
* TDS-0004 — Application Model
* TDR-0001 through TDR-0006
* ARCH-0003 — Architecture Enforcement Specification
* ARCH-0004 — Workspace Specification
* ISP-0001 through ISP-0010
* `docs/implementation/MILESTONE-001-CREATE-ORGANIZATION.md`
* `docs/implementation/MILESTONE-001-DOMAIN-DECISIONS.md`
* `docs/implementation/MILESTONE-001.1-WORKSPACE-INITIALIZATION.md`
* `docs/implementation/MILESTONE-001.2-CRATE-BOUNDARY-PLAN.md`
* `docs/implementation/MILESTONE-001.3-CRATE-INITIALIZATION-PLAN.md`
* `docs/implementation/MILESTONE-001.4-CARGO-MEMBER-INITIALIZATION.md`
* `docs/implementation/MILESTONE-001.5-ORGANIZATION-DOMAIN.md`
* `docs/implementation/ORGANIZATION-TYPE-DECISION.md`