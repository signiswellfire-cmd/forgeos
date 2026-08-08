# Architecture Handbook — Workforce Bounded Context

**Document Type:** Explanatory Handbook (not architecture authority)

**Status:** Explanatory

---

## Purpose

The Workforce bounded context owns organizational capability. It governs professionals, teams, competencies, skills, and organizational capacity. The Workforce context describes **who performs work** rather than **what work is performed**.

The Workforce Unit develops and maintains organizational capability.

---

## Responsibilities

Per TDS-0003, the Workforce Unit owns:

- workforce capability;
- competency management;
- professional development;
- capability assignment;
- organizational capacity.

Per TDS-0002, the Workforce context owns:

- workforce identity;
- professional records;
- capability assignments;
- competency evaluations;
- team relationships.

Per ARCH-0002, the Workforce Domain owns:

- Professionals;
- Teams;
- Skills;
- Competencies;
- Capability Assignments;
- Workforce Metadata;
- Team Memberships.

Per RFC-0015, the Digital Workforce Framework defines how digital Professionals are created, governed, assigned, evaluated, evolved, and coordinated within a ForgeOS Organization. Professionals are organizational roles rather than AI agents.

---

## Key Concepts

| Concept | Meaning (per authority) |
|---------|--------------------------|
| Workforce | The organizational capability composed of professionals, competencies, teams, and skills. |
| Professional | A permanent organizational responsibility that may be fulfilled by a human, local AI, cloud AI, or hybrid execution model. |
| Team | A temporary collection of Professionals assembled around Missions. |
| Competency | A measure of professional capability. |
| Skill | A specific professional capability. |
| Capability Assignment | The assignment of capabilities to professionals or teams. |
| Professional Lifecycle | The progression of a Professional through definition, activation, assignment, mission execution, evaluation, capability improvement, and organizational evolution. |

---

## Lifecycle

Per RFC-0015, Professionals progress through:

1. Definition;
2. Activation;
3. Assignment;
4. Mission Execution;
5. Evaluation;
6. Capability Improvement;
7. Organizational Evolution.

The lifecycle repeats throughout the existence of the Organization.

Per TDS-0002, the Workforce aggregate follows the general aggregate lifecycle:

- Created;
- Initialized;
- Active;
- Modified;
- Archived.

---

## Ownership

Per TDS-0003:

- The Workforce Unit owns workforce capability decisions.
- Capability ownership remains independent from mission ownership.
- Every organizational responsibility has exactly one owner.
- Ownership shall never be implied by implementation.

Per TDS-0002:

- The Workforce aggregate is the authoritative root of the Workforce bounded context.
- Workforce identity is singular.
- Team membership remains internally consistent.
- Competency history is append-only.
- Capability ownership remains explicit.
- Workforce aggregates never modify foreign aggregates.

Per ARCH-0002:

- Professional identity is owned exclusively by this domain.
- Team membership is governed only by this domain.
- Competency history is append-only.
- Capability ownership remains explicit.
- Workforce state shall never be modified directly by foreign domains.

Per RFC-0015:

- Professionals are organizational roles.
- Execution technologies may change without altering the Professional.
- Responsibilities are permanent.
- Implementation is replaceable.
- Professionals operate within organizational governance.

---

## Relationships with Other Bounded Contexts

Per TDS-0003, the Workforce Unit collaborates primarily with:

| Organizational Unit | Collaboration Purpose |
|---------------------|----------------------|
| Organization | Strategic direction and capability development |
| Mission Execution | Capability assignment and mission delivery |
| Governance | Capability governance and competency integrity |
| Knowledge | Organizational learning and capability improvement |

Per TDS-0002, the Workforce context publishes events including:

- ProfessionalRegistered;
- TeamCreated;
- CompetencyEvaluated;
- CapabilityAssigned;
- WorkforceUpdated.

The Workforce context consumes events including:

- MissionAssigned;
- LearningCompleted;
- OrganizationUpdated.

Per ARCH-0002, the Workforce Domain publishes events including:

- ProfessionalCreated;
- ProfessionalUpdated;
- TeamCreated;
- TeamMembershipChanged;
- SkillRegistered;
- CompetencyEvaluated;
- CapabilityAssigned.

The Workforce Domain consumes events including:

- MissionAssigned;
- LearningCompleted;
- OrganizationUpdated.

Per RFC-0015, the Digital Workforce Framework relates to:

- Executive Meetings (coordinate Professionals);
- Missions (Professionals execute approved organizational decisions);
- Capabilities (Professionals contribute to Capabilities);
- Knowledge (Professionals reason from organizational knowledge).

---

## Authority Traceability

| Concern | Authoritative Source |
|---------|----------------------|
| Workforce concept | RFC-0001, RFC-0004, TDS-0003 |
| Workforce responsibilities | TDS-0003, ARCH-ORG-0001 |
| Workforce authority | TDS-0003, ARCH-ORG-0002 |
| Digital Workforce Framework | RFC-0015 |
| Professional lifecycle | RFC-0015 |
| Team formation | RFC-0015, RFC-0029 |
| Competency management | RFC-0028 |
| Domain ownership | TDS-0002 |
| Component ownership | ARCH-0002 |
| Architecture enforcement | ARCH-0003 |

---

## Explanatory Notice

This handbook is explanatory only. It is not architecture authority. It introduces no new workforce capabilities, competency management, professional development, capability assignment, or organizational responsibilities. All authoritative definitions remain in the referenced RFCs, TDSs, and ARCH documents.