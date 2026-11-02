//! Git stays an external executable; never execute user text through a shell.
use forge_domain::{AssignmentId, WorkspaceLease};
use std::io;

pub trait WorktreeManager: Send + Sync {
    fn allocate(&self, assignment: AssignmentId, base_commit: &str) -> io::Result<WorkspaceLease>;
    fn release(&self, assignment: AssignmentId) -> io::Result<()>;
}
