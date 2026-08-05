//! Transaction coordination abstraction for ForgeOS Application Services.
//!
//! This module defines the `Transaction` trait, which standardizes transaction
//! lifecycle management across ForgeOS Application Services (ISP-0006; TDS-0004).
//!
//! The trait is owned by the Application Layer and defines the contract for
//! transaction coordination. Infrastructure implementations provide concrete
//! transaction mechanisms (e.g., SQLx transactions).
//!
//! Transaction ownership remains exclusively within the Application Layer.
//! Repositories participate in transactions but do not own them (ISP-0006).

use thiserror::Error;

/// Errors that can occur during transaction coordination.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum TransactionError {
    /// Transaction has already been begun.
    #[error("Transaction has already been begun")]
    AlreadyBegun,

    /// Transaction has not been begun.
    #[error("Transaction has not been begun")]
    NotBegun,

    /// Transaction commit failed.
    #[error("Transaction commit failed: {0}")]
    CommitFailed(String),

    /// Transaction rollback failed.
    #[error("Transaction rollback failed: {0}")]
    RollbackFailed(String),

    /// Transaction operation failed.
    #[error("Transaction operation failed: {0}")]
    OperationFailed(String),
}

/// Trait defining the canonical transaction lifecycle for ForgeOS Application Services.
///
/// This trait is owned by the Application Layer and defines the contract for
/// transaction coordination (TDS-0004; ISP-0006). Infrastructure implementations
/// provide concrete transaction mechanisms.
///
/// # Transaction Lifecycle
///
/// Every transaction follows the canonical lifecycle:
/// 1. `begin()` — Begin the transaction boundary
/// 2. Execute domain operations through repositories
/// 3. `commit()` — Commit on success OR `rollback()` — Rollback on failure
///
/// # Responsibilities
///
/// Application Services are responsible for:
/// - Calling `begin()` before domain operations
/// - Calling `commit()` after successful execution
/// - Calling `rollback()` on failure
/// - Ensuring commit and rollback are mutually exclusive (ISP-0006)
///
/// # Implementation Notes
///
/// Implementations should:
/// - Be short-lived (ISP-0006)
/// - Preserve aggregate consistency (ISP-0006)
/// - Support deterministic lifecycle (ISP-0006)
/// - Hide infrastructure technology details (ISP-0006)
pub trait Transaction {
    /// Begins a new transaction boundary.
    ///
    /// This method establishes the transaction context within which all
    /// subsequent domain operations will execute. It must be called before
    /// any domain operations that require transactional consistency.
    ///
    /// # Errors
    ///
    /// Returns `TransactionError::AlreadyBegun` if a transaction is already active.
    /// Returns `TransactionError::OperationFailed` if the transaction cannot be begun.
    fn begin(&mut self) -> Result<(), TransactionError>;

    /// Commits the current transaction.
    ///
    /// This method finalizes all operations performed within the transaction
    /// boundary. It should be called only after all domain operations have
    /// completed successfully.
    ///
    /// # Errors
    ///
    /// Returns `TransactionError::NotBegun` if no transaction is active.
    /// Returns `TransactionError::CommitFailed` if the commit operation fails.
    fn commit(&mut self) -> Result<(), TransactionError>;

    /// Rolls back the current transaction.
    ///
    /// This method discards all operations performed within the transaction
    /// boundary. It should be called when domain operations fail or when
    /// the application decides not to commit.
    ///
    /// # Errors
    ///
    /// Returns `TransactionError::NotBegun` if no transaction is active.
    /// Returns `TransactionError::RollbackFailed` if the rollback operation fails.
    fn rollback(&mut self) -> Result<(), TransactionError>;

    /// Checks whether a transaction is currently active.
    ///
    /// This method is optional but useful for testing and debugging.
    /// The default implementation returns `false`.
    ///
    /// # Returns
    ///
    /// `true` if a transaction is active, `false` otherwise.
    fn is_active(&self) -> bool {
        false
    }
}

/// Mock transaction implementation for testing Application Services.
///
/// This mock tracks transaction lifecycle calls and can simulate
/// success or failure scenarios. It is publicly available for testing
/// across crates while maintaining the trait-based architecture.
#[derive(Debug, Clone, Default)]
pub struct MockTransaction {
    begin_called: bool,
    commit_called: bool,
    rollback_called: bool,
    should_fail_commit: bool,
    should_fail_rollback: bool,
}

impl MockTransaction {
    /// Creates a new mock transaction.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether commit should fail.
    pub fn with_commit_failure(mut self, should_fail: bool) -> Self {
        self.should_fail_commit = should_fail;
        self
    }

    /// Sets whether rollback should fail.
    pub fn with_rollback_failure(mut self, should_fail: bool) -> Self {
        self.should_fail_rollback = should_fail;
        self
    }

    /// Checks if begin was called.
    pub fn begin_was_called(&self) -> bool {
        self.begin_called
    }

    /// Checks if commit was called.
    pub fn commit_was_called(&self) -> bool {
        self.commit_called
    }

    /// Checks if rollback was called.
    pub fn rollback_was_called(&self) -> bool {
        self.rollback_called
    }
}

impl Transaction for MockTransaction {
    fn begin(&mut self) -> Result<(), TransactionError> {
        if self.begin_called {
            return Err(TransactionError::AlreadyBegun);
        }
        self.begin_called = true;
        Ok(())
    }

    fn commit(&mut self) -> Result<(), TransactionError> {
        if !self.begin_called {
            return Err(TransactionError::NotBegun);
        }
        if self.should_fail_commit {
            return Err(TransactionError::CommitFailed(
                "Mock commit failure".to_string(),
            ));
        }
        self.commit_called = true;
        Ok(())
    }

    fn rollback(&mut self) -> Result<(), TransactionError> {
        if !self.begin_called {
            return Err(TransactionError::NotBegun);
        }
        if self.should_fail_rollback {
            return Err(TransactionError::RollbackFailed(
                "Mock rollback failure".to_string(),
            ));
        }
        self.rollback_called = true;
        Ok(())
    }

    fn is_active(&self) -> bool {
        self.begin_called && !self.commit_called && !self.rollback_called
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_transaction_lifecycle_succeeds() {
        let mut tx = MockTransaction::new();
        assert!(!tx.is_active());

        tx.begin().unwrap();
        assert!(tx.is_active());
        assert!(tx.begin_was_called());

        tx.commit().unwrap();
        assert!(!tx.is_active());
        assert!(tx.commit_was_called());
        assert!(!tx.rollback_was_called());
    }

    #[test]
    fn mock_transaction_lifecycle_rolls_back() {
        let mut tx = MockTransaction::new();
        tx.begin().unwrap();
        assert!(tx.is_active());

        tx.rollback().unwrap();
        assert!(!tx.is_active());
        assert!(!tx.commit_was_called());
        assert!(tx.rollback_was_called());
    }

    #[test]
    fn mock_transaction_begin_twice_fails() {
        let mut tx = MockTransaction::new();
        tx.begin().unwrap();
        let result = tx.begin();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TransactionError::AlreadyBegun));
    }

    #[test]
    fn mock_transaction_commit_without_begin_fails() {
        let mut tx = MockTransaction::new();
        let result = tx.commit();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TransactionError::NotBegun));
    }

    #[test]
    fn mock_transaction_rollback_without_begin_fails() {
        let mut tx = MockTransaction::new();
        let result = tx.rollback();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TransactionError::NotBegun));
    }

    #[test]
    fn mock_transaction_commit_failure() {
        let mut tx = MockTransaction::new().with_commit_failure(true);
        tx.begin().unwrap();
        let result = tx.commit();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TransactionError::CommitFailed(_)
        ));
    }

    #[test]
    fn mock_transaction_rollback_failure() {
        let mut tx = MockTransaction::new().with_rollback_failure(true);
        tx.begin().unwrap();
        let result = tx.rollback();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TransactionError::RollbackFailed(_)
        ));
    }
}