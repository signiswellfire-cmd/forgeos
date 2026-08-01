# ForgeOS Documentation Standard

**Document Version:** 1.0.0

---

# Purpose

This standard establishes the documentation requirements for every permanent engineering artifact produced within ForgeOS.

Documentation is not considered supplementary material. It is a primary engineering deliverable that preserves organizational knowledge, explains engineering decisions, and enables long-term maintainability.

Every document should remain valuable years after its creation.

---

# Scope

This standard applies to:

* Philosophy documents
* Genome documents
* RFCs
* Technical Design Specifications (TDS)
* Technology Decision Records (TDR)
* Engineering Standards
* Architecture documentation
* Public APIs
* Source code documentation
* Operational runbooks
* Developer guides
* Repository governance documents

Conversation history, meeting notes, and temporary planning documents are outside the scope of this standard.

---

# Context

Software systems evolve.

Engineering organizations evolve.

Contributors change.

Technologies become obsolete.

Documentation exists to ensure that engineering knowledge survives those changes.

ForgeOS treats documentation as permanent organizational memory rather than project paperwork.

---

# Problem Statement

Engineering documentation often becomes outdated because it is written as an afterthought.

Typical problems include:

* undocumented architectural decisions,
* duplicated information,
* inconsistent terminology,
* implementation without rationale,
* documentation that mirrors code instead of explaining intent.

These problems reduce engineering effectiveness over time.

---

# Decision

ForgeOS adopts a documentation-first engineering process.

Permanent engineering knowledge shall be documented before it becomes implementation.

Documentation is considered part of the product and is subject to the same quality expectations as source code.

---

# Documentation Principles

## Documentation Before Implementation

Architectural decisions should be documented before implementation begins.

Implementation may refine documentation but should not silently redefine architecture.

---

## Single Source of Truth

Every permanent concept shall have one authoritative document.

Other documents shall reference that source rather than duplicate information.

Duplication is treated as documentation debt.

---

## Explain Decisions, Not Just Results

Documentation should explain:

* why the problem exists,
* why the chosen solution was selected,
* which alternatives were evaluated,
* what trade-offs were accepted.

Future contributors should not need historical conversations to understand engineering intent.

---

## Stable Knowledge

Documentation should describe enduring engineering knowledge.

Temporary implementation details belong in code, issue trackers, or development notes unless they become permanent engineering knowledge.

---

## Technology Independence

Where practical, documentation should describe concepts rather than implementation technologies.

Implementation technologies may evolve.

Engineering principles should remain stable.

---

## Traceability

Every significant engineering decision should be traceable.

Architecture should be reconstructable from repository documentation alone.

---

# Required Structure

Authoritative engineering documents should include, where appropriate:

* Purpose
* Scope
* Context
* Problem Statement
* Decision
* Rationale
* Alternatives Considered
* Consequences
* Future Considerations
* Relationship to Other Documents

Additional sections may be added when required by the document type.

---

# Writing Standards

Documentation should be:

* concise,
* technically accurate,
* technology-neutral where possible,
* professionally written,
* self-contained,
* versioned,
* maintainable.

Avoid conversational language.

Avoid implementation-specific assumptions unless required.

Avoid unnecessary repetition.

---

# Terminology

Terminology should remain consistent throughout the repository.

Defined concepts should retain consistent capitalization and meaning.

Examples include:

* Organization
* Mission
* Professional
* Blueprint
* Capability
* Knowledge Object
* Genome

New terminology should be introduced deliberately and documented within the project glossary when appropriate.

---

# Versioning

Every authoritative document should include:

* document version,
* last updated date (where appropriate),
* repository path,
* ownership through the repository hierarchy.

Major structural changes should be reflected through version updates.

---

# Review Requirements

Documentation reviews should evaluate:

* correctness,
* clarity,
* completeness,
* consistency,
* traceability,
* architectural alignment,
* duplication.

A document should improve organizational understanding rather than merely satisfy process requirements.

---

# Alternatives Considered

## Documentation After Implementation

Rejected because architectural intent is frequently lost once implementation begins.

---

## Code as Documentation

Rejected because code cannot adequately explain rationale, governance, or organizational decisions.

---

## Distributed Ownership

Rejected because multiple authoritative documents create inconsistency and maintenance overhead.

---

# Consequences

Positive outcomes include:

* durable engineering knowledge,
* improved onboarding,
* explainable architecture,
* reduced documentation drift,
* stronger organizational memory.

Trade-offs include:

* greater documentation effort,
* higher review discipline,
* additional engineering governance.

These trade-offs are intentional and align with ForgeOS principles.

---

# Future Considerations

Future documentation templates should inherit this standard.

Specific document families such as RFCs, TDSs, and TDRs may define additional required sections, but they shall not weaken the principles established here.

Documentation tooling should support this standard rather than redefine it.

---

# Relationship to Other Documents

This standard implements the documentation philosophy established by:

* `docs/philosophy/PHILOSOPHY.md`
* `docs/philosophy/ENGINEERING_PRINCIPLES.md`
* `docs/philosophy/CONSTITUTION.md`

It serves as the authoritative documentation standard for all engineering artifacts within ForgeOS.
