# Architecture Handbook — Process Bounded Context

**Document Type:** Explanatory Handbook (not architecture authority)

**Status:** Explanatory

---

## Purpose

The Process bounded context owns organizational workflow. Processes define repeatable methods for executing work independent of the specific Mission that invokes them. A Process describes **how** work is performed. A Mission determines **why** the work exists.

The Process Engine transforms organizational standards into repeatable operational behavior.

---

## Responsibilities

Per TDS-0003, processes are organizational capabilities that define repeatable organizational behavior. Processes describe how work is performed. Automation implements processes but does not replace them.

Per TDS-0002, the Process context owns:

- process definition;
- process lifecycle;
- process version;
- execution state;
- execution history.

Per ARCH-0002, the Process Domain owns:

- Process Definitions;
- Process Versions;
- Process Instances;
- Process Execution State;
- Process History;
- Process Metrics.

Per RFC-0022, the Process Engine is the authoritative execution framework for all repeatable organizational processes. Processes become governed organizational assets that may evolve independently from implementation technologies.

---

## Key Concepts

| Concept | Meaning (per authority) |
|---------|--------------------------|
| Process | A repeatable organizational behavior that transforms defined inputs into governed outputs. |
| Process Engine | The subsystem responsible for executing, governing, monitoring, and evolving organizational processes. |
| Process Definition | The organizational specification of a repeatable workflow. |
| Process Instance | A specific execution of a process definition. |
| Process Lifecycle | The progression of a process through Proposed, Designed, Approved, Active, Improved, Deprecated, and Archived states. |
| Activity | A discrete unit of organizational work within a process. |
| Decision Point | A point in a process where organizational conditions are evaluated to determine execution paths. |
| Validation | The organizational verification that process outputs meet defined requirements. |

---

## Lifecycle

Per RFC-0022, every Process progresses through:

1. Proposed (initial organizational need identified);
2. Designed (activities, governance, and expected outcomes defined);
3. Approved (process validated through organizational governance);
4. Active (process available for organizational execution);
5. Improved (process updated through organizational learning, historical versions remain traceable);
6. Deprecated (process scheduled for replacement);
7. Archived (historical reference retained).

Per TDS-0002, the Process aggregate follows the general aggregate lifecycle:

- Created;
- Initialized;
- Active;
- Modified;
- Archived.

---

## Ownership

Per TDS-0003:

- Every organizational responsibility has exactly one owner.
- Ownership shall never be implied by implementation.
- Processes describe how work is performed.
- Automation implements processes but does not replace them.

Per TDS-0002:

- The Process aggregate is the authoritative root of the Process bounded context.
- Process definitions are versioned.
- Running process instances preserve version integrity.
- Processes never directly modify foreign aggregates.
- Process execution is deterministic for identical inputs unless explicitly configured otherwise.
- Mission ownership remains outside the Process Domain.
- Process aggregates never modify foreign aggregates.

Per ARCH-0002:

- Process definitions are versioned.
- Running process instances preserve version integrity.
- Processes never directly modify foreign aggregates.
- Process execution is deterministic for identical inputs unless explicitly configured otherwise.
- Mission ownership remains outside the Process Domain.

Per RFC-0022:

- Every Process has one organizational owner, one or more stewards, participating Professionals, and defined governance authority.
- Ownership remains organizational rather than personal.
- Process modification requires documented rationale, impact assessment, organizational review, and version management.
- Process evolution should be deliberate.

---

## Relationships with Other Bounded Contexts

Per TDS-0003, processes are organizational capabilities that support mission execution. Missions invoke processes. Processes remain reusable across many missions.

Per TDS-0002, the Process context publishes events including:

- ProcessDefined;
- ProcessVersionPublished;
- ProcessStarted;
- ProcessSuspended;
- ProcessResumed;
- ProcessCompleted;
- ProcessFailed.

The Process context consumes events including:

- MissionStarted;
- MissionCancelled;
- OrganizationUpdated.

Per ARCH-0002, the Process Domain publishes events including:

- ProcessDefined;
- ProcessVersionPublished;
- ProcessStarted;
- ProcessSuspended;
- ProcessResumed;
- ProcessCompleted;
- ProcessFailed.

The Process Domain consumes events including:

- MissionStarted;
- MissionCancelled;
- OrganizationUpdated.

Per RFC-0022, the Process Engine relates to:

- Missions (processes support mission execution, missions invoke processes);
- Blueprints (blueprints may instantiate predefined processes);
- Engineering Standards (engineering standards constrain process behavior, processes implement standards);
- the Knowledge Graph (processes become graph nodes connected to missions, professionals, standards, capabilities, decisions, blueprints, and knowledge objects).

---

## Authority Traceability

| Concern | Authoritative Source |
|---------|----------------------|
| Process concept | RFC-0001, RFC-0004, RFC-0005, TDS-0003 |
| Process responsibilities | TDS-0003 |
| Process authority | TDS-0003 |
| Process Engine | RFC-0022 |
| Process lifecycle | RFC-0022 |
| Process ownership | TDS-0002, RFC-0022 |
| Domain ownership | TDS-0002 |
| Component ownership | ARCH-0002 |
| Architecture enforcement | ARCH-0003 |

---

## Explanatory Notice

This handbook is explanatory only. It is not architecture authority. It introduces no new process definitions, process execution, process governance, process monitoring, or process evolution responsibilities. All authoritative definitions remain in the referenced RFCs, TDSs, and ARCH documents.