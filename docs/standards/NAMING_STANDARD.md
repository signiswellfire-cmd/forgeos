# ForgeOS Naming Standard

**Document Version:** 1.0.0

---

# Purpose

This standard defines the naming conventions used throughout the ForgeOS ecosystem.

Consistent naming is essential for maintaining architectural clarity, reducing cognitive load, improving discoverability, and preserving engineering knowledge over the lifetime of the project.

Naming is considered an architectural concern rather than a stylistic preference.

---

# Scope

This standard applies to every permanent artifact within ForgeOS, including:

* Source code
* Modules
* Packages
* Services
* APIs
* Domain entities
* RFCs
* TDSs
* TDRs
* Engineering Standards
* Documentation
* Repository directories
* Database objects
* Events
* Commands
* Configuration
* Plugins

---

# Context

As engineering organizations grow, inconsistent terminology becomes one of the largest sources of unnecessary complexity.

Different names for identical concepts create:

* duplicated knowledge,
* inconsistent implementations,
* onboarding difficulty,
* architectural drift,
* misunderstanding between engineering teams.

ForgeOS adopts a repository-wide naming standard to preserve a common engineering language.

---

# Problem Statement

Without consistent naming conventions:

* identical concepts acquire multiple names,
* repositories become harder to navigate,
* APIs become inconsistent,
* documentation loses authority,
* organizational knowledge fragments.

The engineering language of the organization becomes unstable.

---

# Decision

ForgeOS adopts a **domain-driven naming strategy**.

Names should describe business meaning before implementation details.

The same concept shall always use the same name throughout the repository.

---

# Naming Principles

## Domain Before Technology

Names should reflect business concepts rather than implementation.

Preferred:

* Mission
* Professional
* Knowledge
* Capability
* Blueprint
* Organization

Avoid implementation-driven names such as:

* Manager
* Handler
* Processor
* Util
* Helper

unless they genuinely describe the responsibility.

---

## One Concept, One Name

Each business concept shall have one canonical name.

Examples:

Always use:

* Mission

Never alternate between:

* Job
* Task
* Work Item
* Ticket

unless they represent genuinely different concepts.

---

## Intent Before Implementation

Names should communicate purpose.

Preferred:

* MissionRepository
* KnowledgePromotionService
* ExecutiveMeeting

Avoid:

* MissionRepositoryImpl2
* FinalManager
* MiscUtility

---

## Avoid Abbreviations

Abbreviations should only be used when universally recognized.

Preferred:

* Organization

Avoid:

* Org

Preferred:

* Repository

Avoid:

* Repo

Exceptions:

* RFC
* API
* SDK
* UUID
* JSON
* SQL

---

## Stable Terminology

Once a domain concept is introduced, its name should remain stable.

Renaming architectural concepts requires deliberate review because terminology becomes organizational knowledge.

---

# Repository Naming

Directories should use:

* lowercase
* hyphen-separated

Examples:

```text
forgeos-core
forgeos-desktop
forgeos-sdk
mission-engine
knowledge-engine
```

Avoid:

```text
ForgeOSCore
MissionEngine
miscStuff
```

---

# Document Naming

Engineering documents use uppercase identifiers.

Examples:

```text
RFC-0001.md
TDS-0003.md
TDR-0005.md
GENOME.md
VISION.md
MISSION.md
```

Engineering standards use descriptive uppercase names.

Examples:

```text
CODING_STANDARD.md
TESTING_STANDARD.md
ARCHITECTURE_STANDARD.md
```

---

# Source Code Naming

Language-specific conventions may vary.

General expectations:

Classes:

Use nouns.

Examples:

* Mission
* Organization
* Blueprint

Interfaces:

Describe capabilities.

Examples:

* KnowledgeRepository
* EventPublisher

Methods:

Use verbs.

Examples:

* createMission()
* validateKnowledge()
* promoteBlueprint()

Constants:

Use uppercase snake case.

Example:

```text
MAX_MISSION_DEPTH
```

Variables:

Use descriptive names.

Avoid:

```text
x
obj
tmp
data2
```

unless scope is extremely limited.

---

# Database Naming

Tables:

Use singular business nouns where supported by the technology.

Examples:

* mission
* organization
* knowledge

Columns:

Use descriptive lowercase snake_case.

Examples:

* created_at
* updated_at
* mission_status

Primary keys:

Prefer:

```text
id
```

Foreign keys:

```text
organization_id
mission_id
```

---

# Event Naming

Events should describe completed business actions.

Preferred:

* MissionCreated
* MissionCompleted
* KnowledgePromoted
* BlueprintPublished

Avoid:

* DoMission
* UpdateKnowledge
* ProcessThing

Events describe facts rather than commands.

---

# API Naming

APIs should expose business capabilities rather than implementation.

Preferred:

```text
CreateMission
ListKnowledge
PromoteBlueprint
```

Avoid technology-specific terminology in public APIs.

---

# Plugin Naming

Plugins should describe the capability they provide.

Examples:

* GitHub Integration
* Local LLM Provider
* Jira Connector
* Markdown Exporter

Avoid implementation-specific names.

---

# Review Criteria

Naming reviews should evaluate:

* consistency,
* clarity,
* domain alignment,
* discoverability,
* architectural meaning,
* long-term maintainability.

Reviewers should reject names that introduce ambiguity or duplicate existing terminology.

---

# Alternatives Considered

## Personal Naming Preferences

Rejected because organizational consistency outweighs individual preference.

---

## Technology-Specific Naming

Rejected because technologies evolve while domain concepts remain stable.

---

## Shortened Names

Rejected because reduced typing does not justify reduced clarity.

---

# Consequences

Positive outcomes include:

* consistent engineering language,
* improved discoverability,
* stronger documentation,
* reduced onboarding effort,
* clearer architecture,
* better long-term maintainability.

Trade-offs include:

* stricter review requirements,
* occasional refactoring,
* reduced stylistic freedom.

These trade-offs are accepted because they strengthen organizational knowledge.

---

# Future Considerations

Future language-specific standards may extend naming conventions where necessary.

Those extensions shall remain consistent with the domain-first principles defined in this document.

---

# Relationship to Other Documents

This standard supports:

* `docs/philosophy/ENGINEERING_PRINCIPLES.md`
* `docs/standards/CODING_STANDARD.md`
* `docs/standards/ARCHITECTURE_STANDARD.md`
* `docs/genome/GENOME.md`

It is the authoritative naming standard for every permanent engineering artifact within ForgeOS.
