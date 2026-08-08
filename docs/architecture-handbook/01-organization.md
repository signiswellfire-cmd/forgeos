# Architecture Handbook — Organization Bounded Context

**Document Type:** Explanatory Handbook (not architecture authority)

**Status:** Explanatory

---

## Purpose

The Organization bounded context owns organizational identity and structure. It is the root context upon which all other business contexts depend conceptually. The Organization context defines **who the organization is**, not **what it is doing**.

The Organization Unit provides enduring organizational identity and strategic direction.

---

## Responsibilities

Per TDS-0003, the Organization Unit owns:

- organizational identity;
- organizational strategy;
- organizational capability ownership;
- mission portfolio ownership;
- organizational evolution.

Per TDS-0002, the Organization context owns:

- organizational identity;
- organizational profile;
- organizational lifecycle;
- organizational configuration;
- organizational hierarchy metadata.

Per ARCH-0002, the Organization Domain owns:

- Organizations;
- Organization DNA;
- Organization Profiles;
- Organizational Hierarchy;
- Organizational Capabilities;
- Organization Health Records;
- Organizational Metadata.

---

## Key Concepts

| Concept | Meaning (per authority) |
|---------|--------------------------|
| Organization | The enduring operational entity responsible for achieving missions through governed execution. |
| Organizational Unit | A logical area of responsibility with defined authority and ownership. |
| Organizational Capability | A persistent competency owned by an organizational unit. |
| Organizational Responsibility | An obligation assigned to exactly one organizational owner. |
| Organizational Authority | The right to approve, reject, delegate, or govern organizational decisions. |
| Organization DNA | Organizational identity data owned exclusively by the Organization Domain. |
| Organization Health | Organizational health records owned by the Organization Domain. |

---

## Lifecycle

Per TDS-0003, the Organization Model defines how responsibilities evolve over time. Representative organizational lifecycle states include:

- Established;
- Operational;
- Evolving;
- Archived.

The lifecycle represents organizational evolution rather than implementation state.

Per TDS-0002, the Organization aggregate follows the general aggregate lifecycle:

- Created;
- Initialized;
- Active;
- Modified;
- Archived.

---

## Ownership

Per TDS-0003:

- The Organization Unit owns strategic organizational decisions.
- Strategic authority may not be delegated permanently.
- Every organizational responsibility has exactly one owner.
- Organizational ownership is permanent until formally reassigned.
- Ownership shall never be implied by implementation.

Per TDS-0002:

- The Organization aggregate is the authoritative root of the Organization bounded context.
- All mutations of organizational state shall occur through this aggregate.
- The Organization context owns the OrganizationRepository.

Per ARCH-0002:

- The Organization Domain is the sole owner of Organizations, Organization DNA, Organization Profiles, Organizational Hierarchy, Organizational Capabilities, Organization Health Records, and Organizational Metadata.
- No other Implementation Domain may modify these entities directly.

---

## Relationships with Other Bounded Contexts

Per TDS-0003, the Organization Unit collaborates primarily with:

| Organizational Unit | Collaboration Purpose |
|---------------------|----------------------|
| Governance | Strategic direction and organizational integrity |
| Mission Execution | Strategic direction and mission delivery |
| Workforce | Strategic direction and capability assignment |

Per TDS-0002, the Organization context publishes events including:

- OrganizationCreated;
- OrganizationUpdated;
- OrganizationArchived;
- CapabilityRegistered;
- CapabilityRetired;
- OrganizationHealthEvaluated.

The Organization context consumes events including:

- MissionCompleted;
- KnowledgePromoted;
- DecisionApproved;
- WorkforceCapabilityChanged.

Per ARCH-0002, the Organization Domain publishes events including:

- OrganizationCreated;
- OrganizationUpdated;
- OrganizationArchived;
- OrganizationHealthChanged;
- CapabilityRegistered;
- CapabilityRemoved;
- OrganizationDNAModified.

The Organization Domain consumes events including:

- MissionCompleted;
- KnowledgePromoted;
- DecisionApproved;
- WorkforceCapabilityChanged.

---

## Authority Traceability

| Concern | Authoritative Source |
|---------|----------------------|
| Organizational identity | RFC-0001, RFC-0004, TDS-0003 |
| Organizational structure | RFC-0004, TDS-0003 |
| Organizational topology | TDS-0003, ARCH-ORG-0001 |
| Organizational authority | TDS-0003, ARCH-ORG-0002 |
| Organizational governance | TDS-0003, ARCH-ORG-0003 |
| Organizational lifecycle | TDS-0003, ARCH-ORG-0004 |
| Organizational capability ownership | TDS-0003, ARCH-ORG-0005 |
| Domain ownership | TDS-0002 |
| Component ownership | ARCH-0002 |
| Architecture enforcement | ARCH-0003 |

---

## Explanatory Notice

This handbook is explanatory only. It is not architecture authority. It introduces no new organizational policies, responsibilities, authority, governance rules, or ownership. All authoritative definitions remain in the referenced RFCs, TDSs, and ARCH documents.