// cargo run --example 04_command_bus

// Command Bus with more than one command

// Compare with 03_command_bus.rs
// Avoid to write   : let result                 = dispatch(CreateUser { name: "Alice".into() }, CreateUserHandler);
// Instead we write : let result: Option<String> = bus.dispatch(&CreateUser { name: "Alice".into() });
//
// We "drop" the application command bus and it is able to find the good handler
// Doing so the caller doesn't even know "who" will create (or delete) the user nor how this will be done
// This is totally transparent for him

use std::any::{Any, TypeId};
use std::collections::HashMap;

// Command trait
pub trait Command: Any + Send {
    fn as_any(&self) -> &dyn Any;
}
impl<T: Any + Send> Command for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// CommandHandler trait
pub trait CommandHandler: Send {
    fn handle(&self, cmd: &dyn Command) -> Box<dyn Any>;
    fn type_id(&self) -> TypeId;
}

// Implémentation du bus
struct AppCommandBus {
    handlers: HashMap<TypeId, Box<dyn CommandHandler>>,
}

impl AppCommandBus {
    fn new() -> Self {
        AppCommandBus { handlers: HashMap::new() }
    }

    fn register<C: Command, H>(&mut self, handler: H)
    where
        H: CommandHandler + 'static,
    {
        self.handlers.insert(TypeId::of::<C>(), Box::new(handler));
    }

    fn dispatch<R: 'static>(&self, cmd: &dyn Command) -> Option<R> {
        let type_id = cmd.as_any().type_id();
        let handler = self.handlers.get(&type_id)?;

        let result = handler.handle(cmd);
        result.downcast::<R>().ok().map(|boxed| *boxed)
    }
}

// Commandes
struct CreateUser {
    pub name: String,
}
struct DeleteUser {
    pub name: String,
}

// Handlers
struct CreateUserHandler;
impl CommandHandler for CreateUserHandler {
    fn handle(&self, cmd: &dyn Command) -> Box<dyn Any> {
        let create = cmd.as_any().downcast_ref::<CreateUser>().unwrap();
        let msg = format!("User {} is created", create.name);
        Box::new(msg)
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<CreateUser>()
    }
}

struct DeleteUserHandler;
impl CommandHandler for DeleteUserHandler {
    fn handle(&self, cmd: &dyn Command) -> Box<dyn Any> {
        let delete = cmd.as_any().downcast_ref::<DeleteUser>().unwrap();
        let msg = format!("User {} is deleted", delete.name);
        Box::new(msg)
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<DeleteUser>()
    }
}

// Usage
fn main() {
    let mut bus = AppCommandBus::new();
    bus.register::<CreateUser, _>(CreateUserHandler);
    bus.register::<DeleteUser, _>(DeleteUserHandler);

    let result: Option<String> = bus.dispatch(&CreateUser { name: "Alice".into() });
    println!("{}", result.unwrap()); // User Alice is created

    let result: Option<String> = bus.dispatch(&DeleteUser { name: "Alice".into() });
    println!("{}", result.unwrap()); // User Alice is deleted
}
