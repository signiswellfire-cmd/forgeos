//! Manage Workforce Application Service (Milestone 4.0).
//!
//! This crate implements the application service for managing Workforce
//! aggregates. It orchestrates the workforce management workflow without
//! containing business rules (TDS-0002, ISP-0001).
//!
//! # Architecture Notes
//! - Layer: Application (TDS-0002, ARCH-0003)
//! - Depends on: Workforce Domain (TDS-0002)
//! - Depends on: Infrastructure (for repository and transactions)

pub mod service;
pub mod transaction;