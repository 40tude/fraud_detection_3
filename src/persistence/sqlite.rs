// src/persistence/sqlite.rs

use crate::domain::repository::TransactionRepository;
use crate::domain::transaction::Transaction;
use rusqlite::{Connection, Result, params};
use std::sync::Mutex;
use tracing::debug;

/// SQLite-based implementation of TransactionRepository
pub struct SQLiteTransactionRepo {
    conn: Mutex<Connection>,
}

impl SQLiteTransactionRepo {
    /// Initialize a new SQLiteTransactionRepo with a DB file or in-memory DB
    pub fn new(db_path: &str) -> Self {
        let conn = Connection::open(db_path).expect("Failed to open SQLite DB");

        // Create table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS transactions (
                id TEXT PRIMARY KEY,
                amount REAL NOT NULL,
                currency TEXT NOT NULL
            )",
            [],
        )
        .expect("Failed to create table");

        Self { conn: Mutex::new(conn) }
    }
}

impl TransactionRepository for SQLiteTransactionRepo {
    fn save(&self, tx: Transaction) {
        let conn = self.conn.lock().unwrap();
        conn.execute("INSERT OR REPLACE INTO transactions (id, amount, currency) VALUES (?1, ?2, ?3)", params![tx.id, tx.amount, tx.currency])
            .expect("Failed to insert transaction");

        debug!(tx_id = %tx.id, "Saved transaction to SQLite");
    }

    fn get(&self, id: &str) -> Option<Transaction> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, amount, currency FROM transactions WHERE id = ?1").ok()?;

        let mut rows = stmt.query(params![id]).ok()?;

        if let Some(row) = rows.next().ok().flatten() {
            let tx = Transaction {
                id: row.get(0).unwrap(),
                amount: row.get(1).unwrap(),
                currency: row.get(2).unwrap(),
            };
            Some(tx)
        } else {
            None
        }
    }
}
