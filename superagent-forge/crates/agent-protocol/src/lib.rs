use forge_domain::{Assignment, TaskMandate};
use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: u32 = 1;

/// Host-created request; a worker receives a mandate, never authority to expand it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRequest {
    pub version: u32,
    pub assignment: Assignment,
    pub mandate: TaskMandate,
    pub prompt: String,
}
