//! Host-collected evidence. Worker-authored statements cannot satisfy this gate.
use forge_domain::valid_commit;
use thiserror::Error;

pub struct CheckResult {
    pub command: Vec<String>,
    pub exit_code: i32,
}

pub struct ReviewEvidence {
    pub observed_head: String,
    pub diff_inspected: bool,
    pub checks: Vec<CheckResult>,
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("review requires a matching commit, inspected diff and passing host-run checks")]
pub struct ReviewError;

impl ReviewEvidence {
    pub fn bind_commit(&self, reported_head: &str) -> Result<(), ReviewError> {
        if !valid_commit(reported_head) || self.observed_head != reported_head {
            return Err(ReviewError);
        }
        Ok(())
    }

    pub fn validate(&self, reported_head: &str) -> Result<(), ReviewError> {
        self.bind_commit(reported_head)?;
        if !self.diff_inspected
            || self.checks.is_empty()
            || self
                .checks
                .iter()
                .any(|c| c.command.is_empty() || c.exit_code != 0)
        {
            return Err(ReviewError);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stale_commit_missing_checks_and_failed_tests_cannot_pass() {
        let head = "a".repeat(40);
        let mut evidence = ReviewEvidence {
            observed_head: head.clone(),
            diff_inspected: true,
            checks: vec![],
        };
        assert!(evidence.validate(&head).is_err());
        evidence.checks.push(CheckResult {
            command: vec!["cargo".into(), "test".into()],
            exit_code: 1,
        });
        assert!(evidence.validate(&head).is_err());
        evidence.checks[0].exit_code = 0;
        assert!(evidence.validate(&"b".repeat(40)).is_err());
        assert!(evidence.validate(&head).is_ok());
    }
}
