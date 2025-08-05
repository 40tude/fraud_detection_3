// src/domain/transaction.rs

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub amount: f64,
    pub currency: String,
}
