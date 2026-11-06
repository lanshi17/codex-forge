//! Durable storage port. No in-memory implementation is presented as SQLite.
use forge_domain::Task;
use forge_domain::TaskId;
use std::io;

pub const INITIAL_SCHEMA: &str = include_str!("../migrations/001_initial.sql");

pub struct TaskSnapshot {
    pub version: u64,
    pub task: Task,
}

pub trait TaskRepository: Send + Sync {
    fn load(&self, task: TaskId) -> io::Result<Option<TaskSnapshot>>;
    /// Compare-and-swap, atomically with assignments, reports and review audit records.
    /// None inserts only; Some(version) updates only if the persisted version matches.
    fn save(&self, task: &Task, expected_version: Option<u64>) -> io::Result<u64>;
}
