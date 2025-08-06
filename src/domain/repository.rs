// src/domain/repository.rs

use crate::domain::transaction::Transaction;

pub trait TransactionRepository: Send + Sync {
    fn save(&self, tx: Transaction);
    fn get(&self, id: &str) -> Option<Transaction>;
}

use crate::domain::scoring::ScoringResult;

pub trait ScoringResultRepository: Send + Sync {
    fn save(&self, result: ScoringResult);
    fn get(&self, tx_id: &str) -> Option<ScoringResult>;
}
