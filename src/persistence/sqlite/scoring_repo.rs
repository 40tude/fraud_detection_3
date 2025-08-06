// src/persistence/sqlite/scoring_repo.rs

use crate::domain::repository::ScoringResultRepository;
use crate::domain::scoring::ScoringResult;
use rusqlite::{Connection, params};
use std::sync::Mutex;
use tracing::debug;

pub struct SQLiteScoringResultRepo {
    // conn: Connection, Not thread safe
    conn: Mutex<Connection>,
}

impl SQLiteScoringResultRepo {
    pub fn new(db_path: &str) -> Self {
        let conn = Connection::open(db_path).expect("Failed to open SQLite DB");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS scoring_results (
                tx_id TEXT PRIMARY KEY,
                score REAL NOT NULL,
                is_fraud INTEGER NOT NULL
            )",
            [],
        )
        .expect("Failed to create scoring_results table");

        Self { conn: Mutex::new(conn) }
    }
}

impl ScoringResultRepository for SQLiteScoringResultRepo {
    fn save(&self, result: ScoringResult) {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO scoring_results (tx_id, score, is_fraud) VALUES (?1, ?2, ?3)",
            params![result.id, result.score, result.is_fraud as i32],
        )
        .expect("Failed to insert scoring result");
        debug!(tx_id = %result.id, "Saved scoring to SQLite");
    }

    fn get(&self, id: &str) -> Option<ScoringResult> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, score, is_fraud FROM scoring_results WHERE id = ?1").ok()?;

        let mut rows = stmt.query(params![id]).ok()?;

        if let Some(row) = rows.next().ok().flatten() {
            let tx = ScoringResult {
                id: row.get(0).unwrap(),
                score: row.get(1).unwrap(),
                is_fraud: row.get::<_, i32>(2).unwrap() != 0,
            };
            Some(tx)
        } else {
            None
        }
    }
}
