# TDR-0006 — OrganizationId Generation

**TDR Number:** TDR-0006  
**Title:** OrganizationId Generation  
**Status:** Approved  
**Version:** 1.0.0  
**Related TDS:** TDS-0002 — Domain Model  
**Related TDR:** TDR-0001 — Programming Language  
**Related Milestones:** MILESTONE-001-DOMAIN-DECISIONS; MILESTONE-001.5 — Organization Domain Foundation

---

# Purpose

This record resolves the bounded identity-generation gap in the approved Create Organization domain contract: the mechanism by which the Organization aggregate generates its immutable, globally unique `OrganizationId`.

It records the generation strategy and the single dependency exception the strategy requires. It changes no RFC, TDS, Architecture document, manifest, or source file.

---

# Context

Approved requirements establish:

- RFC-0004: the Organization is the root identity; each ForgeOS instance contains exactly one Organization.
- TDS-0002 (Aggregate Identity): identity is immutable, globally unique within the repository, and stable throughout the aggregate lifecycle. The aggregate generates identity at creation.
- MILESTONE-001-DOMAIN-DECISIONS: callers never supply `OrganizationId`; the aggregate generates and retains it.
- TDR-0001: the implementation language is Rust.
- MILESTONE-001.3 / MILESTONE-001.5: the initial Organization Domain crate shall depend only on the Rust standard library, **unless an approved exception is recorded**.

Future architecture (RFC-0041 — Multi-Organization Architecture and the federation series) requires identities to remain globally unique across independent ForgeOS instances without a central coordinator.

Driving characteristics:

- local-first operation;
- full offline capability;
- persistence independence (no database-generated identity);
- Domain-layer ownership;
- deterministic testing via an injectable generator.

---

# Decision

1. **Generation strategy.** `OrganizationId` shall be generated using **version 4 UUID (UUID v4)** — 122 random bits supplied by a cryptographically secure random source.

2. **Domain ownership.** The Organization Domain owns an **`OrganizationIdGenerator` contract** through which the aggregate requests identity generation during creation. Producers of the aggregate receive a generator dependency; tests inject a deterministic generator.

3. **Standard implementation.** The standard generator implementation produces a random UUID v4 using the Rust **`uuid`** crate with the **`v4` feature only**. No serde, `v1`/`v3`/`v5`, timestamp, or other feature is enabled at the Domain layer.

4. **Approved dependency exception.** This TDR is the recorded exception referenced by MILESTONE-001.5 acceptance criterion 1: the `uuid` crate is approved for addition to `forgeos-organization-domain` when implementation begins. No Cargo manifest is modified by this record. All other dependency constraints for the Domain crate remain in force.

5. **Representation.** `OrganizationId` remains an immutable Domain value object. Where a transport or persistence representation is later required, it shall use the canonical lowercase 36-character hyphenated textual form; that boundary is a separate future decision.

6. **Non-secret.** UUID v4 carries no cryptographic guarantee and must not be used as an authorization or security token.

---

# Rationale

- **Global uniqueness without coordination:** 122 random bits make collision probability negligible even across a federated ecosystem of many independent instances. This satisfies TDS-0002's "globally unique within the repository" without a central authority.
- **Local-first and offline:** generation consumes local entropy only; no network, shared service, or clock authority is required.
- **Persistence independence:** identity is produced before persistence and never by a database; this implements the approved "aggregate generates its own identity" rule.
- **Domain ownership:** both the value object and the generator contract remain Domain-owned; identity generation does not leak into Application, Infrastructure, or Platform.
- **Deterministic tests:** tests inject a deterministic generator behind the Domain-owned contract, preserving ISP-0009 determinism requirements.
- **Ecosystem stability:** the `uuid` crate is mature, widely adopted, `no_std`-capable, and minimal when restricted to the `v4` feature.
- **Not clock-dependent:** pure randomness avoids the clock-skew and time-ordering correctness hazards of timestamp-prefixed schemes.

---

# Alternatives Considered

## ULID

Lexicographically sortable, 48-bit timestamp plus 80 random bits.

Rejected because:

- Organization identity has no sortability requirement in ForgeOS;
- the timestamp prefix introduces clock dependence and creation-time disclosure;
- the `ulid` crate is less universally standard than `uuid`.

## Deterministic sequence / counter

A monotonically increasing counter without an external library.

Rejected because:

- not globally unique across instances without an instance-qualifier mechanism, which would itself require an unapproved distribution scheme;
- vulnerable to reset and time-ordering issues;
- constitutes hand-rolled identity machinery the architecture should avoid.

## Timestamp + instance-id composite

Rejected because it requires an instance-identifier bootstrap/distribution mechanism that is itself undeclared and would couple identity to infrastructure and deployment concerns.

## Database-generated identity (auto-increment)

Rejected because it violates TDS-0002's aggregate-generated identity rule and couples the Domain to persistence.

## Caller-supplied identity

Rejected by the approved MILESTONE-001-DOMAIN-DECISIONS contract: callers never supply `OrganizationId`.

---

# Consequences

Positive:

- negligible collision probability;
- native local-first, offline operation;
- persistence-independent identity;
- Domain-owned value object and generator contract;
- deterministic testability;
- one minimal, stable, approved dependency.

Trade-offs:

- one external crate (`uuid`, `v4` feature) enters `forgeos-organization-domain` — approved here as the recorded exception;
- IDs are non-sequential with no ordering semantics — acceptable; no ordering requirement exists;
- IDs are non-secret and must not be used as authorization tokens.

---

# Traceability

| Concern | Authority |
|---|---|
| Aggregate generates identity; callers never supply it | MILESTONE-001-DOMAIN-DECISIONS |
| Identity immutable, globally unique, stable | TDS-0002 |
| Rust/Cargo implementation | TDR-0001 |
| Domain crate dependency constraint and exception path | MILESTONE-001.3; MILESTONE-001.5 |
| Deterministic tests via injectable generator | ISP-0009; ISP-0010 |
| Federated multi-instance future context | RFC-0041 — Multi-Organization Architecture |

---

# References

* RFC-0004 — Organization Model
* RFC-0041 — Multi-Organization Architecture
* TDS-0002 — Domain Model
* TDR-0001 — Programming Language
* `docs/implementation/MILESTONE-001-DOMAIN-DECISIONS.md`
* `docs/implementation/MILESTONE-001.3-CRATE-INITIALIZATION-PLAN.md`
* `docs/implementation/MILESTONE-001.5-ORGANIZATION-DOMAIN.md`
* [uuid crate documentation](https://docs.rs/uuid/latest/uuid/)

TDR-0006 becomes the authoritative technology decision for `OrganizationId` generation in the Organization Domain.