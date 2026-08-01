# Milestone 001.1 — Rust Workspace Initialization

**Status:** Complete  
**Scope:** Workspace initialization only. No domain behavior, application behavior, persistence schema, migration, Tauri command, frontend, or member crate is created by this milestone.

---

# Workspace Structure Created

The canonical Rust implementation workspace is initialized beneath the Repository Workspace as required by TDR-0005 and ARCH-0004:

```text
implementation/
└── rust/
    ├── Cargo.toml
    ├── domains/
    ├── applications/
    ├── infrastructure/
    └── platform/
```

The four category directories contain only `.gitkeep` placeholders so that the approved physical structure is retained before member crates exist. No additional ARCH-0004 categories were created because they are not required for Milestone 1 workspace initialization.

---

# Cargo Configuration

`implementation/rust/Cargo.toml` is the canonical Cargo workspace root.

The workspace declares an empty `members` list. This is intentional:

* no crate names have been approved for the initial implementation;
* no Rust edition or resolver setting is selected before a member crate requires one;
* no dependencies are declared;
* no package, binary, library, build script, feature, or default member exists.

Member crates will be added only when the approved Create Organization implementation plan is authorized to create the corresponding ownership boundary.

---

# Dependency Rules Validation

No member crates currently exist, so the workspace contains no Cargo dependency edges. The initial layout reserves the required ownership categories; future manifests must satisfy this dependency direction:

```text
Platform → Application → Domain
Infrastructure → Domain / Application contracts
```

The following constraints are carried forward from the approved architecture:

| Area | Required rule |
|---|---|
| Domain | Must not depend on SQLx, Tauri, Serde IPC, Infrastructure, or Presentation code. |
| Application | May depend on Domain abstractions; must not depend on concrete persistence or desktop-runtime implementation. |
| Infrastructure | Implements Domain-owned contracts and may depend on approved Platform services; must not define business contracts. |
| Platform | Owns runtime bootstrap and the Tauri boundary; must not own Organization business behavior. |
| Workspace | No circular dependencies; every future crate has one architectural owner. |

The empty workspace is compliant by construction. Dependency-graph validation becomes executable when member manifests are introduced.

---

# Relationship to ARCH-0004

ARCH-0004 defines the Repository Workspace as the parent architecture and `implementation/rust/` as the location of the Cargo workspace. This milestone implements only that physical placement.

ARCH-0004 defines workspace categories rather than mandatory crate names. Accordingly, this milestone creates only `domains`, `applications`, `infrastructure`, and `platform`; it does not create Presentation, Shared, Plugins, Tooling, or other future categories. The workspace structure does not redefine the Component Model, bounded contexts, aggregate ownership, or the vertical-slice implementation pattern.

---

# Validation Performed

1. Confirmed that the workspace root is `implementation/rust/Cargo.toml`.
2. Confirmed that only the four required category directories were created.
3. Confirmed that the workspace has no members and therefore no dependency edges or undeclared technology dependencies.
4. Checked repository whitespace with `git diff --check`.
5. Attempted `cargo metadata --manifest-path implementation/rust/Cargo.toml --no-deps --format-version 1`. It could not run because Cargo is not installed or available on the current environment PATH; no Cargo manifest evaluation was performed.

---

# Unresolved Issues

1. **Initial member-crate names and boundaries:** ARCH-0004 intentionally does not prescribe crate names. They must be defined only when the next approved implementation step requires them.
2. **Rust edition and Cargo resolver:** no member crate currently establishes an edition. These settings must be selected consistently with the first concrete crate and recorded if they constitute a new technology decision.
3. **Authentication and authorization:** the architecture requires these before Application Service execution, but the Milestone 1 authorization mechanism is not yet defined. It must be resolved before enabling a user-facing Create Organization Tauri command.
4. **Cargo toolchain validation:** Cargo must be installed and `cargo metadata` rerun before adding the first member crate.

No deviation from the approved architecture was made.

---

# References

* TDS-0001 — System Architecture
* TDS-0002 — Domain Model
* TDS-0004 — Application Model
* TDR-0001 — Programming Language
* TDR-0005 — Workspace Location Reconciliation
* `docs/architecture/architecture-enforcement-specification.md`
* `docs/architecture/workspace-specification.md`
* ISP-0001 through ISP-0010
