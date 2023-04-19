use tokio::task::JoinHandle;

/// Trait for ability to create an instance of Self.
pub trait New {
    /// Create a new instance of self.
    fn new() -> Self;
}

pub trait Adapter {
    type Actor: New;

    /// Provide the context used to bind zmq sockets.
    fn ctx(&self) -> &zmq::Context;
    /// Endpoint for zmq sockets.
    fn endpoint(&self) -> &str;
    /// Spawn connection task.
    fn connect(&mut self) -> JoinHandle<()>;
    /// Initialize this `Adapter`.
    fn init(ctx: zmq::Context, endpoint: String) -> (Self, JoinHandle<()>) where Self: Sized;
}
