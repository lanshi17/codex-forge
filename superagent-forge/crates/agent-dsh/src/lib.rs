//! Optional external CLI configuration. No DSH SDK or Node dependency is linked.
use forge_agent_backend::AdapterSpec;
use forge_domain::BackendKind;
use std::path::PathBuf;

pub fn specification(executable: PathBuf, arguments: Vec<String>) -> AdapterSpec {
    AdapterSpec {
        kind: BackendKind::Dsh,
        executable,
        arguments,
    }
}
