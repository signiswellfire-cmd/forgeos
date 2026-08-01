# ForgeOS Coding Standard

**Document Version:** 1.0.0

---

# Purpose

This document establishes the mandatory coding standards for all ForgeOS software components.

Its objective is to ensure that every codebase remains understandable, maintainable, testable, and consistent regardless of contributor, programming language, or implementation technology.

These standards are intentionally technology-agnostic wherever possible. Language-specific conventions should extend this document rather than replace it.

---

# Scope

This standard applies to:

* `forgeos-core`
* `forgeos-desktop`
* `forgeos-sdk`
* Internal tools
* Build scripts
* Plugins maintained by the ForgeOS project

Third-party dependencies are excluded except where adapters or wrappers are implemented.

---

# Context

Code is one representation of engineering knowledge.

Poorly structured code obscures architectural intent, increases maintenance cost, and weakens organizational learning.

A consistent coding standard reduces unnecessary variation so engineers can focus on solving business problems rather than interpreting inconsistent implementation styles.

---

# Problem Statement

Without shared coding standards, engineering organizations commonly experience:

* inconsistent naming,
* duplicated logic,
* hidden dependencies,
* oversized components,
* unpredictable behavior,
* increased onboarding time,
* higher maintenance cost.

These issues compound as a project grows.

---

# Decision

ForgeOS adopts a consistency-first coding standard.

Every contribution should improve the readability, maintainability, and explainability of the codebase.

Code should communicate intent before optimization.

---

# Core Principles

## Readability First

Code is read far more frequently than it is written.

Optimize for comprehension before brevity.

---

## Explicit Over Implicit

Hidden behavior should be avoided.

Dependencies, assumptions, and side effects should be visible.

---

## Composition Over Complexity

Build systems from small, composable components.

Avoid monolithic implementations.

---

## Single Responsibility

Each module, class, function, and package should have one clearly defined responsibility.

Responsibilities should not overlap.

---

## Deterministic Behavior

Business logic should produce predictable outputs for identical inputs.

Avoid hidden state and unnecessary global behavior.

---

# Project Structure

Every component should have a clearly defined purpose.

Typical responsibilities include:

* Domain
* Application
* Infrastructure
* Interface
* Shared utilities

Cross-layer dependencies should remain intentional and documented.

---

# Function Guidelines

Functions should:

* Perform one logical operation.
* Have descriptive names.
* Minimize side effects.
* Return predictable results.
* Prefer explicit parameters over shared mutable state.

Large functions should be decomposed into smaller units.

---

# Naming

Names should communicate intent rather than implementation.

Prefer:

* `MissionRepository`
* `KnowledgePromotionService`
* `MissionLifecycle`

Avoid abbreviations unless universally understood.

---

# Error Handling

Errors are part of normal system behavior.

Every recoverable error should:

* communicate the problem,
* preserve useful context,
* support troubleshooting,
* avoid leaking implementation details.

Silent failures are prohibited.

---

# Logging

Logging should support engineering diagnostics.

Logs should be:

* structured,
* meaningful,
* actionable.

Sensitive information must never be logged.

---

# Comments

Comments should explain **why**, not **what**.

Well-written code should make implementation self-explanatory.

Comments that duplicate implementation should be removed.

---

# Documentation

Every public interface should include documentation describing:

* purpose,
* inputs,
* outputs,
* expected behavior,
* constraints.

Complex business rules should reference their authoritative engineering documentation where applicable.

---

# Dependencies

New dependencies should satisfy all of the following:

* solve a validated problem,
* have active maintenance,
* possess an appropriate license,
* reduce overall complexity,
* justify long-term maintenance cost.

Unnecessary dependencies are discouraged.

---

# Testing Expectations

Business logic should be independently testable.

Code should be designed to enable automated testing rather than requiring extensive mocking or manual verification.

Testing standards are defined separately in `TESTING_STANDARD.md`.

---

# Code Review Criteria

Every review should evaluate:

* correctness,
* readability,
* maintainability,
* architectural consistency,
* documentation,
* testability,
* unnecessary complexity.

Reviewers should improve engineering quality rather than merely identify defects.

---

# Alternatives Considered

## Personal Coding Styles

Rejected because inconsistent implementation increases organizational maintenance cost.

## Language-Specific Standards Only

Rejected because ForgeOS spans multiple technologies and requires consistent engineering principles across the repository.

---

# Consequences

Positive outcomes include:

* consistent implementation,
* easier onboarding,
* reduced technical debt,
* predictable architecture,
* improved maintainability.

Trade-offs include:

* additional review discipline,
* reduced stylistic freedom,
* greater emphasis on engineering consistency.

These trade-offs are intentional.

---

# Future Considerations

Language-specific supplements may extend this standard for Rust, TypeScript, Kotlin, or other implementation languages.

Those documents must remain consistent with this standard and may strengthen—but not weaken—its requirements.

---

# Relationship to Other Documents

This standard operationalizes:

* `docs/philosophy/ENGINEERING_PRINCIPLES.md`
* `docs/philosophy/CORE_VALUES.md`
* Future architecture RFCs
* Future testing and naming standards

It is the authoritative coding standard for the ForgeOS repository.
