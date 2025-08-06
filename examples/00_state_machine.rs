// cargo run --example 00_state_machine

// Implements a finite state machine (FSM) in Rust.
// This example models a process that moves through three states:
//     Validated → Enriched → Persisted
// The transition between states is triggered by an input event (Event::Process).

// This example shows how to create a flexible and type-safe FSM using traits and dynamic dispatch.
// Each state is a struct that implements a shared trait called `State`.
// We use `Box<dyn State>` to hold the current state and allow transitions between different types.
// The main loop continues until the FSM reaches a terminal state.

use std::fmt::Debug;

// Define the possible events the FSM can handle.
// In this simple example, there's only one event: Process.
#[derive(Clone, Debug)]
pub enum Event {
    Process,
}

// Define the State trait that all states must implement.
// - handle: defines how the state reacts to an event and transitions to the next state.
// - name: returns the name of the current state as a string for logging and comparison.
pub trait State {
    fn handle(self: Box<Self>, input: Event) -> Box<dyn State>;
    fn name(&self) -> &'static str;
}

// State: Validated
// When receiving Event::Process, transitions to Enriched.
struct Validated;
impl State for Validated {
    fn handle(self: Box<Self>, _event: Event) -> Box<dyn State> {
        println!("State: Validated -> Enriched");
        Box::new(Enriched)
    }

    fn name(&self) -> &'static str {
        "Validated"
    }
}

// State: Enriched
// When receiving Event::Process, transitions to Persisted.
struct Enriched;
impl State for Enriched {
    fn handle(self: Box<Self>, _event: Event) -> Box<dyn State> {
        println!("State: Enriched -> Persisted");
        Box::new(Persisted)
    }

    fn name(&self) -> &'static str {
        "Enriched"
    }
}

// State: Persisted
// This is the final state. It returns itself to indicate that no further transitions occur.
struct Persisted;
impl State for Persisted {
    fn handle(self: Box<Self>, _event: Event) -> Box<dyn State> {
        println!("State: Persisted (final state reached)");
        self
    }

    fn name(&self) -> &'static str {
        "Persisted"
    }
}

// Runs the state machine starting from the Validated state.
// Repeatedly applies the same event and transitions between states.
// Stops when the FSM reaches a state that does not change (final state).
fn process_event(event: Event) {
    let mut state: Box<dyn State> = Box::new(Validated);

    loop {
        // Save the current state's name before moving to the next state
        let current_name = state.name();
        let next = state.handle(event.clone());

        // If the state hasn't changed, we assume we've reached the final state
        if current_name == next.name() {
            println!("Final state: {}", next.name());
            break;
        }

        state = next;
    }
}

fn main() {
    println!("--- State Machine Demo ---");
    // Start the FSM by sending the Process event
    process_event(Event::Process);
}
