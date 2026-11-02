//! OMP configuration boundary. Process execution is intentionally not implemented yet.
use forge_agent_backend::AdapterSpec;
use forge_domain::BackendKind;
use std::path::PathBuf;

pub fn specification(executable: PathBuf, arguments: Vec<String>) -> AdapterSpec {
    AdapterSpec {
        kind: BackendKind::Omp,
        executable,
        arguments,
    }
}
