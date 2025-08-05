// src/persistence/in_memory.rs

use std::collections::HashMap;
use std::sync::Mutex;

use crate::domain::repository::TransactionRepository;
use crate::domain::transaction::Transaction;

pub struct InMemoryTransactionRepo {
    store: Mutex<HashMap<String, Transaction>>,
}

impl InMemoryTransactionRepo {
    pub fn new() -> Self {
        Self { store: Mutex::new(HashMap::new()) }
    }
}

impl TransactionRepository for InMemoryTransactionRepo {
    fn save(&self, tx: Transaction) {
        let tx_id = tx.id.clone(); // keep id before moving tx
        self.store.lock().unwrap().insert(tx_id.clone(), tx);
        tracing::debug!(tx_id = %tx_id, "Saved transaction in memory");
    }

    fn get(&self, id: &str) -> Option<Transaction> {
        self.store.lock().unwrap().get(id).cloned()
    }
}
