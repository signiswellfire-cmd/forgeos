# ForgeOS Git Standard

**Document Version:** 1.0.0

---

# Purpose

This standard defines how Git is used within ForgeOS.

ForgeOS treats Git as more than a version control system. Git is the permanent engineering memory of the organization. Every commit should improve the organization's knowledge, not merely record changes to files.

This standard establishes consistent repository practices that maximize traceability, maintainability, and engineering confidence.

---

# Scope

This standard applies to:

* Source code
* Engineering documentation
* RFCs
* TDSs
* TDRs
* Standards
* Build scripts
* Configuration
* Repository structure

It applies to every repository maintained by the ForgeOS organization.

---

# Context

Repositories frequently become collections of unrelated commits that explain what changed but not why it changed.

Poor Git discipline results in:

* lost engineering rationale,
* difficult code reviews,
* complicated debugging,
* unreliable release history,
* fragmented organizational knowledge.

ForgeOS uses Git to preserve engineering decisions as permanent organizational assets.

---

# Problem Statement

Without consistent repository standards:

* commits become difficult to understand,
* unrelated changes become mixed,
* architectural evolution becomes impossible to reconstruct,
* releases become harder to validate,
* engineering history loses value.

Git history should communicate engineering intent rather than merely file differences.

---

# Decision

ForgeOS adopts a documentation-first Git workflow.

Repository history should tell the engineering story of the project from vision to implementation.

Every commit should represent one logical engineering change.

---

# Repository Principles

## Git Is Permanent Memory

The repository is the organization's permanent engineering memory.

Conversations are temporary.

Engineering knowledge must ultimately be preserved through committed artifacts.

---

## One Logical Change Per Commit

Each commit should represent one coherent engineering decision.

Examples:

* Add a new engineering standard.
* Introduce one RFC.
* Implement one architectural capability.
* Refactor one subsystem.

Avoid combining unrelated work into a single commit.

---

## Documentation Before Implementation

When implementation changes architecture or permanent engineering knowledge:

1. Update documentation.
2. Commit documentation.
3. Implement.
4. Commit implementation.

Implementation should follow documented decisions.

---

## Atomic History

Commits should remain independently understandable.

A reviewer should understand:

* what changed,
* why it changed,
* how it affects the system,

without requiring future commits for explanation.

---

## Clean Main Branch

The main branch should always represent a stable engineering baseline.

Incomplete experiments should not be committed directly to main.

---

# Commit Message Standard

ForgeOS uses conventional, descriptive commit messages.

Format:

```text
<type>: <summary>
```

Examples:

```text
docs: add RFC-0001 ForgeOS Genome
docs: complete bootstrap milestone B2 philosophy
feat(core): implement mission aggregate
feat(desktop): add mission dashboard
refactor(core): simplify knowledge repository
test(core): add mission lifecycle tests
fix(api): validate mission identifiers
build: update development container
chore: reorganize repository structure
```

Commit summaries should:

* be imperative,
* describe one logical change,
* remain concise,
* avoid unnecessary punctuation.

---

# Commit Types

Recommended commit types:

* docs
* feat
* fix
* refactor
* test
* build
* ci
* perf
* style
* chore

New commit types should be introduced only when they improve repository clarity.

---

# Branch Strategy

Branches should represent engineering work rather than personal work.

Preferred examples:

```text
feature/rfc-0001-genome
feature/mission-engine
feature/knowledge-engine

docs/philosophy
docs/standards

fix/mission-validation

refactor/repository-layer
```

Avoid ambiguous branch names such as:

```text
new
testing
changes
ricky-work
misc
```

---

# Pull Requests

Each pull request should:

* solve one engineering problem,
* include required documentation,
* preserve architectural consistency,
* pass automated validation,
* remain reviewable.

Large engineering initiatives should be decomposed into multiple pull requests whenever practical.

---

# Merge Strategy

Preferred merge order:

1. Documentation
2. Architecture
3. Tests
4. Implementation

Repository history should clearly show how engineering knowledge evolved.

---

# Tags

Version tags should identify significant engineering milestones.

Examples:

```text
v0.1.0-foundation
v0.2.0-founder-experience
v0.3.0-knowledge-engine
v1.0.0
```

Tags should represent meaningful project states rather than arbitrary commit counts.

---

# Binary Assets

Binary files should be minimized.

Whenever practical:

* prefer text formats,
* store diagrams as source,
* generate derived assets automatically.

Repository history should remain diff-friendly.

---

# Repository Organization

Repository structure should evolve deliberately.

Large reorganizations require architectural justification and appropriate documentation.

Directory structure should communicate organizational intent.

---

# Review Criteria

Repository reviews should evaluate:

* commit quality,
* history readability,
* logical grouping,
* documentation alignment,
* architectural traceability,
* repository cleanliness.

History is considered an engineering artifact and should receive the same care as source code.

---

# Alternatives Considered

## Timestamp-Based Commit Messages

Rejected because they communicate chronology rather than engineering intent.

---

## Large Batch Commits

Rejected because they reduce traceability and complicate reviews.

---

## Documentation After Implementation

Rejected because implementation alone cannot preserve engineering reasoning.

---

# Consequences

Positive outcomes include:

* understandable repository history,
* easier debugging,
* stronger architectural traceability,
* improved reviews,
* preserved engineering knowledge.

Trade-offs include:

* more disciplined commit practices,
* additional documentation effort,
* smaller, more frequent commits.

These trade-offs are intentional.

---

# Future Considerations

Future Git automation may include:

* commit validation,
* documentation verification,
* branch protection,
* architectural policy enforcement,
* automated changelog generation.

Automation should reinforce this standard rather than replace engineering judgment.

---

# Relationship to Other Documents

This standard implements repository governance established by:

* `docs/philosophy/CONSTITUTION.md`
* `docs/philosophy/ENGINEERING_PRINCIPLES.md`
* `docs/standards/DOCUMENTATION_STANDARD.md`

It is the authoritative Git workflow standard for ForgeOS.
