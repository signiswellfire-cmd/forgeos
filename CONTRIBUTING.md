# Contributing to ForgeOS

Thank you for contributing to ForgeOS.

ForgeOS is built as a long-term engineering organization rather than a collection of source code. Every contribution should improve not only the implementation but also the organization's permanent knowledge.

This document defines the engineering workflow and contribution standards for all contributors.

---

# Guiding Principles

All contributions must align with the core principles of ForgeOS:

* AI is temporary. Knowledge is permanent.
* Knowledge before implementation.
* Processes before automation.
* Architect for tomorrow. Implement for today.
* Founder Experience above feature count.
* Every important decision must be documented.
* Git is the permanent engineering memory.

When in doubt, optimize for long-term maintainability rather than short-term convenience.

---

# Repository as the Source of Truth

The repository is the authoritative source for:

* Product philosophy
* Engineering standards
* Architectural decisions
* Technical design
* Source code

No significant architectural decision should exist only in conversation, meeting notes, or personal knowledge.

---

# Before Writing Code

Before implementing a feature, contributors should determine whether the change affects architecture.

If the answer is yes, documentation comes first.

Typical workflow:

1. Identify the problem.
2. Review existing documentation.
3. Create or update the appropriate RFC, TDS, or TDR.
4. Obtain required approval.
5. Implement the approved design.
6. Update documentation if implementation introduces new permanent knowledge.

Implementation follows architecture—not the reverse.

---

# Documentation Ownership

Each category of knowledge has a single authoritative owner.

| Knowledge             | Authoritative Document          |
| --------------------- | ------------------------------- |
| Vision                | `docs/philosophy/VISION.md`     |
| Mission               | `docs/philosophy/MISSION.md`    |
| Philosophy            | `docs/philosophy/PHILOSOPHY.md` |
| Architecture          | `docs/rfcs/`                    |
| Technical Design      | `docs/tds/`                     |
| Technology Decisions  | `docs/tdr/`                     |
| Engineering Standards | `docs/standards/`               |

Avoid duplicating information across multiple documents. Reference the authoritative document instead.

---

# Pull Request Expectations

Every pull request should:

* Address a single logical change.
* Include documentation updates where applicable.
* Preserve backwards compatibility unless intentionally changed.
* Explain architectural impact.
* Avoid unrelated refactoring.

Large changes should be decomposed into smaller, reviewable pull requests whenever practical.

---

# Commit Guidelines

Use clear, descriptive commit messages.

Examples:

```text
docs: add project roadmap
docs: introduce RFC-0001 ForgeOS Genome
feat(core): implement mission aggregate
refactor(storage): simplify repository interface
test(core): add mission lifecycle tests
```

Keep commits focused on one purpose.

---

# Coding Standards

Implementation must comply with the project's engineering standards once published.

At minimum:

* Single responsibility per component.
* Explicit interfaces.
* Deterministic behavior.
* Clear naming.
* Comprehensive documentation for public APIs.
* Tests for business logic.

---

# Documentation Standards

Documentation should be:

* Self-contained.
* Versioned.
* Written in professional engineering language.
* Free of conversational context.
* Structured for long-term maintenance.

Architecture documents should explain:

* Context
* Problem Statement
* Decision
* Alternatives Considered
* Consequences
* Future Considerations

---

# Review Philosophy

Reviews should improve engineering quality rather than merely identify defects.

Reviewers are encouraged to:

* Challenge assumptions.
* Reduce unnecessary complexity.
* Protect architectural consistency.
* Improve clarity.
* Preserve knowledge.

Agreement is less important than sound engineering reasoning supported by evidence.

---

# Long-Term Goal

Every accepted contribution should make ForgeOS easier to understand, easier to evolve, and less dependent on any individual contributor.

The ultimate measure of success is whether future engineers can continue the project using the repository alone.
