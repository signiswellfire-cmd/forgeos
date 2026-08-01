# Next Session

Execute **Implementation Milestone 001.5 — Organization Domain Foundation**.

The scope is defined and approved in `docs/implementation/MILESTONE-001.5-ORGANIZATION-DOMAIN.md`. Implement the Organization Domain Foundation inside the `forgeos-organization-domain` crate:

* Organization aggregate
* Approved value objects
* Internal entities required by the approved creation semantics
* Domain-owned `OrganizationRepository` contract
* `OrganizationCreated` domain event
* Approved domain error model
* Deterministic domain tests

Each implementation item must trace to the approved RFC, TDS, TDR, Architecture Package, ISP, and milestone documents. The planning milestones MILESTONE-001.2 (Crate Boundary Plan) and MILESTONE-001.3 (Crate Initialization Plan) are approved. Do not create Infrastructure, Application, Platform, persistence, or IPC code in this milestone.

The `OrganizationId` generation mechanism is already approved in TDR-0006 (UUID v4 through a Domain-owned generator contract), and the `OrganizationType` representation is approved in the OrganizationType Decision (String-backed value object). Implement them in the Domain Foundation; do not decide them implicitly in source code.

Additional RFC expansion beyond the current approved RFC set is deferred until implementation experience requires new architectural decisions.

Do not invent missing technology decisions, choose a frontend framework or persistence library, or bypass architecture.
