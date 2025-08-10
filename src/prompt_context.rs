use crate::local_repo::LocalRepo;
use crate::pull_request::PullRequest;

pub struct PromptContext {
    pub repo: LocalRepo,
    pub pr: PullRequest,
    // diff: Option<GitDiff>,
    // issues: Vec<JiraIssue>,
}
