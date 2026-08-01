# ForgeOS Decision Index

**Document Version:** 1.0.0

**Last Updated:** 2026-08-01

---

# Purpose

This document is the master index of permanent engineering decisions within the ForgeOS repository.

Its purpose is to provide a single navigation point for locating the authoritative document that owns each architectural, technical, organizational, or engineering decision.

This document intentionally does **not** duplicate decision content.

Every decision has exactly one authoritative owner.

---

# Ownership Principle

ForgeOS follows the principle of **single ownership of knowledge**.

Each permanent decision must have one authoritative document.

Other documents should reference the authoritative source instead of repeating the same information.

---

# Decision Status

Each indexed decision uses one of the following states.

| Status     | Meaning                                                        |
| ---------- | -------------------------------------------------------------- |
| Planned    | Decision has been identified but not yet documented.           |
| Draft      | Engineering document is being written.                         |
| Approved   | Decision has been approved and is the current source of truth. |
| Superseded | A newer decision replaces this entry.                          |
| Deprecated | Decision is retained for historical reference only.            |

---

# Philosophy Decisions

| Decision                 | Authoritative Document                      | Status  |
| ------------------------ | ------------------------------------------- | ------- |
| Product Vision           | `docs/philosophy/VISION.md`                 | Planned |
| Product Mission          | `docs/philosophy/MISSION.md`                | Planned |
| Core Philosophy          | `docs/philosophy/PHILOSOPHY.md`             | Planned |
| Engineering Constitution | `docs/philosophy/CONSTITUTION.md`           | Planned |
| Core Values              | `docs/philosophy/CORE_VALUES.md`            | Planned |
| Engineering Principles   | `docs/philosophy/ENGINEERING_PRINCIPLES.md` | Planned |

---

# Genome

| Decision       | Authoritative Document  | Status  |
| -------------- | ----------------------- | ------- |
| ForgeOS Genome | `docs/genome/GENOME.md` | Planned |

---

# Requests for Comments (RFC)

| RFC       | Subject            | Status  |
| --------- | ------------------ | ------- |
| RFC-0001  | ForgeOS Genome     | Planned |
| RFC-0002  | Knowledge Model    | Planned |
| RFC-0003  | Knowledge Graph    | Planned |
| RFC-0004  | Organization Model | Planned |
| RFC-0005  | Forge Pipeline     | Planned |
| RFC-0006+ | Reserved           | Planned |

---

# Technical Design Specifications (TDS)

| TDS       | Subject             | Status  |
| --------- | ------------------- | ------- |
| TDS-0001  | System Architecture | Planned |
| TDS-0002  | Domain Model        | Planned |
| TDS-0003  | Organization Model  | Planned |
| TDS-0004  | Mission Engine      | Planned |
| TDS-0005+ | Reserved            | Planned |

---

# Technology Decision Records (TDR)

| TDR       | Subject              | Status  |
| --------- | -------------------- | ------- |
| TDR-0001  | Programming Language | Planned |
| TDR-0002  | Desktop Framework    | Planned |
| TDR-0003  | Storage Strategy     | Planned |
| TDR-0004  | Event Bus            | Planned |
| TDR-0005+ | Reserved             | Planned |

---

# Engineering Standards

| Standard               | Authoritative Document                     | Status  |
| ---------------------- | ------------------------------------------ | ------- |
| Coding Standard        | `docs/standards/CODING_STANDARD.md`        | Planned |
| Documentation Standard | `docs/standards/DOCUMENTATION_STANDARD.md` | Planned |
| Architecture Standard  | `docs/standards/ARCHITECTURE_STANDARD.md`  | Planned |
| Testing Standard       | `docs/standards/TESTING_STANDARD.md`       | Planned |
| Git Standard           | `docs/standards/GIT_STANDARD.md`           | Planned |
| Naming Standard        | `docs/standards/NAMING_STANDARD.md`        | Planned |

---

# Repository Governance

This document should be updated whenever:

* A new RFC is created.
* A TDS is approved.
* A TDR is approved.
* A philosophy document becomes authoritative.
* A decision is superseded or deprecated.

The Decision Index should always allow a contributor to locate the current source of truth for any significant engineering decision within the repository.

---

# Bootstrap Completion

Bootstrap Milestone B1 is complete when every document listed in the Repository Foundation milestone exists in the repository and this Decision Index is established as the navigation entry point for all future engineering decisions.
