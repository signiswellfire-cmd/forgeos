# OrganizationType — Domain Representation Decision

**Status:** Approved  
**Version:** 1.0.0  
**Scope:** Bounded domain representation for the `OrganizationType` value object in the Create Organization domain contract. This record defines no source code and changes no RFC, TDS, Architecture document, manifest, or dependency.

---

# Context

The approved `MILESTONE-001-DOMAIN-DECISIONS.md` establishes:

- `organization_type` is a required caller-provided field for Create Organization;
- `organization_type` must contain at least one non-whitespace character;
- the value is **preserved as supplied** — no case folding, display-name transformation, length limit, or type enumeration is introduced; and
- further OrganizationType taxonomy is explicitly deferred and requires separate authority.

TDS-0002 defines value objects as immutable and identified solely by value. This record resolves the remaining question: how `OrganizationType` is represented in the Domain given that constraint.

---

# Approved Representation

`OrganizationType` is a **String-backed immutable value object** owned by the Organization Domain.

It holds the caller-supplied organization type exactly as provided after boundary normalization described below. It:

- is immutable after construction;
- compares by value;
- has no independent lifecycle and no repository;
- exposes no mutable state;
- is aggregatable metadata, not an enum, in this milestone.

The value object may be constructed only through a **validated constructor** that enforces the approved validation rules in this record. Callers cannot construct an invalid `OrganizationType`.

---

# Validation Rules

1. **Required:** `organization_type` must be present. Missing input is rejected at construction with a domain validation error.
2. **Non-whitespace content required:** the value must contain at least one non-whitespace character. Whitespace-only values are rejected with a domain validation error.
3. **Preserved as supplied:** after the boundary normalization below, the stored value is byte-for-byte the value the caller supplied. No case folding, no trimming of leading/trailing or internal whitespace beyond the boundary check, no length limit, no character-set restriction, and no string transformation is applied.
4. **No enumeration:** `OrganizationType` is not a closed set in Milestone 1.5. Any non-whitespace string is a valid construction input absent a future approved taxonomy.

## Boundary normalization (outer edges only)

Only this normalization applies at the IPC/application boundary, **before** the Domain value object sees the input:

- if the value consists solely of whitespace characters, it is treated as invalid under rule 2;
- otherwise the value is passed into the Domain **exactly as supplied**.

The Domain layer performs no additional normalization. The Domain value object's equality is therefore exact: two `OrganizationType` values are equal only if their stored strings are identical.

---

# Deferred Decisions

The following remain **outside** Milestone 1.5 and require separate authority before implementation:

- an OrganizationType taxonomy or controlled vocabulary (e.g., enumerated categories);
- case-folding, canonicalization, or display-name policies;
- minimum / maximum length limits;
- allowed-character or language restrictions;
- localization or display formatting;
- OrganizationType-driven behavior (workflows, validation, or capabilities selected by type);
- persistence or IPC serialization form of `OrganizationType`.

---

# Consequences

Positive:

- a minimal, deterministic, Domain-owned value object that satisfies the approved "preserved as supplied" rule;
- no behavioral coupling to an undeveloped taxonomy;
- exact-by-value equality, simplifying tests and future contract changes.

Trade-offs:

- callers can supply arbitrary strings, so the Domain stores any non-whitespace type; this is accepted because taxonomy is intentionally deferred;
- a future taxonomy change will require a new domain decision and potential migration of persisted values.

---

# Traceability

| Concern | Authority |
|---|---|
| `organization_type` is a required field, preserved as supplied, no enumeration | MILESTONE-001-DOMAIN-DECISIONS |
| Value objects are immutable, equality by value, no lifecycle, no repository | TDS-0002 |
| Validation error classification for missing/whitespace input | TDS-0001; ISP-0008 |
| Domain-owned value-object vocabulary | TDS-0002; ARCH-0002 |
| No dependencies or boundary changes in Milestone 1.5 | MILESTONE-001.5 |

---

# References

* RFC-0004 — Organization Model
* TDS-0001 — System Architecture
* TDS-0002 — Domain Model
* ARCH-0002 — Component Model
* ISP-0008 — Error Handling Pattern
* `docs/implementation/MILESTONE-001-DOMAIN-DECISIONS.md`
* `docs/implementation/MILESTONE-001.5-ORGANIZATION-DOMAIN.md`

This record is the authoritative Milestone 1.5 representation decision for `OrganizationType`.