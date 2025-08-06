// src/domain/repository.rs

use crate::domain::transaction::Transaction;

pub trait TransRepository: Send + Sync {
    fn save(&self, tx: Transaction);
    fn get(&self, id: &str) -> Option<Transaction>;
}

use crate::domain::scoring::Score;

pub trait ScoreRepository: Send + Sync {
    fn save(&self, result: Score);
    fn get(&self, tx_id: &str) -> Option<Score>;
}
