//! Governance Infrastructure (Milestone 3.0).
//!
//! This crate provides the infrastructure implementation for the Governance
//! bounded context, including repository implementations and event publishers.
//!
//! # Architecture Notes
//! - Layer: Infrastructure (TDS-0002, ARCH-0003)
//! - Depends on: Governance Domain (TDS-0002)
//! - Implements: GovernanceRepository (ISP-0004)

pub mod repository;
pub mod event_publisher;