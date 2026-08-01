# Milestone 001 — Create Organization Domain Decisions

**Status:** Approved  
**Version:** 1.0.0  
**Scope:** The bounded domain and application contract for the Create Organization vertical slice.

---

# Purpose

This decision package resolves the creation contract required by `MILESTONE-001-CREATE-ORGANIZATION.md`. It refines the approved Organization model only for the first creation use case; it does not change aggregate ownership, introduce a new bounded context, or define later Organization management behavior.

---

# Authority and Traceability

| Concern | Authority | Decision consequence |
|---|---|---|
| Organization is the root entity; each ForgeOS instance has exactly one Organization | RFC-0004 — Organization Model | Creation is a singleton operation for one ForgeOS instance. |
| Organization owns identity, profile, lifecycle, configuration, and hierarchy metadata | TDS-0002 — Organization Domain Context | The aggregate is the sole creation and mutation boundary. |
| Aggregate identity is immutable, globally unique, and stable | TDS-0002 — Identity Strategy | The aggregate generates and retains its identifier; callers do not supply it. |
| Aggregate lifecycle is Created → Initialized → Active → Modified → Archived | TDS-0002 — Aggregate Lifecycle | A successful creation initializes the aggregate and returns it in Active status. |
| Organization value-object vocabulary | TDS-0002 — Organization Value Objects | Name, type, status, version, and identity are represented as immutable domain value objects. |
| OrganizationRepository supports existence verification and optimistic concurrency | TDS-0002 — Organization Repository Contract | The application checks singleton existence before creation; persistence enforces the result atomically. |
| Error categories | TDS-0001 — Error Handling; ISP-0008 | Domain, application, and infrastructure failures have separate ownership and classification. |

---

# Decision

## Explicit Milestone 1 decisions

Neither RFC-0004 nor TDS-0002 specifies a field-level Create Organization API, an identifier-generation mechanism, or initial default values. To remove those bounded blockers, this approved package makes the following new Milestone 1 decisions within the constraints of those authorities:

* the caller provides only name and organization type;
* the aggregate generates its own identity;
* a newly created aggregate has no capabilities, hierarchy members, or additional configuration;
* its initial observable status is Active; and
* its initial version is 1.

These decisions do not alter the approved Organization aggregate, ownership, lifecycle order, or value-object vocabulary. Any expansion of the creation surface requires a subsequent documented decision.

## Command input contract

The Create Organization command accepts exactly these caller-provided fields:

| Field | Required | Meaning |
|---|---:|---|
| `name` | Yes | The requested `OrganizationName`. |
| `organization_type` | Yes | The requested `OrganizationType`. |

The caller does not provide an organization identifier, status, version, capabilities, hierarchy, configuration, or classification. Those concepts remain aggregate-owned. This is a deliberately minimal Milestone 1 surface and does not preclude later Organization commands.

## Aggregate creation semantics

The application service asks the Organization aggregate to create itself from the command's validated domain values. The aggregate:

1. generates an immutable, globally unique `OrganizationId`;
2. establishes the supplied name and type as Organization-owned metadata;
3. initializes the aggregate without capabilities, hierarchy members, or additional configuration; and
4. records the `OrganizationCreated` domain event.

The aggregate transitions through the conceptual Created and Initialized stages within this creation operation and is returned as Active. This implements, rather than alters, the lifecycle order defined by TDS-0002.

## Initial lifecycle and status

The initial externally observable `OrganizationStatus` is **Active**. The initial `OrganizationVersion` is **1**. These two initial values are bounded Milestone 1 business decisions necessary to make the approved lifecycle and optimistic-concurrency contract executable. They are not claimed to be pre-existing TDS field-level rules.

## Uniqueness requirement

Exactly one Organization may exist in each ForgeOS instance, as required by RFC-0004. Before creation, the application service uses the domain-owned repository's existence-verification contract. The concrete persistence implementation must also enforce the singleton constraint within the same transaction so concurrent requests cannot create two Organizations.

## Validation rules

The following validation is approved for Milestone 1:

1. `name` is required and must contain at least one non-whitespace character.
2. `organization_type` is required and must contain at least one non-whitespace character.
3. `name` and `organization_type` are preserved as supplied after boundary normalization; no case folding, display-name transformation, length limit, or type enumeration is introduced by this decision.
4. Creation is rejected when an Organization already exists for the ForgeOS instance.

Rules 1–3 are minimal structural validity rules for the approved required command inputs. Rule 4 derives from RFC-0004's exactly-one-Organization requirement. Further naming policy, OrganizationType taxonomy, profile fields, hierarchy content, capability defaults, and configuration defaults remain outside Milestone 1 and require separate authority.

## Result contract

On success, the application result contains only stable creation facts:

* `organization_id`;
* `name`;
* `organization_type`;
* `status` (`Active`); and
* `version` (`1`).

The result is a DTO-facing application contract, not the Organization aggregate. It exposes no mutable entity, repository, persistence, or event-dispatch detail.

---

# Domain and Application Error Model

| Error owner | Classification | Conditions | Boundary behavior |
|---|---|---|---|
| Domain | Validation | Missing or whitespace-only `name` or `organization_type` when creating their value objects | Returned as a domain validation error; no persistence attempt. |
| Domain/Application | Business Rule | An Organization already exists | Returned as an `OrganizationAlreadyExists` business-rule failure; no second aggregate is created. |
| Application | Validation | DTO cannot be structurally mapped to the command | Returned as an application validation error; the aggregate is not invoked. |
| Infrastructure | Infrastructure | Storage, migration, transaction, or repository operation fails | Translated at the infrastructure boundary; domain implementation details are not exposed. |
| Platform/Application | Security | The caller is not authorized after the approved authorization mechanism exists | Returned as a security error before domain execution. Milestone 1 does not define authentication or authorization policy. |
| Any boundary | Unexpected | A failure outside the above categories | Propagated as a structured unexpected error; it must not be silently suppressed or converted into a business rule. |

All recoverable failures use explicit result types. Domain errors remain domain-specific where practical. This table applies TDS-0001 and ISP-0008; it does not create a generic error abstraction or select an error-handling library.

---

# Consequences

The slice has a minimal, deterministic creation surface and a concrete singleton rule. The Organization aggregate stays authoritative, and later commands can add Organization management behavior without redefining creation.

The trade-off is that users cannot supply profile details, configuration, hierarchy, capabilities, or a richer OrganizationType taxonomy in Milestone 1. Those additions require new domain decisions and tests.

---

# Future Considerations

Future work may define OrganizationType values, profile and classification fields, lifecycle transitions beyond creation, configuration semantics, hierarchy population, capability registration, and authentication/authorization policy. None is implied by this package.

---

# References

* RFC-0004 — Organization Model
* TDS-0001 — System Architecture
* TDS-0002 — Domain Model
* TDS-0004 — Application Model
* ARCH-0002 — Component Model
* ISP-0001, ISP-0002, ISP-0004, ISP-0005, ISP-0008, ISP-0010
* `docs/implementation/MILESTONE-001-CREATE-ORGANIZATION.md`
