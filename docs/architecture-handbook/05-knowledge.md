# Architecture Handbook — Knowledge Bounded Context

**Document Type:** Explanatory Handbook (not architecture authority)

**Status:** Explanatory

---

## Purpose

The Knowledge bounded context owns organizational knowledge. Knowledge represents validated organizational understanding rather than transient execution state. The Knowledge context governs Knowledge Objects, Knowledge Relationships, Blueprints, Knowledge Promotion, and Organizational Learning.

The Knowledge Unit maintains validated organizational knowledge.

---

## Responsibilities

Per TDS-0003, the Knowledge Unit owns:

- knowledge promotion;
- blueprint publication;
- knowledge classification;
- organizational learning;
- knowledge stewardship.

Per TDS-0002, the Knowledge context owns:

- knowledge identity;
- lifecycle;
- relationships;
- classification;
- provenance.

Per ARCH-0002, the Knowledge Domain owns:

- Knowledge Objects;
- Knowledge Relationships;
- Blueprint Metadata;
- Knowledge Categories;
- Knowledge Lifecycle State;
- Knowledge Tags;
- Knowledge Provenance.

Per RFC-0002, Knowledge Objects are the canonical representation of permanent organizational knowledge. Knowledge is never owned by an individual contributor; knowledge belongs to the Organization.

Per RFC-0003, the Knowledge Graph is the organizational memory system that connects every permanent knowledge object, engineering artifact, architectural decision, professional, mission, process, capability, and blueprint into a single navigable network.

Per RFC-0009, Knowledge Promotion is the mechanism through which temporary engineering outputs become permanent organizational assets.

---

## Key Concepts

| Concept | Meaning (per authority) |
|---------|--------------------------|
| Knowledge | Validated organizational understanding that has been promoted for long-term organizational use. |
| Knowledge Object | The smallest permanent unit of organizational knowledge. |
| Knowledge Graph | The organizational memory system connecting knowledge objects into a navigable network. |
| Blueprint | A reusable organizational pattern promoted into validated knowledge. |
| Knowledge Promotion | The controlled process of promoting validated implementation experience into permanent organizational knowledge. |
| Knowledge Lifecycle | The progression of knowledge through Draft, Review, Approved, Deprecated, and Archived states. |
| Knowledge Relationship | A typed connection between knowledge objects (e.g., derives_from, references, depends_on, validates, supersedes). |

---

## Lifecycle

Per RFC-0002, Knowledge Objects progress through:

1. Draft (initial engineering understanding, not yet validated);
2. Review (undergoing technical review);
3. Approved (becomes authoritative, part of permanent organizational memory);
4. Deprecated (retained for historical reference but should no longer guide engineering decisions);
5. Archived (preserved but no longer expected to evolve).

Per RFC-0009, Knowledge Promotion follows:

1. Candidate (potential knowledge identified);
2. Evaluation (evaluated for accuracy, general applicability, architectural consistency, organizational value, long-term usefulness);
3. Validation (subject matter experts validate engineering quality);
4. Promotion (approved knowledge becomes a permanent Knowledge Object);
5. Maintenance (promoted knowledge evolves over time);
6. Retirement (knowledge transitions to deprecated or archived state).

Per TDS-0002, the Knowledge aggregate follows the general aggregate lifecycle:

- Created;
- Initialized;
- Active;
- Modified;
- Archived.

---

## Ownership

Per TDS-0003:

- The Knowledge Unit owns promoted organizational knowledge.
- Knowledge publication follows governance requirements.
- Every organizational responsibility has exactly one owner.
- Knowledge belongs to the Organization, not to individual contributors.
- Ownership shall never be implied by implementation.

Per TDS-0002:

- The Knowledge aggregate is the authoritative root of the Knowledge bounded context.
- Knowledge ownership is singular.
- Provenance is immutable.
- Relationships remain explicitly typed.
- Blueprint publication preserves lineage.
- Knowledge promotion never destroys historical versions.
- Knowledge aggregates never modify foreign aggregates.

Per ARCH-0002:

- Knowledge ownership is singular.
- Knowledge provenance is immutable.
- Blueprint publication preserves historical lineage.
- Knowledge relationships remain explicitly typed.
- Knowledge promotion never destroys historical versions.

Per RFC-0002:

- Knowledge belongs to the Organization.
- Professionals may create, review, refine, or promote Knowledge Objects, but ownership remains institutional.
- Knowledge Objects should be uniquely identifiable, versioned, reviewable, explainable, reusable, and independently referenceable.

Per RFC-0003:

- Every permanent Knowledge Object may participate in one or more graph relationships.
- Relationships themselves become permanent engineering knowledge.
- The Knowledge Graph is an organizational asset.

---

## Relationships with Other Bounded Contexts

Per TDS-0003, the Knowledge Unit collaborates primarily with:

| Organizational Unit | Collaboration Purpose |
|---------------------|----------------------|
| Mission Execution | Organizational learning and knowledge capture |
| Governance | Knowledge approval and organizational learning |
| Memory | Institutional preservation and organizational traceability |

Per TDS-0002, the Knowledge context publishes events including:

- KnowledgeCreated;
- KnowledgeUpdated;
- KnowledgePromoted;
- KnowledgeArchived;
- BlueprintPublished;
- RelationshipEstablished.

The Knowledge context consumes events including:

- DecisionApproved;
- MissionCompleted;
- ProcessCompleted;
- MemoryInstitutionalized.

Per ARCH-0002, the Knowledge Domain publishes events including:

- KnowledgeCreated;
- KnowledgeUpdated;
- KnowledgePromoted;
- KnowledgeArchived;
- BlueprintPublished;
- KnowledgeRelationshipCreated.

The Knowledge Domain consumes events including:

- DecisionApproved;
- MissionCompleted;
- ProcessCompleted;
- LearningCompleted.

Per RFC-0002, Knowledge Objects may reference one another through relationships including:

- derives_from;
- references;
- depends_on;
- validates;
- supersedes;
- produces;
- promotes;
- implements;
- belongs_to;
- assigned_to;
- uses;
- enables.

Per RFC-0009, Knowledge Promotion occurs after Validation within the Forge Pipeline. Promotion is the transition from execution to organizational learning.

---

## Authority Traceability

| Concern | Authoritative Source |
|---------|----------------------|
| Knowledge concept | RFC-0001, RFC-0002, RFC-0004, TDS-0003 |
| Knowledge responsibilities | TDS-0003, ARCH-ORG-0001 |
| Knowledge authority | TDS-0003, ARCH-ORG-0002 |
| Knowledge Model | RFC-0002 |
| Knowledge Graph | RFC-0003 |
| Knowledge Promotion | RFC-0009 |
| Knowledge Lifecycle | RFC-0002 |
| Blueprint management | RFC-0002, RFC-0010 |
| Domain ownership | TDS-0002 |
| Component ownership | ARCH-0002 |
| Architecture enforcement | ARCH-0003 |

---

## Explanatory Notice

This handbook is explanatory only. It is not architecture authority. It introduces no new knowledge promotion, blueprint stewardship, knowledge classification, organizational learning, or knowledge stewardship responsibilities. All authoritative definitions remain in the referenced RFCs, TDSs, and ARCH documents.