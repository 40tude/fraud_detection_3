// src/state_machine/state.rs

use super::event::Event;
use std::any::Any;
use std::fmt::Debug;

pub trait State: Debug + Any {
    fn handle(self: Box<Self>, input: Event) -> Box<dyn State>;
    fn name(&self) -> &'static str;
    fn as_any(self: Box<Self>) -> Box<dyn Any>; // needed for downcast
}

#[derive(Debug)]
pub struct Validated;
impl State for Validated {
    fn handle(self: Box<Self>, _event: Event) -> Box<dyn State> {
        println!("State: Validated -> Enriched");
        Box::new(Enriched)
    }

    fn name(&self) -> &'static str {
        "Validated"
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

// #[derive(Debug)]
// pub struct Enriched;
// impl State for Enriched {
//     fn handle(self: Box<Self>, _event: Event) -> Box<dyn State> {
//         println!("State: Enriched -> Persisted");
//         Box::new(Persisted)
//     }

//     fn name(&self) -> &'static str {
//         "Enriched"
//     }
// }

#[derive(Debug)]
pub struct Enriched;

use crate::domain::fraud_scorer::FraudScorer;
use crate::domain::transaction::Transaction;

impl Enriched {
    pub fn handle_with_scorer(self: Box<Self>, tx: &Transaction, scorer: &dyn FraudScorer) -> Box<dyn State> {
        if scorer.is_fraud(tx) {
            println!("State: Enriched -> FlaggedAsFraud");
            Box::new(FlaggedAsFraud)
        } else {
            println!("State: Enriched -> Persisted");
            Box::new(Persisted)
        }
    }
}

impl State for Enriched {
    fn handle(self: Box<Self>, _event: Event) -> Box<dyn State> {
        // Should never be used in this version
        println!("Enriched: call handle_with_scorer instead.");
        self
    }

    fn name(&self) -> &'static str {
        "Enriched"
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

#[derive(Debug)]
pub struct Persisted;
impl State for Persisted {
    fn handle(self: Box<Self>, _event: Event) -> Box<dyn State> {
        println!("State: Persisted (final state reached)");
        self
    }

    fn name(&self) -> &'static str {
        "Persisted"
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

#[derive(Debug)]
pub struct FlaggedAsFraud;
impl State for FlaggedAsFraud {
    fn handle(self: Box<Self>, _event: Event) -> Box<dyn State> {
        println!("State: FlaggedAsFraud (final state reached)");
        self
    }

    fn name(&self) -> &'static str {
        "FlaggedAsFraud"
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
