//! Create Governance Application Service (Milestone 3.0).
//!
//! This crate implements the application service for creating Governance
//! aggregates. It orchestrates the creation workflow without containing
//! business rules (TDS-0002, ISP-0001).
//!
//! # Architecture Notes
//! - Layer: Application (TDS-0002, ARCH-0003)
//! - Depends on: Governance Domain (TDS-0002)
//! - Depends on: Infrastructure (for repository and transactions)

pub mod service;
pub mod transaction;