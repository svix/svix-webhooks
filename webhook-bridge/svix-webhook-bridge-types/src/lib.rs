pub use async_trait::async_trait;

/// Effectively a black box to the supervisor.
/// Plugins should run until they are done, and likely they should not be "done" until the program
/// exits.
#[async_trait]
pub trait Plugin: Send {
    async fn run(&self) -> std::io::Result<()>;
}
