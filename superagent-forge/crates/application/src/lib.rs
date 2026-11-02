//! Trusted application boundary. Caller identities must come from host authentication.
use forge_domain::*;
use forge_review::{ReviewError, ReviewEvidence};
use std::collections::BTreeSet;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct Policy {
    pub authorizers: BTreeSet<PrincipalId>,
    pub reviewers: BTreeSet<PrincipalId>,
    pub merge_authorizers: BTreeSet<PrincipalId>,
}

pub struct TaskService {
    policy: Policy,
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("caller is not permitted to perform this action")]
    Forbidden,
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Review(#[from] ReviewError),
}

/// A bound approval result, not a git operation. A future merger must recheck HEAD.
#[derive(Debug, Clone)]
pub struct MergeAuthorization {
    pub task_id: TaskId,
    pub assignment_id: AssignmentId,
    pub head_commit: String,
    pub authorized_by: PrincipalId,
}

impl TaskService {
    pub fn new(policy: Policy) -> Self {
        Self { policy }
    }

    pub fn authorize(
        &self,
        caller: &PrincipalId,
        task: &mut Task,
        mandate: TaskMandate,
        now: u64,
    ) -> Result<(), ApplicationError> {
        if !self.policy.authorizers.contains(caller) {
            return Err(ApplicationError::Forbidden);
        }
        Ok(task.authorize(mandate, now)?)
    }

    pub fn review(
        &self,
        caller: &PrincipalId,
        task: &mut Task,
        review: ReviewRecord,
        evidence: &ReviewEvidence,
    ) -> Result<(), ApplicationError> {
        if !self.policy.reviewers.contains(caller) || caller != &review.reviewer {
            return Err(ApplicationError::Forbidden);
        }
        match review.decision {
            ReviewDecision::Approve => evidence.validate(&review.reviewed_head)?,
            ReviewDecision::Reject => evidence.bind_commit(&review.reviewed_head)?,
        }
        Ok(task.review(review)?)
    }

    pub fn complete(
        &self,
        caller: &PrincipalId,
        task: &mut Task,
        report: ExecutionReport,
    ) -> Result<(), ApplicationError> {
        if task
            .assignments()
            .last()
            .is_none_or(|a| &a.worker != caller)
        {
            return Err(ApplicationError::Forbidden);
        }
        Ok(task.complete(report)?)
    }

    pub fn authorize_merge(
        &self,
        caller: &PrincipalId,
        task: &Task,
    ) -> Result<MergeAuthorization, ApplicationError> {
        if !self.policy.merge_authorizers.contains(caller) || task.status() != TaskStatus::Approved
        {
            return Err(ApplicationError::Forbidden);
        }
        let review = task.reviews().last().ok_or(ApplicationError::Forbidden)?;
        Ok(MergeAuthorization {
            task_id: task.id(),
            assignment_id: review.assignment_id,
            head_commit: review.reviewed_head.clone(),
            authorized_by: caller.clone(),
        })
    }
}
