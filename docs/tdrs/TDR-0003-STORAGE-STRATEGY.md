# TDR-0003 — Storage Strategy

**TDR Number:** TDR-0003  
**Title:** Storage Strategy  
**Status:** Approved  
**Version:** 1.0.0  
**Related TDS:** TDS-0001 — System Architecture; TDS-0002 — Domain Model; TDS-0004 — Application Model  
**Related TDRs:** TDR-0001 — Programming Language; TDR-0002 — Desktop Framework

---

# Purpose

This Technology Decision Record selects the Milestone 1 local storage implementation for the Create Organization vertical slice. It resolves the database library, migration, and transaction details while preserving Domain-owned repository contracts and Infrastructure-owned persistence.

---

# Context

ForgeOS Phase 1 requires SQLite storage and local-first operation. TDS-0001 and TDS-0002 require persistence behind Domain-owned repositories, prohibit Domain dependencies on databases, and require each aggregate to have one persistence owner. TDS-0004 assigns transaction boundaries to Application Services.

The selected approach must work with Rust and Tauri 2.x, support deterministic local setup, preserve replaceable Infrastructure adapters, and support versioned schema evolution without placing persistence concerns in the Organization aggregate.

---

# Decision

ForgeOS Milestone 1 shall use **SQLite** as its local database through **SQLx**, configured only with the SQLite driver and the Tokio runtime integration required by the Tauri backend.

SQLx is an Infrastructure-only dependency. No Domain, Application, or Presentation component may expose SQLx types or SQL statements.

## Migration strategy

Schema changes shall be forward-only, ordered SQL migrations maintained with SQLx's migration support. Migrations are versioned implementation artifacts owned by Infrastructure and executed during application bootstrap before the Create Organization command is made available. The migration history stored with the database is the authoritative record of which schema version has been applied.

Migrations may create only tables owned by the Organization persistence implementation for this slice. Shared tables across domains are prohibited. Destructive, corrective, or data-repair migration policies are deferred until a later decision is required.

## Transaction strategy

The Create Organization Application Service owns the logical transaction boundary. Its Infrastructure transaction coordinator opens one SQLite transaction, makes the Organization repository participate in that transaction, and commits only after the aggregate persistence operation succeeds. On any domain, repository, or storage failure, the transaction rolls back.

`OrganizationCreated` is dispatched only after successful commit. Domain state is not changed by rollback handling, and transaction objects never enter the Domain model.

## Concurrency strategy

The singleton Organization constraint is enforced inside the creation transaction. The repository's optimistic-concurrency contract is preserved using the approved `OrganizationVersion` value; Milestone 1 does not add a cross-domain locking mechanism or distributed transaction.

---

# Rationale

SQLite satisfies the approved local-first Phase 1 persistence requirement without a server dependency. SQLx provides a Rust SQLite driver, asynchronous pool and transaction support, and supported migration tooling while allowing Infrastructure to use explicit SQL and map records to Domain objects. This keeps the repository interface independent of the selected technology and avoids coupling the aggregate to an ORM model.

Forward-only migrations preserve repository knowledge and make local schema evolution reproducible. An Application-owned logical transaction with Infrastructure participation implements the transaction ownership defined by TDS-0004 and ISP-0006.

---

# Architectural Alignment

* **TDS-0001:** Infrastructure performs persistence; Domain remains database-independent.
* **TDS-0002:** `OrganizationRepository` remains Domain-owned, persists one aggregate, performs existence checks and optimistic concurrency, and hides persistence technology.
* **TDS-0004:** the Application Service owns the use-case transaction boundary.
* **ARCH-0003:** Infrastructure implements rather than defines business contracts; the selected library is not permitted to leak upward.
* **ISP-0004 and ISP-0006:** repositories participate in, but do not own, transactions; mappings and technical errors remain in Infrastructure.

---

# Alternatives Considered

## rusqlite

Rejected for Milestone 1 because it is a synchronous SQLite binding and would require a separate blocking-execution policy to fit the asynchronous Tauri backend. SQLx provides the needed asynchronous driver, pooling, transactions, and migration support in one approved Infrastructure dependency.

## Diesel

Rejected because its ORM/query DSL would introduce a larger persistence abstraction into the first slice. The slice requires explicit Infrastructure mapping and a small repository implementation, not an additional domain-shaped persistence model.

## SeaORM

Rejected because it introduces an active-record ORM layer on top of SQLx. The additional abstraction is not necessary for one aggregate repository and risks obscuring the Domain-to-record mapping required by the architecture.

## Filesystem serialization

Rejected because it does not provide the approved SQLite storage direction, migration history, or the repository transaction behavior required by Milestone 1.

## Remote database

Rejected because ForgeOS Phase 1 is local-first and cloud services are optional rather than runtime requirements.

---

# Consequences

Positive consequences:

* local, serverless persistence aligned with Phase 1;
* explicit and reproducible schema evolution;
* transaction participation without exposing database concerns to the Domain;
* SQLx provides SQLite, migrations, and transaction support within a Rust-compatible library.

Trade-offs:

* Infrastructure must maintain explicit SQL mappings and migration artifacts;
* SQLite concurrency is intentionally limited to the local application model;
* SQLx and its Tokio runtime integration become Infrastructure implementation dependencies.

These trade-offs are accepted because they preserve the approved layer boundaries and local-first runtime model.

---

# Future Considerations

Future decisions may define database location, backup/export, encryption-at-rest, retention, migrations requiring data transformation, database diagnostics, and storage adapters for cloud or alternative local providers. Those concerns must not alter the Domain repository contract.

---

# References

* RFC-0004 — Organization Model
* TDS-0001 — System Architecture
* TDS-0002 — Domain Model
* TDS-0004 — Application Model
* TDR-0001 — Programming Language
* TDR-0002 — Desktop Framework
* `docs/architecture/persistence-model.md`
* `docs/architecture/architecture-enforcement-specification.md`
* ISP-0004 — Repository Pattern
* ISP-0006 — Transaction Pattern
* [SQLx documentation](https://docs.rs/sqlx/latest/sqlx/)
