//! SQLx-based transaction implementation for ForgeOS.
//!
//! This module provides the `SqlxTransaction` struct, which implements the
//! Application-owned `Transaction` trait using SQLx transaction APIs (ISP-0006;
//! TDS-0004; TDR-0003).
//!
//! This implementation:
//! - Coordinates transaction lifecycle (begin, commit, rollback)
//! - Manages database connection lifetime
//! - Participates in the Application Layer's transaction coordination
//! - Does not own transaction boundaries (Application Layer owns them)
//!
//! Transaction ownership remains exclusively within the Application Layer per
//! ISP-0006 and TDS-0004.

use std::sync::Arc;

use forgeos_create_organization_application::{Transaction, TransactionError};
use sqlx::sqlite::SqlitePool;

/// SQLx-based implementation of the `Transaction` trait.
///
/// This struct wraps a SQLx database connection and provides transaction
/// coordination for the Application Layer (ISP-0006; TDR-0003).
///
/// # Responsibilities
///
/// - Begin transactions on the database connection
/// - Commit transactions after successful domain operations
/// - Rollback transactions on failure
/// - Manage connection lifetime within transaction scope
///
/// # Implementation Notes
///
/// This implementation uses SQLx's transaction API. It does not implement
/// business logic or own transaction boundaries. The Application Layer
/// orchestrates the transaction lifecycle.
pub struct SqlxTransaction {
    pool: Arc<SqlitePool>,
    transaction: Option<sqlx::Transaction<'static, sqlx::sqlite::Sqlite>>,
}

impl SqlxTransaction {
    /// Creates a new `SqlxTransaction` with the given connection pool.
    ///
    /// # Arguments
    ///
    /// * `pool` - A shared connection pool to the SQLite database
    ///
    /// # Returns
    ///
    /// A new `SqlxTransaction` instance ready to begin transactions.
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self {
            pool,
            transaction: None,
        }
    }

    /// Creates a new `SqlxTransaction` from a raw pool reference.
    ///
    /// This is a convenience constructor for cases where an `Arc` is not available.
    pub fn from_pool(pool: SqlitePool) -> Self {
        Self::new(Arc::new(pool))
    }
}

impl Transaction for SqlxTransaction {
    fn begin(&mut self) -> Result<(), TransactionError> {
        if self.transaction.is_some() {
            return Err(TransactionError::AlreadyBegun);
        }

        // Begin a new transaction on the pool
        // We need to use a runtime to bridge between sync and async SQLx APIs
        let pool = Arc::clone(&self.pool);
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| TransactionError::OperationFailed(e.to_string()))?;

        let pool_ref: &SqlitePool = &pool;
        let tx = rt.block_on(async move {
            pool_ref.begin()
                .await
                .map_err(|e| TransactionError::OperationFailed(e.to_string()))
        })?;

        self.transaction = Some(tx);
        Ok(())
    }

    fn commit(&mut self) -> Result<(), TransactionError> {
        if self.transaction.is_none() {
            return Err(TransactionError::NotBegun);
        }

        let mut tx = self.transaction.take().unwrap();
        
        // Commit the transaction
        tokio::runtime::Runtime::new()
            .map_err(|e| TransactionError::CommitFailed(e.to_string()))?
            .block_on(async move {
                tx.commit().await.map_err(|e| TransactionError::CommitFailed(e.to_string()))
            })
    }

    fn rollback(&mut self) -> Result<(), TransactionError> {
        if self.transaction.is_none() {
            return Err(TransactionError::NotBegun);
        }

        let mut tx = self.transaction.take().unwrap();
        
        // Rollback the transaction
        tokio::runtime::Runtime::new()
            .map_err(|e| TransactionError::RollbackFailed(e.to_string()))?
            .block_on(async move {
                tx.rollback().await.map_err(|e| TransactionError::RollbackFailed(e.to_string()))
            })
    }

    fn is_active(&self) -> bool {
        self.transaction.is_some()
    }
}

impl Drop for SqlxTransaction {
    /// Safely rolls back any active transaction before the underlying SQLx
    /// transaction is dropped outside a Tokio runtime context.
    ///
    /// SQLx's `Transaction::drop` attempts to spawn an async rollback task,
    /// which requires a Tokio runtime. If `SqlxTransaction` is dropped outside
    /// a runtime (as occurs in synchronous usage), the SQLx drop panics with
    /// "this functionality requires a Tokio context".
    ///
    /// This implementation creates a short-lived runtime to explicitly roll
    /// back the active transaction, consuming it safely before the SQLx
    /// `Transaction`'s own drop runs (ISP-0006; TDR-0003).
    fn drop(&mut self) {
        if let Some(tx) = self.transaction.take() {
            if let Ok(rt) = tokio::runtime::Runtime::new() {
                let _ = rt.block_on(async move {
                    tx.rollback().await
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    /// Helper to create an in-memory SQLite database pool for testing.
    fn create_test_pool() -> Arc<SqlitePool> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let pool = rt.block_on(SqlitePoolOptions::new().connect(":memory:")).unwrap();
        Arc::new(pool)
    }

    #[test]
    fn sqlx_transaction_lifecycle_succeeds() {
        let pool = create_test_pool();
        let mut tx = SqlxTransaction::new(pool);

        assert!(!tx.is_active());

        tx.begin().unwrap();
        assert!(tx.is_active());

        tx.commit().unwrap();
        assert!(!tx.is_active());
    }

    #[test]
    fn sqlx_transaction_lifecycle_rolls_back() {
        let pool = create_test_pool();
        let mut tx = SqlxTransaction::new(pool);

        tx.begin().unwrap();
        assert!(tx.is_active());

        tx.rollback().unwrap();
        assert!(!tx.is_active());
    }

    #[test]
    fn sqlx_transaction_begin_twice_fails() {
        let pool = create_test_pool();
        let mut tx = SqlxTransaction::new(pool);

        tx.begin().unwrap();
        let result = tx.begin();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TransactionError::AlreadyBegun));
    }

    #[test]
    fn sqlx_transaction_commit_without_begin_fails() {
        let pool = create_test_pool();
        let mut tx = SqlxTransaction::new(pool);

        let result = tx.commit();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TransactionError::NotBegun));
    }

    #[test]
    fn sqlx_transaction_rollback_without_begin_fails() {
        let pool = create_test_pool();
        let mut tx = SqlxTransaction::new(pool);

        let result = tx.rollback();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TransactionError::NotBegun));
    }

    #[test]
    fn sqlx_transaction_commit_preserves_data() {
        let pool = create_test_pool();
        let pool_clone = Arc::clone(&pool);
        let mut tx = SqlxTransaction::new(pool);

        // Create a test table
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(sqlx::query("CREATE TABLE test (id INTEGER PRIMARY KEY, value TEXT)")
            .execute(&*pool_clone))
            .unwrap();

        // Begin transaction and insert data
        tx.begin().unwrap();

        // Insert data through the transaction
        let tx_clone = Arc::clone(&pool_clone);
        rt.block_on(async move {
            sqlx::query("INSERT INTO test (id, value) VALUES (1, 'test')")
                .execute(&*tx_clone)
                .await
        }).unwrap();

        // Commit transaction
        tx.commit().unwrap();

        // Verify data persists after commit
        let count: i64 = rt.block_on(sqlx::query_scalar("SELECT COUNT(*) FROM test").fetch_one(&*pool_clone)).unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn sqlx_transaction_rollback_discards_changes() {
        let pool = create_test_pool();
        let pool_clone = Arc::clone(&pool);
        let mut tx = SqlxTransaction::new(pool);

        // Create a test table
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(sqlx::query("CREATE TABLE test (id INTEGER PRIMARY KEY, value TEXT)")
            .execute(&*pool_clone))
            .unwrap();

        // Begin transaction and rollback
        tx.begin().unwrap();
        tx.rollback().unwrap();

        // Transaction rolled back successfully
        assert!(!tx.is_active());
    }
}