//! Dependency validation only; execution concurrency is a later milestone.
use forge_domain::TaskId;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ScheduleError {
    #[error("dependency points to a task outside the submitted graph")]
    MissingDependency,
    #[error("task dependency graph contains a cycle")]
    Cycle,
}

pub fn order(graph: &BTreeMap<TaskId, BTreeSet<TaskId>>) -> Result<Vec<TaskId>, ScheduleError> {
    if graph.values().flatten().any(|id| !graph.contains_key(id)) {
        return Err(ScheduleError::MissingDependency);
    }
    let mut pending = graph.clone();
    let mut sorted = Vec::with_capacity(graph.len());
    while !pending.is_empty() {
        let ready: Vec<_> = pending
            .iter()
            .filter(|(_, deps)| deps.is_empty())
            .map(|(id, _)| *id)
            .collect();
        if ready.is_empty() {
            return Err(ScheduleError::Cycle);
        }
        for id in ready {
            pending.remove(&id);
            for deps in pending.values_mut() {
                deps.remove(&id);
            }
            sorted.push(id);
        }
    }
    Ok(sorted)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dependencies_precede_dependents() {
        let a = TaskId::new();
        let b = TaskId::new();
        let c = TaskId::new();
        let graph = BTreeMap::from([
            (a, BTreeSet::new()),
            (b, BTreeSet::from([a])),
            (c, BTreeSet::from([a, b])),
        ]);
        assert_eq!(order(&graph).unwrap(), vec![a, b, c]);
    }
    #[test]
    fn rejects_cycles_and_dangling_dependencies() {
        let a = TaskId::new();
        let b = TaskId::new();
        assert_eq!(
            order(&BTreeMap::from([(a, BTreeSet::from([b]))])),
            Err(ScheduleError::MissingDependency)
        );
        assert_eq!(
            order(&BTreeMap::from([
                (a, BTreeSet::from([b])),
                (b, BTreeSet::from([a]))
            ])),
            Err(ScheduleError::Cycle)
        );
    }
}
