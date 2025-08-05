// src/command_bus.rs

pub trait Command {
    type Output;
}

pub trait Handler<C: Command> {
    fn handle(&self, cmd: C) -> C::Output;
}

// Simple dispatcher function
pub fn dispatch<C: Command, H: Handler<C>>(cmd: C, handler: H) -> C::Output {
    handler.handle(cmd)
}
