# Founder Guide

## Purpose

This guide explains how the Founder should work with ForgeOS throughout the lifetime of the project.

ForgeOS is designed to reduce the Founder's cognitive load by transforming engineering knowledge into permanent organizational assets. The Founder is responsible for defining direction, while the engineering organization is responsible for turning that direction into executable, documented, and maintainable systems.

This document defines that relationship.

---

# Founder Responsibilities

The Founder owns decisions that determine the identity and future of the product.

These responsibilities include:

* Product vision
* Business strategy
* Customer value
* Market positioning
* Licensing
* Commercial direction
* Investment priorities
* Final approval for architectural changes that affect long-term product direction

The Founder should avoid making implementation decisions unless they materially affect one of the responsibilities listed above.

---

# Engineering Responsibilities

The engineering organization owns the technical execution of the product.

This includes:

* Architecture
* Engineering standards
* Documentation
* Technical design
* Technology selection
* Repository organization
* Code quality
* Testing strategy
* Technical debt management

Engineering decisions should be made according to documented standards and supported by evidence.

---

# Decision Framework

Every decision should be classified before work begins.

## Product Decisions

Examples:

* New product capabilities
* Pricing model
* Licensing changes
* Target audience
* Business priorities

These require Founder approval.

---

## Engineering Decisions

Examples:

* Internal architecture
* Refactoring
* Component boundaries
* Technology upgrades
* Repository organization
* Documentation improvements

These are owned by engineering unless they materially affect product direction.

---

# Repository First

The repository is the permanent engineering memory of ForgeOS.

Whenever an important architectural or technical decision is made, it should be recorded in the appropriate engineering document.

Conversations are for collaboration.

The repository is for institutional knowledge.

---

# Engineering Lifecycle

ForgeOS follows a documentation-first engineering process.

1. Identify a problem.
2. Review existing documentation.
3. Produce or update the appropriate RFC, TDS, or TDR.
4. Obtain approval when required.
5. Implement.
6. Validate.
7. Preserve resulting knowledge.

Implementation should never become the primary source of architectural truth.

---

# Working with the Chief Software Architect

The Chief Software Architect is expected to:

* Challenge assumptions respectfully.
* Improve approved ideas when appropriate.
* Reduce unnecessary complexity.
* Protect architectural integrity.
* Preserve engineering knowledge.
* Think in long-term organizational terms rather than short-term feature delivery.

Agreement is not the objective.

Engineering excellence is.

---

# Chat Sessions

Chat sessions are temporary.

Before ending a session, ensure the repository is updated with:

* `docs/PROJECT_STATUS.md`
* `docs/NEXT_SESSION.md`
* `docs/HANDOVER.md` (when required)

A future engineering session should begin by reading the repository rather than relying on previous conversations.

---

# Success Criteria

ForgeOS succeeds when:

* Product knowledge is preserved.
* Architectural decisions are traceable.
* Engineering standards are consistently applied.
* New contributors can understand the project from the repository alone.
* The organization continues to improve regardless of which engineer—or AI model—is currently contributing.

The ultimate objective is to build an engineering organization whose knowledge outlives every individual contributor.
