//! Lifecycle port; a process supervisor and OS isolation must precede execution support.
use forge_agent_backend::{AgentExecutor, BackendFuture};
use forge_domain::AgentSession;

pub trait AgentRuntime: Send + Sync {
    fn executor(&self) -> &dyn AgentExecutor;
    fn recover(&self) -> BackendFuture<'_, Vec<AgentSession>>;
}
