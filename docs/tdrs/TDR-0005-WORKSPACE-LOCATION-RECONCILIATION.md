# TDR-0005 — Workspace Location Reconciliation

**TDR Number:** TDR-0005  
**Title:** Workspace Location Reconciliation  
**Status:** Approved  
**Version:** 1.0.0  
**Related TDS:** TDS-0001 — System Architecture; TDS-0002 — Domain Model; TDS-0004 — Application Model  
**Related TDR:** TDR-0001 — Programming Language

---

# Context

ForgeOS is preparing its first Rust workspace for Implementation Milestone 1. Before the workspace is created, repository documentation must identify one canonical physical location for its Cargo workspace.

The approved documentation distinguishes the repository's implementation workspace from a Rust-specific Cargo workspace inside it. A previous implementation request described a top-level `forgeos-core/` directory, while the Architecture Package defines a Rust workspace beneath `implementation/rust/`.

This record reconciles those descriptions before any directory, Cargo manifest, crate, or source code is created.

---

# Conflicting References

## TDR-0001 — Programming Language

TDR-0001 requires Rust and a Cargo workspace. Its Repository Organization section lists representative top-level crate paths, including:

```text
forgeos-core/
forgeos-domain/
forgeos-application/
forgeos-infrastructure/
forgeos-platform/
```

The wording is representative and permits additional crates subject to architectural dependency rules. It does not establish a mandatory physical repository location or a required crate-directory topology.

## ARCH-0004 — Workspace Specification

`docs/architecture/workspace-specification.md` states that the Rust implementation **shall reside beneath the Implementation Workspace** and provides the physical layout:

```text
implementation/
└── rust/
    ├── Cargo.toml
    ├── applications/
    ├── domains/
    ├── infrastructure/
    ├── platform/
    ├── presentation/
    ├── shared/
    ├── plugins/
    └── tooling/
```

The Workspace Specification also states that it defines categories rather than mandatory crate names.

---

# Authority Analysis

The source-of-truth hierarchy places TDRs above Architecture documents for technology choices, but the two documents govern different concerns:

* **TDR-0001** selects Rust and Cargo. Its crate list is illustrative technology guidance.
* **ARCH-0004** owns the physical Repository Workspace and Cargo Workspace location. It is the specific authority for repository topology.

There is therefore no conflict in architectural intent. The apparent conflict arises only when TDR-0001's representative names are interpreted as mandatory root-level paths. That interpretation would contradict ARCH-0004's explicit physical-location rule and introduce an undocumented repository restructuring.

---

# Decision

**Option B is the canonical Rust workspace location:**

```text
implementation/rust/
```

The Repository Workspace remains the parent implementation architecture. The Cargo workspace is one Rust implementation workspace within it. Its root manifest shall be located at:

```text
implementation/rust/Cargo.toml
```

`forgeos-core/` is not a canonical top-level workspace location. The representative names in TDR-0001 may inform future crate naming only when consistent with ARCH-0004's categories and approved implementation needs.

This decision does not create a workspace, require all ARCH-0004 categories to exist initially, alter crate ownership, or select a crate structure for Milestone 1.

---

# Repository Impact

After approval, all Rust implementation artifacts for ForgeOS shall be located beneath `implementation/rust/`. The initial Milestone 1 workspace may create only the approved minimum categories required for Domain, Application, Infrastructure, and Platform ownership.

No top-level `forgeos-core/`, `forgeos-domain/`, `forgeos-application/`, `forgeos-infrastructure/`, or `forgeos-platform/` directory shall be created as a competing workspace root.

Documentation or plans that use `forgeos-core` as a representative path shall be read as illustrative unless and until a later approved workspace decision makes a specific name canonical.

---

# Migration and Cleanup Requirement

No migration or cleanup is currently required because no Rust workspace, Cargo manifest, crate directory, or source code has been created.

If implementation artifacts are created at a root-level `forgeos-core/` location before this record is approved, they must not be copied into a second workspace. A follow-up approved migration plan must first establish the exact targets, preserve Git history, update references, and remove the obsolete location only after verification.

---

# Consequences

Positive consequences:

* the physical repository layout follows the approved Workspace Specification;
* future implementation technologies can coexist under the Repository Workspace;
* Rust remains a contained implementation workspace rather than becoming the repository architecture;
* no architecture is inferred from illustrative TDR crate names.

Trade-off:

* implementation instructions using `forgeos-core/` must be reconciled to the approved `implementation/rust/` location before execution.

---

# References

* TDS-0001 — System Architecture
* TDS-0002 — Domain Model
* TDS-0004 — Application Model
* TDR-0001 — Programming Language
* `docs/architecture/workspace-specification.md`
* `docs/architecture/architecture-enforcement-specification.md`
* `docs/implementation/ISP-0010-vertical-slice-pattern.md`
