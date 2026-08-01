# ForgeOS Architecture — Component Model

**Document ID:** ARCH-0002

**Title:** Component Model

**Status:** Approved

**Version:** 1.0.0

**Related Documents**

- TDS-0001 — System Architecture
- TDR-0001 — Programming Language
- TDR-0002 — Desktop Framework
- ARCH-0001 — System Context

---

# Purpose

This document defines the implementation decomposition of ForgeOS Core.

Unlike the System Context, which defines runtime boundaries, this document specifies the internal implementation domains that compose ForgeOS Core.

Each Implementation Domain represents a stable architectural boundary that can be translated directly into one or more Rust crates.

This document is the authoritative source for architectural ownership.

---

# Scope

This specification defines:

- implementation domains;
- architectural ownership;
- public interfaces;
- internal components;
- persistence ownership;
- domain events;
- extension points;
- architectural invariants.

Technology choices remain defined by the applicable Technology Decision Records.

---

# Architectural Ownership

ForgeOS is decomposed into **Implementation Domains**.

An Implementation Domain is the smallest architectural unit that owns a cohesive area of business capability.

The following ownership rules apply:

- Every crate has exactly one architectural owner.
- An Implementation Domain may own one or more crates.
- A crate shall not have multiple architectural owners.
- Cross-domain interaction occurs only through published interfaces or domain events.
- Architectural ownership shall remain stable even if physical crate organization changes.

---

# Implementation Domains

The MVP defines the following Implementation Domains:

1. Core Runtime
2. Organization Domain
3. Mission Domain
4. Process Domain
5. Knowledge Domain
6. Memory Domain
7. Workforce Domain
8. Governance Domain
9. Application Services
10. Infrastructure
11. Platform
12. Presentation

Subsequent sections define each domain.

---

# Implementation Domain — Core Runtime

## Purpose

The Core Runtime coordinates application startup, lifecycle management, dependency composition, and runtime orchestration.

It provides the execution environment for all other implementation domains without owning business behavior.

---

## Public Interfaces

The Core Runtime exposes:

- Runtime Bootstrap
- Service Registry
- Application Lifecycle
- Shutdown Coordination
- Health Status

No business-oriented APIs are exposed.

---

## Internal Components

Representative internal components include:

- Bootstrap Coordinator
- Dependency Composition Root
- Runtime Initializer
- Lifecycle Manager
- Service Locator (composition only)
- Configuration Loader
- Startup Validator

These components remain internal to the domain.

---

## Owned Data

The Core Runtime owns:

- runtime configuration;
- startup metadata;
- runtime state;
- initialization status.

Persistent organizational data is explicitly excluded.

---

## Published Events

Representative events include:

- RuntimeStarting
- RuntimeStarted
- RuntimeStopping
- RuntimeStopped
- ConfigurationLoaded

These events communicate runtime lifecycle only.

---

## Consumed Events

The Core Runtime may consume:

- ProcessTerminationRequested
- ConfigurationChanged
- PluginLoaded
- PluginUnloaded

Business events are not consumed directly.

---

## Persistence Responsibilities

The Core Runtime persists only technical runtime metadata where necessary.

It shall never own business entities or organizational state.

---

## Allowed Dependencies

The Core Runtime may depend on:

- Platform
- Infrastructure bootstrap services
- Configuration services

It shall not depend upon business domains.

---

## Forbidden Dependencies

The Core Runtime shall not depend on:

- Organization Domain
- Mission Domain
- Knowledge Domain
- Workforce Domain
- Governance Domain

Business capability ownership remains outside this domain.

---

## Extension Points

Supported extension points include:

- runtime initialization hooks;
- startup validation;
- plugin registration;
- lifecycle observers.

Extensions shall not alter runtime ownership.

---

## Architectural Invariants

The following rules shall always remain true.

- The Core Runtime owns no business logic.
- The Core Runtime owns no domain aggregates.
- Runtime initialization is deterministic.
- Shutdown occurs in reverse initialization order.
- Startup failures shall not leave partially initialized services active.
- Dependency composition occurs only during application startup.

These invariants require formal architectural review before modification.

---

# Implementation Domain — Organization Domain

## Purpose

The Organization Domain owns the organizational structure and identity of ForgeOS.

It provides the authoritative model for Organizations, organizational identity, organizational hierarchy, organizational capabilities, and organizational health.

Every business operation requiring organizational context depends on this domain.

The Organization Domain does not coordinate execution; it defines the organizational environment within which execution occurs.

*End of Part 1.*

# Implementation Domain — Organization Domain (continued)

## Public Interfaces

The Organization Domain exposes services for:

- Organization management
- Organization retrieval
- Organization hierarchy
- Organization DNA access
- Organization capability discovery
- Organization health retrieval

All operations return domain models or domain-defined DTOs through the Application Layer.

Direct repository access from other domains is prohibited.

---

## Internal Components

Representative internal components include:

- Organization Aggregate
- Organization Factory
- Organization Repository Interface
- Organization Domain Service
- Organization Policy
- Organization Health Calculator
- Organization DNA Manager
- Organizational Hierarchy Service

These components remain internal implementation details.

---

## Owned Data

The Organization Domain is the sole owner of:

- Organizations
- Organization DNA
- Organization Profiles
- Organizational Hierarchy
- Organizational Capabilities
- Organization Health Records
- Organizational Metadata

No other Implementation Domain may modify these entities directly.

---

## Published Events

Representative events include:

- OrganizationCreated
- OrganizationUpdated
- OrganizationArchived
- OrganizationHealthChanged
- CapabilityRegistered
- CapabilityRemoved
- OrganizationDNAModified

These events notify dependent domains without transferring ownership.

---

## Consumed Events

Representative consumed events include:

- MissionCompleted
- KnowledgePromoted
- DecisionApproved
- WorkforceCapabilityChanged

The Organization Domain consumes these events to maintain organizational state and metrics.

---

## Persistence Responsibilities

The Organization Domain owns persistence for:

- organizational aggregates;
- organization metadata;
- hierarchy relationships;
- capability registrations;
- organizational health snapshots.

Repository implementations belong to Infrastructure.

Repository interfaces belong to this domain.

---

## Allowed Dependencies

The Organization Domain may depend upon:

- shared kernel abstractions;
- common value objects;
- domain event contracts.

---

## Forbidden Dependencies

The Organization Domain shall not depend upon:

- Mission Domain
- Knowledge Domain
- Workforce Domain
- Infrastructure
- Presentation
- Platform

Relationships are expressed through events and identifiers.

---

## Extension Points

Supported extension points include:

- organization validators;
- capability providers;
- organization import/export adapters;
- health metric contributors.

---

## Architectural Invariants

The following constraints shall always remain true.

- Organization identity is immutable after creation except through governed lifecycle operations.
- Organization DNA is owned exclusively by this domain.
- Organizational hierarchy is authoritative.
- Capability ownership is centralized within this domain.
- External domains shall never modify organization aggregates directly.

These invariants define the constitutional boundary of the Organization Domain.

---

# Implementation Domain — Mission Domain

## Purpose

The Mission Domain owns the lifecycle of organizational missions.

It defines:

- mission planning;
- mission execution;
- mission completion;
- mission outcomes;
- mission ownership.

Mission execution coordinates work but does not implement business rules owned by other domains.

---

## Public Interfaces

The Mission Domain exposes:

- Mission creation
- Mission planning
- Mission assignment
- Mission execution
- Mission completion
- Mission retrieval

Mission state transitions are governed exclusively by this domain.

---

## Internal Components

Representative internal components include:

- Mission Aggregate
- Mission Repository Interface
- Mission Planner
- Mission Scheduler
- Mission Lifecycle Manager
- Mission Assignment Policy
- Mission State Machine
- Mission Validator

---

## Owned Data

The Mission Domain owns:

- Missions
- Mission Plans
- Mission Assignments
- Mission Status
- Mission History
- Mission Outcomes
- Mission Dependencies

---

## Published Events

Representative events include:

- MissionCreated
- MissionPlanned
- MissionAssigned
- MissionStarted
- MissionPaused
- MissionCompleted
- MissionCancelled
- MissionOutcomeRecorded

---

## Consumed Events

Representative events include:

- OrganizationUpdated
- CapabilityRegistered
- ProfessionalAssigned
- ProcessCompleted

Mission execution reacts to organizational changes but does not own them.

---

## Persistence Responsibilities

The Mission Domain owns persistence of:

- mission aggregates;
- mission execution history;
- assignment records;
- mission outcome records.

Only Mission repositories persist mission state.

---

## Allowed Dependencies

The Mission Domain may depend upon:

- shared kernel abstractions;
- common value objects;
- event contracts.

Mission coordination with other domains occurs through identifiers and domain events.

---

## Forbidden Dependencies

The Mission Domain shall not depend upon:

- Knowledge Domain
- Governance Domain
- Presentation
- Infrastructure

Cross-domain orchestration belongs to the Application Layer.

---

## Extension Points

Mission extensions may contribute:

- scheduling strategies;
- assignment policies;
- execution observers;
- mission templates.

These extensions shall preserve mission ownership.

---

## Architectural Invariants

The following constraints shall always remain true.

- Mission state transitions are controlled exclusively by the Mission Domain.
- Mission execution never modifies foreign aggregates directly.
- Mission ownership remains singular.
- Mission history is append-only.
- Mission completion is irreversible except through governed corrective processes.

Mission integrity shall remain independent of implementation technology.

*End of Part 2.*

# Implementation Domain — Process Domain

## Purpose

The Process Domain owns the definition, execution, and monitoring of organizational processes.

Processes describe *how* work is executed. Missions determine *what* work is performed. This distinction shall remain permanent.

The Process Domain provides reusable execution workflows that may be referenced by multiple Missions.

---

## Public Interfaces

The Process Domain exposes:

- Process definition management
- Process version management
- Process execution
- Process suspension and resumption
- Process completion
- Process status queries

Only this domain may alter process state.

---

## Internal Components

Representative internal components include:

- Process Aggregate
- Process Repository Interface
- Process Definition Manager
- Process Execution Engine
- Process State Machine
- Process Validator
- Process Version Manager

---

## Owned Data

The Process Domain owns:

- Process Definitions
- Process Versions
- Process Instances
- Process Execution State
- Process History
- Process Metrics

---

## Published Events

Representative events include:

- ProcessDefined
- ProcessVersionPublished
- ProcessStarted
- ProcessSuspended
- ProcessResumed
- ProcessCompleted
- ProcessFailed

---

## Consumed Events

Representative events include:

- MissionStarted
- MissionCancelled
- OrganizationUpdated

The Process Domain reacts to organizational execution but does not own mission lifecycle.

---

## Persistence Responsibilities

The Process Domain exclusively owns persistence of:

- process definitions;
- execution state;
- execution history;
- process metrics.

---

## Allowed Dependencies

The Process Domain may depend upon:

- shared kernel abstractions;
- common value objects;
- event contracts.

---

## Forbidden Dependencies

The Process Domain shall not depend upon:

- Knowledge Domain
- Workforce Domain
- Infrastructure
- Presentation

---

## Extension Points

Supported extensions include:

- execution strategies;
- validation rules;
- workflow templates;
- process observers.

---

## Architectural Invariants

The following constraints shall always remain true.

- Process definitions are versioned.
- Running process instances preserve version integrity.
- Processes never directly modify foreign aggregates.
- Process execution is deterministic for identical inputs unless explicitly configured otherwise.
- Mission ownership remains outside the Process Domain.

---

# Implementation Domain — Knowledge Domain

## Purpose

The Knowledge Domain owns organizational knowledge.

It provides the authoritative implementation for:

- Knowledge Objects;
- Knowledge Graph;
- Knowledge Relationships;
- Knowledge Lifecycle;
- Blueprint ownership.

All knowledge promotion, classification, and retrieval originate from this domain.

---

## Public Interfaces

The Knowledge Domain exposes:

- Knowledge creation
- Knowledge retrieval
- Knowledge classification
- Knowledge promotion
- Knowledge archival
- Knowledge relationship management

---

## Internal Components

Representative internal components include:

- Knowledge Aggregate
- Knowledge Repository Interface
- Knowledge Graph Manager
- Knowledge Classifier
- Knowledge Lifecycle Manager
- Blueprint Manager
- Knowledge Validator

---

## Owned Data

The Knowledge Domain owns:

- Knowledge Objects
- Knowledge Relationships
- Blueprint Metadata
- Knowledge Categories
- Knowledge Lifecycle State
- Knowledge Tags
- Knowledge Provenance

---

## Published Events

Representative events include:

- KnowledgeCreated
- KnowledgeUpdated
- KnowledgePromoted
- KnowledgeArchived
- BlueprintPublished
- KnowledgeRelationshipCreated

---

## Consumed Events

Representative events include:

- DecisionApproved
- MissionCompleted
- ProcessCompleted
- LearningCompleted

These events provide new organizational knowledge candidates.

---

## Persistence Responsibilities

The Knowledge Domain exclusively owns persistence for:

- knowledge aggregates;
- graph metadata;
- blueprint metadata;
- lifecycle state;
- relationship records.

---

## Allowed Dependencies

The Knowledge Domain may depend upon:

- shared kernel abstractions;
- event contracts;
- common value objects.

---

## Forbidden Dependencies

The Knowledge Domain shall not depend upon:

- Mission Domain
- Workforce Domain
- Infrastructure
- Presentation

---

## Extension Points

Supported extensions include:

- classifiers;
- indexing strategies;
- relationship analyzers;
- blueprint validators.

---

## Architectural Invariants

The following constraints shall always remain true.

- Knowledge ownership is singular.
- Knowledge provenance is immutable.
- Blueprint publication preserves historical lineage.
- Knowledge relationships remain explicitly typed.
- Knowledge promotion never destroys historical versions.

---

# Implementation Domain — Memory Domain

## Purpose

The Memory Domain preserves institutional memory.

It owns:

- Executive Memory;
- Engineering Memory;
- Organization Memory;
- historical organizational context.

Memory preserves organizational reasoning rather than operational execution.

---

## Public Interfaces

The Memory Domain exposes:

- Memory recording
- Memory retrieval
- Timeline reconstruction
- Historical comparison
- Memory classification

---

## Internal Components

Representative internal components include:

- Memory Aggregate
- Memory Repository Interface
- Timeline Builder
- Historical Context Service
- Memory Classifier
- Memory Validator

---

## Owned Data

The Memory Domain owns:

- Executive Memories
- Engineering Memories
- Organization Memories
- Historical Timelines
- Memory References
- Memory Metadata

---

## Published Events

Representative events include:

- MemoryRecorded
- MemoryUpdated
- MemoryInstitutionalized
- TimelineRebuilt

---

## Consumed Events

Representative events include:

- DecisionApproved
- MissionCompleted
- KnowledgePromoted
- OrganizationEvolved

The Memory Domain records organizational history without assuming ownership of those events.

---

## Persistence Responsibilities

The Memory Domain exclusively owns persistence for:

- institutional memories;
- historical timelines;
- contextual references;
- memory metadata.

---

## Allowed Dependencies

The Memory Domain may depend upon:

- shared kernel abstractions;
- common value objects;
- event contracts.

---

## Forbidden Dependencies

The Memory Domain shall not depend upon:

- Mission Domain
- Governance Domain
- Infrastructure
- Presentation

---

## Extension Points

Supported extensions include:

- timeline reconstruction algorithms;
- memory classification strategies;
- archival policies;
- historical analytics.

---

## Architectural Invariants

The following constraints shall always remain true.

- Institutional memory is append-only.
- Historical provenance is immutable.
- Memory entries remain traceable to authoritative organizational artifacts.
- Memory ownership is exclusive to this domain.
- Historical reconstruction never alters recorded history.

*End of Part 3.*

# Implementation Domain — Workforce Domain

## Purpose

The Workforce Domain owns the organizational workforce model.

It defines and governs:

- Professionals;
- Teams;
- Skills;
- Competencies;
- Capability assignments;
- Workforce relationships.

The Workforce Domain represents organizational capacity rather than organizational execution.

---

## Public Interfaces

The Workforce Domain exposes:

- Professional management
- Team management
- Skill management
- Competency management
- Capability assignment
- Workforce queries

Only this domain may modify workforce state.

---

## Internal Components

Representative internal components include:

- Professional Aggregate
- Team Aggregate
- Skill Aggregate
- Competency Aggregate
- Capability Assignment Manager
- Workforce Repository Interface
- Workforce Policy
- Competency Evaluator

---

## Owned Data

The Workforce Domain owns:

- Professionals
- Teams
- Skills
- Competencies
- Capability Assignments
- Workforce Metadata
- Team Memberships

---

## Published Events

Representative events include:

- ProfessionalCreated
- ProfessionalUpdated
- TeamCreated
- TeamMembershipChanged
- SkillRegistered
- CompetencyEvaluated
- CapabilityAssigned

---

## Consumed Events

Representative events include:

- MissionAssigned
- LearningCompleted
- OrganizationUpdated

The Workforce Domain updates workforce capability without assuming ownership of Missions or Organizations.

---

## Persistence Responsibilities

The Workforce Domain exclusively owns persistence of:

- workforce aggregates;
- competency records;
- capability assignments;
- team structures.

---

## Allowed Dependencies

The Workforce Domain may depend upon:

- shared kernel abstractions;
- common value objects;
- domain event contracts.

---

## Forbidden Dependencies

The Workforce Domain shall not depend upon:

- Mission Domain
- Knowledge Domain
- Infrastructure
- Presentation

---

## Extension Points

Supported extensions include:

- competency assessment algorithms;
- capability recommendation strategies;
- workforce analytics providers;
- organizational role providers.

---

## Architectural Invariants

The following constraints shall always remain true.

- Professional identity is owned exclusively by this domain.
- Team membership is governed only by this domain.
- Competency history is append-only.
- Capability ownership remains explicit.
- Workforce state shall never be modified directly by foreign domains.

---

# Implementation Domain — Governance Domain

## Purpose

The Governance Domain owns organizational authority.

It governs:

- Decisions;
- Policies;
- Standards;
- Delegated Authority;
- Executive Oversight;
- Governance workflows.

The Governance Domain defines *who may decide* rather than *how work is executed*.

---

## Public Interfaces

The Governance Domain exposes:

- Decision management
- Policy management
- Authority management
- Standard management
- Governance evaluation
- Approval workflows

---

## Internal Components

Representative internal components include:

- Decision Aggregate
- Policy Aggregate
- Authority Aggregate
- Governance Repository Interface
- Approval Engine
- Policy Evaluator
- Authority Validator
- Governance Rules Engine

---

## Owned Data

The Governance Domain owns:

- Decisions
- Policies
- Standards
- Delegated Authorities
- Governance Records
- Approval History

---

## Published Events

Representative events include:

- DecisionApproved
- DecisionRejected
- PolicyPublished
- PolicyRetired
- AuthorityDelegated
- AuthorityRevoked

---

## Consumed Events

Representative events include:

- MissionCompleted
- OrganizationUpdated
- CapabilityRegistered
- MemoryInstitutionalized

Governance evaluates organizational changes but does not own them.

---

## Persistence Responsibilities

The Governance Domain exclusively owns persistence of:

- governance aggregates;
- authority records;
- approval history;
- policy metadata.

---

## Allowed Dependencies

The Governance Domain may depend upon:

- shared kernel abstractions;
- event contracts;
- common value objects.

---

## Forbidden Dependencies

The Governance Domain shall not depend upon:

- Mission Domain
- Knowledge Domain
- Infrastructure
- Presentation

---

## Extension Points

Supported extensions include:

- policy evaluators;
- governance validators;
- approval strategies;
- compliance providers.

---

## Architectural Invariants

The following constraints shall always remain true.

- Governance authority is singular.
- Decision history is immutable.
- Policies are versioned.
- Delegated authority is explicitly traceable.
- Governance rules remain independent of infrastructure implementation.

---

# Implementation Domain — Application Services

## Purpose

The Application Services Domain coordinates use cases across Implementation Domains.

It does **not** own business rules.

Application Services orchestrate execution by invoking domain interfaces and publishing application-level outcomes.

---

## Public Interfaces

Representative application services include:

- Create Organization
- Execute Mission
- Promote Knowledge
- Record Memory
- Approve Decision
- Evaluate Organization Health

These interfaces form the primary entry point for backend execution.

---

## Internal Components

Representative internal components include:

- Command Handlers
- Query Handlers
- DTO Mappers
- Transaction Coordinator
- Authorization Coordinator
- Event Dispatcher
- Application Service Registry

---

## Owned Data

Application Services own no persistent business entities.

They may own transient execution state such as:

- command context;
- transaction scope;
- execution metadata.

---

## Published Events

Representative events include:

- ApplicationCommandCompleted
- ApplicationCommandFailed
- QueryExecuted

Business events remain the responsibility of their originating domains.

---

## Consumed Events

Application Services may consume:

- UI commands;
- scheduled tasks;
- plugin requests;
- integration requests.

These inputs are translated into domain operations.

---

## Persistence Responsibilities

Application Services own no repositories.

Persistence is delegated entirely to the owning Implementation Domain.

---

## Allowed Dependencies

Application Services may depend upon:

- every business domain;
- shared kernel abstractions;
- domain interfaces;
- infrastructure abstractions.

Dependencies shall always point toward published interfaces.

---

## Forbidden Dependencies

Application Services shall not depend upon:

- repository implementations;
- desktop runtime internals;
- operating system APIs.

---

## Extension Points

Supported extensions include:

- additional command handlers;
- additional query handlers;
- orchestration pipelines;
- authorization middleware.

---

## Architectural Invariants

The following constraints shall always remain true.

- Application Services own no business rules.
- Every use case terminates within an owning Implementation Domain.
- Domain state changes occur only through domain interfaces.
- Transaction boundaries are coordinated here.
- DTOs never become domain entities.

*End of Part 4.*

# Implementation Domain — Infrastructure

## Purpose

The Infrastructure Domain provides concrete implementations for technical capabilities required by ForgeOS.

Infrastructure exists to support the business domains. It never defines organizational behavior.

Typical responsibilities include:

- persistence implementations;
- search providers;
- AI provider adapters;
- filesystem adapters;
- import/export;
- notification providers;
- networking;
- telemetry.

---

## Public Interfaces

Infrastructure exposes implementations of interfaces owned by other domains.

Representative implementations include:

- Organization Repository
- Mission Repository
- Knowledge Repository
- Event Publisher
- Search Provider
- AI Provider Adapter
- Storage Provider

Infrastructure shall not expose business-oriented APIs.

---

## Internal Components

Representative internal components include:

- Repository Implementations
- AI Adapter
- Storage Adapter
- Filesystem Adapter
- Search Adapter
- Plugin Loader
- Event Bus Implementation
- Logging Provider
- Configuration Provider

---

## Owned Data

Infrastructure owns only technical operational data, including:

- cache entries;
- index metadata;
- provider configuration;
- temporary files;
- telemetry records.

Infrastructure does **not** own organizational entities.

---

## Published Events

Representative events include:

- ProviderConnected
- ProviderDisconnected
- SearchIndexUpdated
- PluginLoaded
- PluginUnloaded
- InfrastructureHealthChanged

---

## Consumed Events

Infrastructure may consume domain events solely to execute technical work.

Examples:

- KnowledgePromoted
- MissionCompleted
- OrganizationUpdated

Infrastructure shall not interpret organizational meaning.

---

## Persistence Responsibilities

Infrastructure implements persistence.

Ownership remains with the originating Implementation Domain.

Infrastructure shall not redefine repository contracts.

---

## Allowed Dependencies

Infrastructure may depend upon:

- Domain-owned interfaces;
- Platform services;
- external libraries;
- operating system services.

---

## Forbidden Dependencies

Infrastructure shall not depend upon:

- Presentation;
- Application workflows;
- domain implementation details.

---

## Extension Points

Supported extension points include:

- storage providers;
- AI providers;
- search engines;
- import/export providers;
- notification providers.

---

## Architectural Invariants

The following constraints shall always remain true.

- Infrastructure owns no business rules.
- Infrastructure implements interfaces but never defines them.
- External technologies remain replaceable.
- Provider failures shall not corrupt domain state.
- Infrastructure dependencies remain outward-facing.

---

# Implementation Domain — Platform

## Purpose

The Platform Domain provides runtime capabilities independent of business functionality.

Examples include:

- application lifecycle;
- dependency injection;
- configuration;
- diagnostics;
- runtime health;
- process management.

---

## Public Interfaces

Representative interfaces include:

- Runtime Bootstrap
- Lifecycle Manager
- Configuration Service
- Diagnostics Service
- Health Monitor

---

## Internal Components

Representative internal components include:

- Bootstrap Coordinator
- Dependency Container
- Configuration Manager
- Runtime Monitor
- Diagnostics Manager

---

## Owned Data

Platform owns:

- runtime configuration;
- diagnostic state;
- runtime health metadata.

Persistent business data is prohibited.

---

## Published Events

Representative events include:

- RuntimeStarted
- RuntimeStopping
- ConfigurationReloaded
- DiagnosticsUpdated

---

## Consumed Events

Representative events include:

- ApplicationStarting
- ApplicationStopping

---

## Persistence Responsibilities

Platform persists only technical runtime metadata where required.

---

## Allowed Dependencies

Platform may depend upon:

- operating system services;
- desktop runtime;
- infrastructure bootstrap.

---

## Forbidden Dependencies

Platform shall not depend upon business domains.

---

## Extension Points

Supported extensions include:

- runtime diagnostics;
- startup hooks;
- health checks;
- monitoring providers.

---

## Architectural Invariants

The following constraints shall always remain true.

- Platform contains no organizational behavior.
- Platform owns no business entities.
- Platform remains replaceable independently of business domains.
- Runtime services remain technology-oriented.

---

# Implementation Domain — Presentation

## Purpose

The Presentation Domain provides the graphical user interface and user interaction model.

It translates user intent into Application Service requests and renders responses.

---

## Public Interfaces

Representative interfaces include:

- User Commands
- User Queries
- Navigation
- Notifications
- Dashboards
- Workspace Views

---

## Internal Components

Representative internal components include:

- Window Manager
- Navigation Controller
- View Models
- UI Components
- Command Dispatcher
- State Store

---

## Owned Data

Presentation owns transient UI state only.

Examples:

- window layout;
- navigation state;
- selected entities;
- view preferences.

Business entities are not owned by this domain.

---

## Published Events

Representative events include:

- UserCommandRequested
- ViewOpened
- ViewClosed
- NotificationAcknowledged

---

## Consumed Events

Representative events include:

- ApplicationCommandCompleted
- DomainNotifications
- RuntimeStatusChanged

---

## Persistence Responsibilities

Presentation may persist user interface preferences.

Business persistence is prohibited.

---

## Allowed Dependencies

Presentation may depend upon:

- Application Services;
- desktop runtime;
- UI framework.

---

## Forbidden Dependencies

Presentation shall not depend upon:

- Domain;
- repository implementations;
- storage providers;
- AI providers.

---

## Extension Points

Supported extensions include:

- dashboards;
- panels;
- menus;
- inspectors;
- custom views.

---

## Architectural Invariants

The following constraints shall always remain true.

- Presentation owns no business logic.
- Presentation communicates exclusively through Application Services.
- Domain entities never cross the UI boundary directly.
- UI state remains transient.
- Business workflows remain outside Presentation.

---

# Domain Relationship Matrix

| Domain | Primary Responsibility | Architectural Owner |
|----------|------------------------|---------------------|
| Core Runtime | Runtime coordination | Core Runtime |
| Organization | Organizational identity | Organization Domain |
| Mission | Organizational execution | Mission Domain |
| Process | Workflow execution | Process Domain |
| Knowledge | Organizational knowledge | Knowledge Domain |
| Memory | Institutional memory | Memory Domain |
| Workforce | Organizational workforce | Workforce Domain |
| Governance | Organizational authority | Governance Domain |
| Application Services | Use case orchestration | Application Services |
| Infrastructure | Technical implementations | Infrastructure |
| Platform | Runtime services | Platform |
| Presentation | User interaction | Presentation |

Every runtime component shall have exactly one architectural owner.

---

# Architectural Ownership Summary

The Implementation Domains defined in this document are the authoritative owners of ForgeOS Core.

Subsequent artifacts shall not redefine ownership.

Specifically:

- **ARCH-0003 — Dependency Rules** shall derive all permitted dependencies from these domains.
- **ARCH-0004 — Workspace Specification** shall map these domains into the physical Cargo workspace.
- No implementation may introduce a new architectural owner without an approved architectural change.

---

# Cross References

Authoritative ownership:

- Architectural intent — RFC Series
- System Architecture — TDS-0001
- Technology Decisions — TDR Series
- Runtime Context — ARCH-0001
- Component Ownership — **ARCH-0002 (this document)**

---

# Codex Readiness

## Implementation Status

**Ready for implementation of ForgeOS Core domain boundaries.**

A Senior Software Engineer can now:

- identify every Implementation Domain;
- determine the architectural owner of every future crate;
- understand domain responsibilities;
- identify persistence ownership;
- define public interfaces;
- implement domain events;
- enforce architectural invariants.

without inventing domain boundaries.

## Remaining Architectural Dependencies

Implementation of the Cargo workspace requires:

- **ARCH-0003 — Dependency Rules**
- **ARCH-0004 — Workspace Specification**

These documents define dependency enforcement and physical workspace organization. They do not modify the architectural ownership established here.

## Architectural Stability

The Implementation Domain model is considered stable for the ForgeOS MVP.

Future architectural evolution shall preserve:

- single architectural ownership;
- explicit domain boundaries;
- interface-driven communication;
- domain-owned persistence;
- published architectural invariants.

Changes to these principles require formal architectural review.

---

# Document Completion

This document is complete.

It is the authoritative implementation specification for the internal decomposition of ForgeOS Core and the foundation for dependency enforcement and Rust workspace organization.