# Implementation Milestone 1 — Create Organization Vertical Slice

**Status:** Planning  
**Scope:** Implementation plan only. This document authorizes no source-code, workspace, crate, technology-selection, or architecture changes.

---

# 1. Objective

Plan the first end-to-end ForgeOS capability: **Create Organization**.

The completed slice will accept a validated application request at the Tauri IPC boundary, invoke one coordinating application workflow, create the Organization aggregate through domain behavior, persist it through the domain-owned `OrganizationRepository` contract, publish the resulting domain event, and return a response DTO. It will demonstrate:

* an Organization domain aggregate;
* an application command and command handler;
* one coordinating application service;
* a domain-owned repository abstraction and replaceable infrastructure implementation;
* local persistence through an approved technology decision;
* a Tauri command/DTO boundary; and
* automated verification at each architectural boundary.

The slice does not introduce Mission, Process, Knowledge, Workforce, Governance, AI, plugin, or frontend-framework behavior.

---

# 2. Architecture Traceability

Every implementation item shall use the following authorities in the stated order. A lower authority may clarify implementation but may not redefine a higher authority.

| Planned decision | RFC authority | TDS authority | TDR authority | Architecture Package | ISP authority |
|---|---|---|---|---|---|
| Organization is the first capability and persistent organizational identity | RFC-0004 — Organization Model; RFC-0014 — ForgeOS Identity | TDS-0002 Organization context | TDR-0001 (Rust) | `domain-model.md`, `component-model.md` | ISP-0010 |
| Aggregate ownership, entities, value objects, repository contract, and domain events | RFC-0004 | TDS-0002 | TDR-0001 | `aggregate-boundaries.md`, `entity-relationships.md`, `domain-event-model.md`, `persistence-model.md` | ISP-0004, ISP-0005 |
| One command, handler, and coordinating application service | RFC-0004 | TDS-0004 Application Model | TDR-0001 | `application-model.md`, `application-services.md`, `command-query-model.md`, `workflow-orchestration.md`, `component-model.md` | ISP-0001, ISP-0002, ISP-0006, ISP-0008 |
| Dependency composition and layer isolation | RFC-0001 — ForgeOS Genome | TDS-0001, TDS-0002, TDS-0004 | TDR-0001 | `architecture-enforcement-specification.md`, `workspace-specification.md`, `system-context.md` | ISP-0007, ISP-0010 |
| Persistence adapter responsibility | RFC-0004 | TDS-0001, TDS-0002 | No persistence library decision is currently approved | `persistence-model.md`, `component-model.md` | ISP-0004, ISP-0006, ISP-0008 |
| Desktop IPC, DTOs, and Tauri command | RFC-0001 | TDS-0001, TDS-0004 | TDR-0002 (Tauri 2.x) | `system-context.md`, `integration-boundaries.md`, `component-model.md` | ISP-0001, ISP-0002, ISP-0008, ISP-0010 |
| Layered and end-to-end tests | RFC-0020 — Engineering Standards Framework | TDS-0001, TDS-0002, TDS-0004 | TDR-0001 | `architecture-enforcement-specification.md`, `system-context.md` | ISP-0009, ISP-0010 |

This table is a traceability map, not an extension of any specification. Where the plan identifies an unresolved item, implementation must stop at that boundary until the required authority is recorded.

---

# 3. Domain Implementation Plan

## Ownership and aggregate

Implement the `Organization` aggregate only in the Organization Domain. TDS-0002 defines it as the authoritative root of the Organization bounded context and the sole mutation boundary for organizational state. No other domain may modify it directly.

The aggregate owns organizational identity, profile, lifecycle, configuration, and hierarchy metadata. The implementation shall expose behavior rather than mutable internal state, enforce aggregate consistency, and remain independent of persistence, Tauri, and other infrastructure.

## Entities

The Organization aggregate contains the representative internal entities established by TDS-0002:

* `OrganizationProfile`
* `OrganizationHierarchy`
* `OrganizationCapability`
* `OrganizationClassification`

They have no independent lifecycle outside the Organization aggregate. Milestone 1 shall implement only the entities required by the approved Create Organization input and initialization semantics; it shall not add entity behavior or fields by assumption.

## Value objects

Use the approved Organization value-object vocabulary:

* `OrganizationId`
* `OrganizationName`
* `OrganizationStatus`
* `OrganizationType`
* `OrganizationVersion`
* `CapabilityIdentifier`

Value objects are immutable and compare by value. Their construction and validation rules must be derived from approved domain requirements; no field-level rules, defaults, or normalization policies are defined by this milestone plan.

## Domain invariants

The slice must preserve the following approved invariants:

1. Organization is the single aggregate root and architectural owner of organizational identity.
2. All organizational state mutations occur through the aggregate.
3. Internal entities are not externally mutable and have no independent lifecycle.
4. Value objects are immutable.
5. Business rules reside in the aggregate or an approved domain service, never in application, platform, presentation, or infrastructure code.
6. Cross-domain collaboration uses published contracts or domain events rather than direct state mutation.

The required validity rules for Organization creation—required inputs, allowed initial status, defaults, uniqueness semantics, and failure behavior—are not specified by the approved documents. They are unresolved decisions, not rules this plan may invent.

## Domain events

On successful aggregate creation, collect and publish the immutable `OrganizationCreated` event from the Organization bounded context. The event is a completed business fact; it must not contain workflow coordination or infrastructure behavior. Event dispatch occurs after the successful transaction according to ISP-0005 and ISP-0006. No consumer is in scope for this slice.

---

# 4. Application Implementation Plan

## Command

Define one **Create Organization Command** as an application-layer request representing the approved input contract. Its exact fields and structural validation rules are deferred until that contract is approved. The command contains no business behavior and does not expose domain entities.

## Command handler

Define one command handler as the application entry point for this command. It shall:

1. perform only application-level structural validation;
2. route the request to the Create Organization application service;
3. return an application result or a classified error.

It shall not make business decisions, mutate domain state directly, access a concrete persistence adapter, or own transaction behavior.

## Application service

Define one Create Organization application service as the sole coordinator for the use case. It receives domain-owned abstractions through constructor injection and shall coordinate the transaction, aggregate creation, repository persistence, and post-commit event dispatch. It contains no organization business rules or persistence implementation logic.

## Workflow sequence

```text
Create Organization DTO
  → Tauri command validates/authorizes the boundary request
  → Create Organization Command
  → Create Organization Command Handler
  → Create Organization Application Service (transaction boundary)
  → Organization aggregate behavior and invariant enforcement
  → OrganizationRepository domain contract
  → Infrastructure repository implementation
  → commit successful transaction
  → dispatch OrganizationCreated
  → map application result to response DTO
```

Failures are returned through the applicable domain, application, or infrastructure error boundary. The transaction rolls back on unsuccessful persistence or domain execution; infrastructure details must not escape into the domain.

---

# 5. Infrastructure Implementation Plan

## Repository interface usage

The Organization Domain owns `OrganizationRepository`. The application service may depend on and invoke only this domain-owned contract. The interface must support the TDS-0002 contract: create, retrieve, update, archive, existence verification, and optimistic concurrency. Milestone 1 uses the creation path and any contract operations necessary to enforce an approved creation invariant.

## Repository implementation location

Implement the concrete Organization repository in the Infrastructure implementation area defined by `workspace-specification.md`. It implements the Domain-owned interface, contains persistence mapping and technical error translation, and exposes no business-oriented API. It must remain replaceable without a Domain change.

## Persistence responsibility

Infrastructure owns storage interaction, mapping, transaction participation, and technical failure translation. The Organization aggregate remains persistence-agnostic, and the repository interface must not expose a persistence technology.

Phase 1 identifies SQLite storage as a product deliverable, but the available TDR package does not select a database access library, migration strategy, schema approach, or concrete transaction mechanism. This plan deliberately selects none. A concrete persistence adapter cannot be implemented until that technology decision is approved or located in existing approved authority.

---

# 6. Platform Boundary Plan

TDR-0002 establishes Tauri 2.x as the desktop host. The Create Organization Tauri command is the IPC boundary and shall:

* accept an immutable request DTO;
* perform boundary structural validation and authorization coordination;
* map the DTO to the application command;
* invoke the application service through the approved application boundary; and
* map the application result or classified error to an immutable response DTO.

Only DTOs cross IPC. Domain entities and domain value objects must never be serialized directly across the boundary. The platform owns Tauri runtime hosting and IPC routing; it must not acquire Organization business responsibility. No frontend framework, client state model, or IPC serialization technology is selected by this plan.

---

# 7. Testing Plan

## Domain tests

Run pure, deterministic tests without persistence or Tauri dependencies. Verify approved aggregate behavior, value-object immutability, aggregate ownership, emitted `OrganizationCreated` event, and each approved domain failure path. Add tests for field-level creation rules only after those rules are approved.

## Application tests

Use test doubles for the domain-owned repository and event-dispatch boundaries. Verify that the command handler delegates to the sole application service; the service coordinates the transaction, persists only through `OrganizationRepository`, dispatches events only after successful commit, and propagates classified failures without adding business logic.

## Infrastructure tests

After a concrete persistence decision is approved, verify the repository implementation against the `OrganizationRepository` contract: persistence mapping, retrieval of persisted aggregates, uniqueness/concurrency behavior if approved, transaction participation, and infrastructure-error translation. These tests must not encode business rules.

## End-to-end tests

After the Tauri composition root and persistence adapter are available, verify the published Tauri command accepts a request DTO, rejects malformed boundary input, executes the complete application path, persists a successful organization creation, returns only a response DTO, and does not serialize domain entities across IPC.

All tests shall be deterministic, behavior-focused, independently executable, and aligned with ISP-0009 and ISP-0010.

---

# 8. Dependency Validation

Before implementation is accepted, validate the slice against ARCH-0003 and the applicable ISP patterns:

| Validation | Required result |
|---|---|
| Organization Domain dependency contract | Depends only on approved shared/value-object/event contracts; no Infrastructure, Presentation, or other domain dependency. |
| Application Services dependency contract | Depends on domain interfaces/abstractions, not repository implementations, desktop runtime internals, or OS APIs. |
| Infrastructure dependency contract | Implements Domain-owned contracts and does not define business contracts or depend on Presentation. |
| Platform dependency contract | Hosts Tauri and composition only; does not own Organization behavior. |
| Ownership and repository validation | Organization has one owner; `OrganizationRepository` remains Domain-owned; concrete repository remains Infrastructure-owned. |
| IPC and DTO validation | DTOs alone traverse Tauri IPC; domain entities do not cross the boundary. |
| ISP pattern validation | The slice conforms to ISP-0001 (application service), ISP-0002 (command handler), ISP-0004 (repository), ISP-0005 (event), ISP-0006 (transaction), ISP-0007 (DI), ISP-0008 (errors), ISP-0009 (testing), and ISP-0010 (vertical slice). |

Validation shall use the compile-time, repository-time, and runtime mechanisms specified by ARCH-0003 once the approved workspace exists. No validation mechanism may bypass the layer boundaries it is intended to verify.

---

# 9. Codex Implementation Readiness

**Can a senior engineer implement this vertical slice without inventing architecture?**

**Not completely.** The architectural shape, ownership, language, desktop boundary, implementation patterns, and test responsibilities are approved. A senior engineer can prepare the workspace plan, domain-facing contracts, application orchestration plan, Tauri boundary plan, and dependency validation approach without inventing architecture.

The following decisions are required before a complete, persistence-backed Create Organization slice can be implemented:

1. **Create Organization domain contract:** the authoritative input fields, required creation invariants, initial lifecycle/status semantics, defaults, uniqueness policy, and result/error contract are not specified in the approved sources.
2. **Persistence technology detail:** SQLite is a Phase 1 product requirement, but no approved document selects its Rust access library, migration/schema strategy, or transaction mechanism.
3. **IPC serialization detail:** TDR-0002 intentionally defers IPC serialization. The DTO boundary is approved, but its serialization technology must be authorized before concrete transport implementation.

These are bounded decisions. Resolving them must follow the established RFC → TDS → TDR → Architecture → ISP authority order; they must not be made implicitly in source code or this plan.
