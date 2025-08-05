// src/domain/fraud_scorer.rs

use crate::domain::transaction::Transaction;
use rand::Rng;

// Trait that defines fraud detection behavior
pub trait FraudScorer {
    fn is_fraud(&self, tx: &Transaction) -> bool;
}

// Random scoring implementation
pub struct RandomScorer {
    pub fraud_rate: f64,
}

impl FraudScorer for RandomScorer {
    fn is_fraud(&self, _tx: &Transaction) -> bool {
        // let mut rng = rand::thread_rng();
        let mut rng = rand::rng(); // New function replacing thread_rng()
        // rng.gen_bool(self.fraud_rate)
        rng.random_bool(self.fraud_rate)
    }
}

// Rule-based scoring implementation
pub struct RuleBasedScorer;

impl FraudScorer for RuleBasedScorer {
    fn is_fraud(&self, tx: &Transaction) -> bool {
        tx.amount > 1000.0 || tx.currency == "BTC"
    }
}

// Placeholder for future ML integration
pub struct MlModelScorer;

impl FraudScorer for MlModelScorer {
    fn is_fraud(&self, tx: &Transaction) -> bool {
        println!("Calling ML model for transaction {}", tx.id);
        false // Stubbed for now
    }
}
