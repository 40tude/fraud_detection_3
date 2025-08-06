// cargo run --example 03_command_bus

// A trait defines shared behavior that types can implement.
// Here, we define a trait named `Command`.
//
// Inside the trait, we declare an "associated type" named `Output`.
// - `type Output;` is *not* a method or a field.
// - It declares a placeholder type that each implementer of the trait
//   must specify when implementing `Command`.
//
// For example, one implementer might define `Output` as `String`,
// another as `u32`, depending on what kind of result their command produces.
//
// This associated type typically represents the result produced
// when the command is executed.
pub trait Command {
    type Output;
}

// This trait defines a `Handler` — a type capable of processing a command of type `C`.
//
// `C: Command` is a constraint that says:
// - The type `C` must implement the `Command` trait.
// - This means `C` has an associated type called `Output`.
//
// The `handle` method takes a command of type `C` (by value),
// and returns a result of type `C::Output`, which is the output
// defined by the specific implementation of `Command` for `C`.
//
// Example: If `C` is a `PrintCommand` and its `Output` is `()`, then
// `handle` will take a `PrintCommand` and return `()` (unit).
//
// This design allows handlers to be generic over many kinds of commands,
// each potentially producing a different kind of result.
pub trait Handler<C: Command> {
    fn handle(&self, cmd: C) -> C::Output;
}

// This struct represents a concrete command: "Create a user".
// It contains the data needed to perform the command — in this case, just a name.
//
// By convention, command structs are usually simple data holders.
// The actual logic is provided by a `Handler`.
struct CreateUser {
    pub name: String,
}

// We implement the `Command` trait for `CreateUser`.
// This tells Rust that `CreateUser` is a valid command,
// and specifies what kind of result (`Output`) is expected when the command is handled.
//
// In this case, handling the command will return a `String`,
// for example, the ID of the newly created user or a confirmation message.
impl Command for CreateUser {
    type Output = String;
}

// This struct represents a handler for the `CreateUser` command.
//
// It doesn't need to store any state, so it's defined as an empty struct.
// In real applications, a handler might hold references to a database,
// a logger, or other services needed to perform the operation.
struct CreateUserHandler;

// We implement the `Handler` trait for `CreateUserHandler`,
// specifying that it handles commands of type `CreateUser`.
//
// This means that `CreateUserHandler` must define the `handle` method,
// which takes a `CreateUser` command and returns a `String` —
// as specified by `CreateUser`'s associated `Output` type.
impl Handler<CreateUser> for CreateUserHandler {
    fn handle(&self, cmd: CreateUser) -> String {
        // Here we simulate creating a user by returning a confirmation message.
        // In a real application, this might insert a user into a database
        // and return the user's ID or a status message.
        format!("Created user: {}", cmd.name)
    }
}

// This function acts as a "Command Bus" or dispatcher.
// It takes a command of some type `C`, and a handler `H` that knows how to handle that command.
// It then calls the handler's `handle()` method and returns the result.
//
// The function is generic over:
// - `C`, the type of the command, which must implement the `Command` trait.
// - `H`, the type of the handler, which must implement `Handler<C>` — meaning it knows how to handle `C`.
//
// Why can’t we just write this?
// fn dispatch(cmd: Command, handler: Handler) -> Command::Output // DOES NOT COMPILE
//
// 1. Traits are not types
// In Rust, Command and Handler are traits, not concrete types.
// You can’t write cmd: Command because Rust doesn’t know which type you mean.
// You have to tell Rust: “This parameter is of some type C, and C implements the Command trait.”
//
// That’s why we write:
// fn dispatch<C: Command, H: Handler<C>>(cmd: C, handler: H) -> C::Output
// It introduces type parameters C and H, and constrains them with the traits they must implement.
//
// What about `-> Command::Output` ?
// Rust doesn't know which type’s Output you're referring to.
// The Output associated type depends on the specific type C that implements Command.
//
// That’s why we must refer to it as:
// -> C::Output
// Here, C is a concrete type parameter constrained by the Command trait, and C::Output is the associated type for that specific implementation.

fn dispatch<C: Command, H: Handler<C>>(cmd: C, handler: H) -> C::Output {
    handler.handle(cmd)
}

fn main() {
    // let result = dispatch(CreateUser { name: String::from("Alice") }, CreateUserHandler);
    let result = dispatch(CreateUser { name: "Alice".into() }, CreateUserHandler);
    println!("{result}"); // Output: Created user: Alice
}
