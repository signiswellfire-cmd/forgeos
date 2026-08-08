//! Workforce Infrastructure (Milestone 4.0).
//!
//! This crate provides the infrastructure implementation for the Workforce
//! bounded context, including repository implementations and event publishers.
//!
//! # Architecture Notes
//! - Layer: Infrastructure (TDS-0002, ARCH-0003)
//! - Depends on: Workforce Domain (TDS-0002)
//! - Implements: WorkforceRepository (ISP-0004)

pub mod repository;
pub mod event_publisher;