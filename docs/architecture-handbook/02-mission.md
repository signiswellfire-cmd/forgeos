# Architecture Handbook — Mission Bounded Context

**Document Type:** Explanatory Handbook (not architecture authority)

**Status:** Explanatory

---

## Purpose

The Mission bounded context owns organizational execution. A Mission represents an intentional piece of organizational work with a defined objective, lifecycle, ownership, and outcome. The Mission context determines **what the organization is attempting to accomplish**, independent of how that work is executed.

The Mission Execution Unit transforms organizational intent into executable missions.

---

## Responsibilities

Per TDS-0003, the Mission Execution Unit owns:

- mission ownership;
- mission planning;
- mission execution;
- mission completion;
- mission coordination.

Per TDS-0002, the Mission context owns:

- mission identity;
- mission lifecycle;
- mission planning;
- mission execution state;
- mission outcome.

Per ARCH-0002, the Mission Domain owns:

- Missions;
- Mission Plans;
- Mission Assignments;
- Mission Status;
- Mission History;
- Mission Outcomes;
- Mission Dependencies.

Per RFC-0021, the Mission Engine is the authoritative execution subsystem. Every organizational objective requiring coordinated execution shall be represented as a Mission.

---

## Key Concepts

| Concept | Meaning (per authority) |
|---------|--------------------------|
| Mission | A business objective pursued by the Organization. Missions define *why* work is performed. |
| Mission Ownership | Accountability for achieving a mission outcome. |
| Mission Owner | The single accountable owner of a mission. |
| Mission Lifecycle | The organizational progression of a mission through defined states. |
| Mission Engine | The execution orchestration subsystem of ForgeOS. |
| Mission Delegation | Transfer of execution authority without transferring mission ownership. |

---

## Lifecycle

Per TDS-0003, mission ownership persists throughout the mission lifecycle. Representative mission lifecycle states include:

- Defined;
- Planned;
- Approved;
- Active;
- Completed;
- Evaluated;
- Archived.

Per RFC-0021, the Mission Engine defines the following lifecycle:

- Proposed;
- Planned;
- Approved;
- Active;
- Validation;
- Knowledge Promotion;
- Completed;
- Archived.

Per ARCH-ORG-0004, the mission lifecycle visualizes the organizational progression of a mission. Lifecycle authority remains defined by TDS-0003.

---

## Ownership

Per TDS-0003:

- Every mission has exactly one accountable owner.
- Mission ownership persists throughout the mission lifecycle.
- Execution authority may be delegated while mission ownership remains unchanged.
- Delegation shall not transfer accountability, transfer mission ownership, or modify governance authority.
- Missions consume capabilities but never own them.

Per TDS-0002:

- The Mission aggregate is the authoritative root of the Mission bounded context.
- Mission ownership is singular.
- Mission lifecycle transitions are validated by the aggregate.
- Mission history is append-only.
- Mission outcome remains traceable.
- Only the Mission context may modify mission state.

Per ARCH-0002:

- Mission state transitions are controlled exclusively by the Mission Domain.
- Mission execution never modifies foreign aggregates directly.
- Mission ownership remains singular.
- Mission history is append-only.
- Mission completion is irreversible except through governed corrective processes.

---

## Relationships with Other Bounded Contexts

Per TDS-0003, the Mission Execution Unit collaborates primarily with:

| Organizational Unit | Collaboration Purpose |
|---------------------|----------------------|
| Organization | Strategic direction and mission delivery |
| Governance | Mission integrity and governance compliance |
| Workforce | Capability assignment and mission delivery |
| Knowledge | Organizational learning |

Per TDS-0002, the Mission context publishes events including:

- MissionCreated;
- MissionPlanned;
- MissionAssigned;
- MissionStarted;
- MissionPaused;
- MissionCompleted;
- MissionCancelled;
- MissionOutcomeRecorded.

The Mission context consumes events including:

- OrganizationUpdated;
- CapabilityRegistered;
- ProcessCompleted;
- DecisionApproved.

Per ARCH-0002, the Mission Domain publishes events including:

- MissionCreated;
- MissionPlanned;
- MissionAssigned;
- MissionStarted;
- MissionPaused;
- MissionCompleted;
- MissionCancelled;
- MissionOutcomeRecorded.

The Mission Domain consumes events including:

- OrganizationUpdated;
- CapabilityRegistered;
- ProfessionalAssigned;
- ProcessCompleted.

Per RFC-0021, the Mission Engine relates to:

- Executive Meetings (create, prioritize, redirect, or conclude Missions);
- the Forge Pipeline (implements the execution stages);
- Knowledge Promotion (Mission outputs become promotion candidates);
- the Knowledge Graph (each Mission becomes a graph node).

---

## Authority Traceability

| Concern | Authoritative Source |
|---------|----------------------|
| Mission concept | RFC-0001, RFC-0004, TDS-0003 |
| Mission ownership | TDS-0003, ARCH-ORG-0004 |
| Mission lifecycle | TDS-0003, ARCH-ORG-0004, RFC-0021 |
| Mission execution | RFC-0005, RFC-0021 |
| Mission Engine | RFC-0021 |
| Domain ownership | TDS-0002 |
| Component ownership | ARCH-0002 |
| Architecture enforcement | ARCH-0003 |

---

## Explanatory Notice

This handbook is explanatory only. It is not architecture authority. It introduces no new mission lifecycle stages, mission ownership, governance participation, delegation rules, or organizational responsibilities. All authoritative definitions remain in the referenced RFCs, TDSs, and ARCH documents.