// src/domain/scoring.rs

#[derive(Debug, Clone)]
pub struct Score {
    pub id: String,
    pub score: f64,
    pub is_fraud: bool,
}
