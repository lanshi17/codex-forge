//! Reserved MCP contract only. No stdio listener or JSON-RPC routing is active yet.
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ToolContract {
    pub name: &'static str,
    pub mutates_state: bool,
    pub implemented: bool,
}

pub fn planned_tools() -> Vec<ToolContract> {
    vec![
        ToolContract {
            name: "forge_task_create",
            mutates_state: true,
            implemented: false,
        },
        ToolContract {
            name: "forge_task_status",
            mutates_state: false,
            implemented: false,
        },
        ToolContract {
            name: "forge_assignment_cancel",
            mutates_state: true,
            implemented: false,
        },
    ]
}
