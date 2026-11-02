//! Composition root for trusted use cases and optional executor ports.
use forge_agent_backend::AgentExecutor;
use forge_application::TaskService;

pub struct TaskOrchestrator {
    pub tasks: TaskService,
    pub executors: Vec<Box<dyn AgentExecutor>>,
}
