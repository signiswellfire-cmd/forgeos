# ForgeOS Architecture — Workspace Specification

**Document ID:** ARCH-0004

**Title:** Workspace Specification

**Status:** Approved

**Version:** 1.0.0

**Related Documents**

- TDS-0001 — System Architecture
- TDR-0001 — Programming Language
- TDR-0002 — Desktop Framework
- ARCH-0001 — System Context
- ARCH-0002 — Component Model
- ARCH-0003 — Architecture Enforcement Specification

---

# Purpose

This document specifies how the approved ForgeOS architecture is physically organized within the repository.

It defines:

- Repository Architecture
- Repository Structure
- Physical Repository Layout
- Implementation Workspace
- Cargo Workspace

This document is the authoritative bridge between the approved architecture and the implementation repository.

It does not redefine architectural ownership or implementation domains.

---

# Scope

This specification defines:

- logical repository organization;
- physical repository layout;
- implementation workspace organization;
- Cargo workspace organization;
- repository evolution rules;
- architectural ownership of repository areas.

Internal crate responsibilities are intentionally outside the scope of this document and are expected to emerge through implementation and subsequent Knowledge Promotion.

---

# Repository Architecture

## Definition

The ForgeOS Repository is the permanent architectural container for the project.

It contains:

- governance;
- architecture;
- implementation;
- engineering knowledge;
- tooling;
- tests;
- plugins;
- software artifacts.

The repository exists independently of any specific implementation technology.

---

## Architectural Responsibilities

The repository shall:

- preserve architectural knowledge;
- preserve engineering history;
- contain all implementation assets;
- support multiple implementation technologies;
- remain the authoritative source of truth.

Chat history shall never become part of the architectural baseline.

---

## Architectural Layers

The repository is organized into three architectural layers.

```text
Repository
    │
    ├── Architecture Layer
    │
    ├── Implementation Layer
    │
    └── Knowledge Layer
```

Each layer has distinct ownership.

Responsibilities shall not overlap.

---

# Repository Structure

The following top-level logical areas are defined.

| Area | Responsibility |
|------|----------------|
| Governance | Project governance and decision authority |
| Documentation | RFCs, TDSs, TDRs, architecture, standards |
| Knowledge | Promoted engineering knowledge |
| Implementation | Executable implementations |
| Tooling | Build, automation and engineering tools |
| Plugins | Extension ecosystem |
| SDK | Public developer assets |
| Tests | Cross-workspace verification |
| Examples | Reference implementations |

These are logical architectural areas.

Their physical directory layout is defined separately.

---

# Architectural Ownership

Each top-level repository area has one architectural owner.

| Repository Area | Architectural Owner |
|-----------------|--------------------|
| Governance | Governance Framework |
| Documentation | Documentation Architecture |
| Knowledge | Knowledge Promotion Process |
| Implementation | Implementation Architecture |
| Tooling | Engineering Infrastructure |
| Plugins | Plugin Architecture |
| SDK | SDK Architecture |
| Tests | Engineering Standards |
| Examples | Engineering Standards |

Ownership is architectural rather than organizational.

---

# Repository Boundaries

The repository owns:

- all authoritative documentation;
- all implementation assets;
- build configuration;
- engineering automation;
- promoted engineering knowledge.

The repository does not own:

- developer workstations;
- external services;
- deployment environments;
- temporary artifacts.

---

# Repository Architectural Invariants

The following constraints shall always remain true.

- The repository remains implementation-technology independent.
- Every repository artifact has exactly one architectural owner.
- Repository organization remains stable as implementations evolve.
- Documentation remains authoritative over implementation.
- Architectural artifacts shall not be generated dynamically.

Violation of these invariants requires architectural review.

---

# Cross References

This specification derives repository organization from:

- ARCH-0002 — Component Model
- ARCH-0003 — Architecture Enforcement Specification

It does not introduce new architectural boundaries.

*End of Part 1.*

# Physical Repository Layout

This section defines the normative physical organization of the ForgeOS repository.

The physical layout realizes the logical Repository Structure defined previously.

Directory names defined by this specification are authoritative.

---

# Top-Level Repository Layout

The ForgeOS repository shall conform to the following high-level structure.

```text
forgeos/
├── docs/
│   ├── architecture/
│   ├── governance/
│   ├── knowledge/
│   ├── rfcs/
│   ├── tds/
│   ├── tdrs/
│   └── standards/
│
├── implementation/
│
├── plugins/
│
├── sdk/
│
├── tooling/
│
├── tests/
│
├── examples/
│
├── scripts/
│
├── .github/
│
└── README.md
```

Additional top-level directories require architectural approval in accordance with the Repository Evolution Rules defined later in this document.

---

# Repository Area Responsibilities

## `/docs`

Purpose

The `/docs` directory contains every authoritative engineering artifact.

Contents include:

- governance;
- RFCs;
- TDSs;
- TDRs;
- architecture specifications;
- engineering standards;
- promoted engineering knowledge.

Executable implementation artifacts shall not reside within `/docs`.

---

## `/implementation`

Purpose

Contains executable implementations approved by the architecture.

This directory is implementation-neutral.

Examples may include:

```text
implementation/
    rust/
    python/
    web/
    mobile/
```

The existence of one implementation technology shall not constrain the addition of future implementation technologies.

---

## `/plugins`

Purpose

Contains first-party plugin implementations.

Third-party plugins are intentionally outside the repository scope.

Plugins remain implementation artifacts and are governed by the Plugin Architecture.

---

## `/sdk`

Purpose

Contains developer-facing SDKs and public integration assets.

SDKs shall remain independent of application-specific implementation details wherever practical.

---

## `/tooling`

Purpose

Contains engineering tooling including:

- architecture validation;
- build automation;
- code generation;
- developer utilities;
- repository maintenance.

Tooling supports implementation but does not own business functionality.

---

## `/tests`

Purpose

Contains repository-level testing assets.

Examples include:

- architectural conformance tests;
- integration test assets;
- performance test suites;
- interoperability tests.

Individual implementation workspaces may contain additional implementation-specific tests.

---

## `/examples`

Purpose

Contains reference implementations and sample projects.

Examples are educational artifacts and shall not become implementation dependencies.

---

## `/scripts`

Purpose

Contains repository automation scripts that are not part of the implementation runtime.

Examples include:

- bootstrap utilities;
- release automation;
- documentation generation;
- repository maintenance.

Scripts shall not contain production business logic.

---

## `/.github`

Purpose

Contains repository automation specific to GitHub.

Examples include:

- CI workflows;
- issue templates;
- pull request templates;
- repository configuration.

Repository automation shall remain separate from implementation code.

---

# Implementation Workspace

The Implementation Workspace is the architectural area that contains executable implementations.

It is independent of any specific programming language.

---

## Responsibilities

The Implementation Workspace owns:

- executable applications;
- reusable libraries;
- implementation tests;
- implementation configuration;
- implementation build assets.

It does not own:

- governance;
- architectural documentation;
- engineering standards;
- promoted knowledge.

---

## Architectural Invariants

The following constraints shall always remain true.

- Multiple implementation technologies may coexist.
- Each implementation technology remains isolated.
- Shared architectural ownership is prohibited.
- Cross-implementation dependencies require explicit architectural approval.
- The repository architecture remains stable as implementation technologies evolve.

---

## Implementation Workspace Categories

Implementation assets are organized into the following categories.

| Category | Purpose |
|----------|---------|
| Applications | Executable products |
| Libraries | Shared implementation components |
| Services | Long-running implementation services |
| Tooling | Implementation-specific engineering tools |
| Test Assets | Implementation-specific testing |

These categories are conceptual and may be represented differently by each implementation technology.

---

# Implementation Workspace Ownership

Every implementation artifact shall have one architectural owner.

Ownership is inherited from the corresponding Implementation Domain defined by ARCH-0002.

Physical location does not determine ownership.

Ownership remains architectural.

---

# Cross References

The Implementation Workspace derives its ownership model from:

- ARCH-0002 — Component Model

Its enforcement is defined by:

- ARCH-0003 — Architecture Enforcement Specification

This document defines only the physical organization of implementation assets.

*End of Part 2.*

# Cargo Workspace

The Cargo Workspace defines the Rust implementation contained within the Implementation Workspace.

It is a Rust-specific realization of the approved architecture and shall not redefine architectural ownership.

---

# Purpose

The Cargo Workspace exists to:

- organize Rust crates;
- enforce architectural boundaries;
- enable reproducible builds;
- support incremental compilation;
- simplify testing;
- provide a stable implementation topology.

Business architecture remains defined by ARCH-0002.

---

# Cargo Workspace Location

The Rust implementation shall reside beneath the Implementation Workspace.

Representative layout:

```text id="8l1pzf"
implementation/
└── rust/
    ├── Cargo.toml
    ├── Cargo.lock
    │
    ├── applications/
    ├── domains/
    ├── infrastructure/
    ├── platform/
    ├── presentation/
    ├── shared/
    ├── plugins/
    └── tooling/
```

This specification defines categories rather than mandatory crate names.

Crate names emerge from implementation and subsequent Knowledge Promotion.

---

# Workspace Categories

## Applications

Purpose

Executable binaries.

Examples include:

- desktop application;
- command-line tools;
- maintenance utilities.

Applications coordinate execution and shall not own business rules.

---

## Domains

Purpose

Business implementation domains.

Representative categories:

- organization;
- mission;
- process;
- knowledge;
- memory;
- workforce;
- governance.

Each crate has exactly one architectural owner.

---

## Infrastructure

Purpose

Concrete implementations of technical services.

Representative responsibilities:

- persistence;
- search;
- AI providers;
- import/export;
- storage;
- networking.

Infrastructure implements contracts owned elsewhere.

---

## Platform

Purpose

Runtime services independent of business behavior.

Representative responsibilities:

- bootstrap;
- configuration;
- diagnostics;
- dependency composition;
- runtime health.

---

## Presentation

Purpose

Desktop presentation layer.

Representative responsibilities:

- UI composition;
- view models;
- navigation;
- user interaction;
- IPC integration.

---

## Shared

Purpose

Implementation assets intentionally shared across implementation domains.

Examples may include:

- primitives;
- value objects;
- serialization helpers;
- common utilities.

Shared shall remain minimal.

Business ownership shall never migrate into Shared.

---

## Plugins

Purpose

First-party plugin implementations.

Plugins remain isolated implementation units.

---

## Tooling

Purpose

Rust-specific engineering tools.

Examples include:

- generators;
- migration tools;
- validation utilities;
- architecture verification tools.

Tooling shall not own production business functionality.

---

# Workspace Layering

The Cargo Workspace shall preserve the approved architectural layering.

```text id="1n0gwz"
Applications
      │
      ▼
Presentation
      │
      ▼
Application Services
      │
      ▼
Implementation Domains
      │
      ▼
Shared
      │
      ▼
Infrastructure
      │
      ▼
Platform
```

The dependency direction is governed by ARCH-0003.

---

# Workspace Ownership

Workspace organization follows architectural ownership.

| Workspace Category | Architectural Owner |
|--------------------|---------------------|
| Applications | Application Services |
| Presentation | Presentation Domain |
| Domains | Corresponding Implementation Domain |
| Infrastructure | Infrastructure Domain |
| Platform | Platform Domain |
| Shared | Shared Kernel Governance |
| Plugins | Plugin Architecture |
| Tooling | Engineering Infrastructure |

Physical location does not alter ownership.

---

# Feature Policy

Cargo features shall satisfy the following principles.

- Features are additive.
- Features shall not alter architectural ownership.
- Features shall not bypass dependency contracts.
- Optional providers shall be feature-gated where practical.
- Business behavior shall not depend upon feature combinations.

---

# Build Organization

The Cargo Workspace shall support:

- reproducible builds;
- deterministic dependency resolution;
- incremental compilation;
- workspace-wide testing;
- workspace-wide linting;
- documentation generation.

Workspace configuration shall remain centralized.

---

# Testing Organization

Testing responsibilities are organized by scope.

| Scope | Responsibility |
|--------|----------------|
| Unit | Individual crates |
| Integration | Cross-crate interactions |
| Architectural | Workspace conformance |
| End-to-End | Application behavior |
| Performance | Runtime characteristics |

Testing ownership follows architectural ownership.

---

# Workspace Architectural Invariants

The following constraints shall always remain true.

- Every crate has exactly one architectural owner.
- Workspace organization shall not redefine implementation domains.
- Shared remains intentionally small.
- Business rules remain outside Applications.
- Infrastructure implements but does not define contracts.
- Platform remains technology-oriented.
- Presentation remains business-rule free.

Changes to these invariants require architectural review.

---

# Cross References

The Cargo Workspace derives from:

- ARCH-0002 — Component Model
- ARCH-0003 — Architecture Enforcement Specification

This section defines physical Rust organization only.

*End of Part 3.*

# Repository Evolution Rules

The ForgeOS repository is intended to remain stable throughout the lifetime of the project.

Repository evolution shall preserve architectural integrity while allowing implementation technologies and engineering knowledge to evolve.

Repository evolution is governed rather than ad hoc.

---

# Evolution Principles

The following principles govern repository evolution.

1. Repository Architecture is stable.
2. Implementation technologies are replaceable.
3. Architectural ownership is explicit.
4. Repository organization evolves conservatively.
5. Knowledge accumulates; it is not replaced.
6. Documentation remains authoritative over implementation.
7. Implementation informs future Knowledge Promotion but does not redefine architecture without approval.

---

# Top-Level Directory Governance

Top-level repository directories are considered architectural assets.

Creation of a new top-level directory requires:

- a demonstrated architectural need;
- identification of the architectural owner;
- documentation of its responsibility;
- confirmation that no existing directory satisfies the requirement;
- formal architectural approval.

Top-level directories shall not be introduced for convenience alone.

---

# Repository Ownership Rules

Every repository artifact shall have exactly one architectural owner.

Ownership applies to, but is not limited to:

- directories;
- documents;
- implementation assets;
- generated artifacts;
- build configuration;
- automation scripts;
- examples;
- tests;
- plugins;
- SDKs.

Physical location shall not be used to infer ownership.

Architectural ownership is normative.

---

# Repository Growth Rules

Repository growth shall occur by extending existing architectural areas whenever practical.

Creation of parallel structures with overlapping responsibilities is prohibited.

Representative examples of prohibited growth include:

- duplicate documentation hierarchies;
- competing standards directories;
- multiple plugin roots;
- multiple implementation roots for the same technology;
- duplicated governance locations.

Growth should increase capability without increasing ambiguity.

---

# Repository Refactoring Rules

Repository refactoring shall preserve:

- architectural ownership;
- document authority;
- implementation traceability;
- repository history.

Refactoring shall not invalidate existing architectural references without coordinated updates.

---

# Repository Deprecation Rules

Repository artifacts may be deprecated only when:

- a successor has been approved;
- authoritative references have been updated;
- migration guidance exists;
- historical traceability is preserved.

Removal shall follow deprecation.

Deletion without deprecation is prohibited for authoritative architectural artifacts.

---

# Future Implementation Technologies

The Repository Architecture intentionally supports multiple implementation technologies.

Representative future implementations may include:

```text
implementation/
    rust/
    python/
    web/
    mobile/
    embedded/
```

Each implementation technology shall:

- remain isolated;
- define its own implementation workspace;
- conform to the same architectural ownership model;
- comply with ARCH-0003.

The addition of a new implementation technology shall not require restructuring the repository.

---

# Knowledge Promotion

Implementation produces engineering experience.

Validated engineering experience becomes organizational knowledge through the Knowledge Promotion process.

Examples include:

- validated crate responsibilities;
- public API design;
- testing strategies;
- implementation patterns;
- performance characteristics;
- operational guidance.

These artifacts belong in the Knowledge layer rather than the architectural bootstrap.

---

# Repository Lifecycle

The repository evolves through the following lifecycle.

```text
Architecture
      ↓
Implementation
      ↓
Validation
      ↓
Knowledge Promotion
      ↓
Standards Refinement
      ↓
Next Implementation
```

Architecture establishes intent.

Implementation validates intent.

Knowledge Promotion preserves validated engineering experience.

---

# Repository Integrity

The following conditions indicate repository integrity.

- Every architectural artifact has one authoritative owner.
- Every implementation artifact is traceable to an approved architectural owner.
- Every implementation technology resides within an Implementation Workspace.
- Every Cargo Workspace conforms to the Repository Architecture.
- Every promoted engineering artifact is traceable to validated implementation experience.

Repository integrity shall be continuously monitored using the enforcement mechanisms defined by ARCH-0003.

---

# Cross References

| Concern | Authoritative Document |
|----------|------------------------|
| System Architecture | TDS-0001 |
| Technology Decisions | TDR Series |
| Runtime Context | ARCH-0001 |
| Component Ownership | ARCH-0002 |
| Architecture Enforcement | ARCH-0003 |
| Workspace Organization | **ARCH-0004 (this document)** |

This document defines the physical realization of the approved architecture. It introduces no new implementation domains or technology decisions.

---

# Codex Readiness

## Implementation Status

**Design Package 1 is implementation-ready.**

A Senior Software Engineer can now:

- establish the repository;
- organize the implementation workspace;
- create the Cargo workspace;
- assign architectural ownership;
- implement runtime topology;
- enforce architectural contracts;
- begin the first vertical slice;

without inventing repository organization or architectural boundaries.

## Remaining Work Before Full Implementation

Future work proceeds through implementation rather than architectural prediction.

Implementation may identify opportunities for:

- Knowledge Promotion;
- engineering standards refinement;
- additional Technology Decision Records where genuinely required.

These activities refine implementation knowledge without altering the approved architectural foundation.

---

# Architectural Stability

The Repository Architecture is considered stable for the ForgeOS MVP.

Future changes shall preserve:

- repository architecture;
- implementation domain ownership;
- architectural enforcement;
- repository evolution governance.

Changes to these principles require formal architectural review.

---

# Document Completion

This document is complete.

Together with:

- TDS-0001 — System Architecture
- TDR-0001 — Programming Language
- TDR-0002 — Desktop Framework
- ARCH-0001 — System Context
- ARCH-0002 — Component Model
- ARCH-0003 — Architecture Enforcement Specification

it completes **Design Package 1** and establishes the implementation-ready architectural baseline for ForgeOS MVP.