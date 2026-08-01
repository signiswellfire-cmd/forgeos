# ForgeOS Testing Standard

**Document Version:** 1.0.0

---

# Purpose

This standard establishes the testing philosophy, requirements, and quality expectations for every software component developed within ForgeOS.

Testing exists to provide confidence that engineering knowledge has been implemented correctly, continues to behave as intended, and remains safe to evolve.

Testing is an engineering activity—not a quality assurance phase.

---

# Scope

This standard applies to:

* `forgeos-core`
* `forgeos-desktop`
* `forgeos-sdk`
* Internal libraries
* Plugins maintained by ForgeOS
* Build automation
* Infrastructure components where applicable

This document governs automated testing. Manual exploratory testing supplements—but does not replace—these requirements.

---

# Context

ForgeOS is intended to become a long-lived engineering platform.

Long-lived systems require confidence that change does not unintentionally alter established behavior.

Without consistent testing standards, every modification increases uncertainty, discourages refactoring, and weakens organizational confidence.

Testing preserves organizational trust in the software.

---

# Problem Statement

Engineering organizations commonly experience:

* fragile refactoring,
* regressions,
* undocumented assumptions,
* inconsistent testing practices,
* overreliance on manual verification,
* excessive end-to-end testing with insufficient unit coverage.

These issues reduce delivery speed and increase maintenance cost.

---

# Decision

ForgeOS adopts a **testing pyramid** supported by deterministic business logic and automated validation.

Testing shall verify behavior rather than implementation.

Tests are considered permanent engineering assets.

---

# Testing Principles

## Business Rules First

Business rules are the most valuable part of the system.

They shall receive the highest testing priority.

Infrastructure should be replaceable without rewriting business-rule tests.

---

## Deterministic Tests

Every automated test should produce identical results for identical inputs.

Tests shall avoid:

* timing dependencies,
* hidden state,
* execution order dependencies,
* external network requirements,
* shared mutable data.

Non-deterministic tests are considered defects.

---

## Fast Feedback

The majority of tests should execute quickly.

Engineers should receive feedback within minutes rather than hours.

Slow test suites discourage frequent execution.

---

## Behavior Over Implementation

Tests should validate externally observable behavior.

Implementation details may evolve without requiring widespread test changes.

Tests tightly coupled to implementation increase maintenance cost.

---

## Independent Execution

Every test should execute independently.

No test should rely on:

* execution order,
* previously created data,
* global application state,
* results produced by another test.

---

## Automation by Default

Every regression should result in an automated test whenever practical.

Manual verification alone is insufficient for permanent engineering knowledge.

---

# Testing Pyramid

ForgeOS adopts the following testing distribution.

## Unit Tests

Purpose:

Validate individual business rules.

Characteristics:

* Fast
* Deterministic
* Isolated
* High volume

Expected coverage:

Highest.

---

## Integration Tests

Purpose:

Validate interaction between components.

Examples:

* Repository implementations
* Persistence
* Event handling
* Plugin integration

Expected coverage:

Moderate.

---

## System Tests

Purpose:

Validate complete workflows.

Examples:

* Mission lifecycle
* Organization creation
* Knowledge promotion

Expected coverage:

Target critical business workflows.

---

## End-to-End Tests

Purpose:

Validate user-facing capabilities.

Characteristics:

* Lowest quantity
* Highest execution cost
* Highest maintenance cost

Only essential business scenarios should be automated end-to-end.

---

# Test Design

Good tests should be:

* understandable,
* deterministic,
* independent,
* repeatable,
* maintainable,
* focused on one behavior.

A failing test should identify a single engineering problem.

---

# Test Naming

Test names should describe behavior.

Preferred examples:

* createsMissionSuccessfully
* rejectsDuplicateOrganization
* promotesKnowledgeAfterValidation

Avoid generic names such as:

* test1
* works
* validationTest

---

# Test Data

Test data should be:

* minimal,
* explicit,
* representative,
* isolated.

Avoid unnecessary fixtures or overly complex datasets.

Synthetic data should be preferred unless production characteristics are required.

---

# Mocking

Mock only external boundaries.

Examples include:

* databases,
* AI providers,
* network services,
* file systems,
* external APIs.

Avoid mocking business rules.

Excessive mocking weakens confidence in system behavior.

---

# Code Coverage

Coverage is a diagnostic metric rather than a quality objective.

High coverage with poor assertions is less valuable than lower coverage with meaningful behavioral verification.

Coverage targets should prioritize:

1. Business rules.
2. Domain logic.
3. Mission workflows.
4. Knowledge management.
5. Organizational governance.

---

# Continuous Integration

Every pull request should automatically execute:

* static analysis,
* formatting verification,
* unit tests,
* integration tests,
* documentation validation (where applicable).

Failures should block integration until resolved.

The main branch should remain releasable at all times.

---

# Defect Resolution

Every confirmed regression should include:

1. A failing automated test.
2. The implementation fix.
3. Verification that the new test passes.

This prevents recurrence and strengthens organizational knowledge.

---

# Review Criteria

Testing reviews should evaluate:

* behavioral correctness,
* readability,
* determinism,
* isolation,
* execution speed,
* maintainability,
* meaningful assertions.

Reviewers should reject tests that merely increase coverage without improving confidence.

---

# Alternatives Considered

## Manual Testing Only

Rejected because manual testing cannot preserve engineering knowledge or reliably prevent regressions.

---

## Coverage-Driven Development

Rejected because coverage percentages do not measure engineering quality.

---

## End-to-End Heavy Testing

Rejected because excessive end-to-end testing produces slow feedback and fragile test suites.

---

# Consequences

Positive outcomes include:

* safer refactoring,
* reduced regressions,
* higher engineering confidence,
* faster delivery,
* improved maintainability,
* stronger organizational knowledge.

Trade-offs include:

* additional implementation effort,
* continuous maintenance of automated tests,
* greater engineering discipline.

These trade-offs are accepted because they improve long-term software quality.

---

# Future Considerations

Technology-specific testing guides may extend this standard.

Future standards may define:

* performance testing,
* security testing,
* AI validation,
* plugin certification,
* compatibility testing.

Those documents shall remain consistent with this standard.

---

# Relationship to Other Documents

This standard implements the engineering philosophy established by:

* `docs/philosophy/ENGINEERING_PRINCIPLES.md`
* `docs/philosophy/CORE_VALUES.md`
* `docs/standards/CODING_STANDARD.md`
* `docs/standards/ARCHITECTURE_STANDARD.md`

It is the authoritative testing standard for ForgeOS.
