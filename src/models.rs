use std::path::PathBuf;

/// Repository information for prompt generation
#[derive(Debug, Clone)]
pub struct RepoInfo {
    pub path: PathBuf,
}

impl RepoInfo {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

/// Future: Pull request information from GitHub API
#[derive(Debug)]
pub struct PullRequestInfo {
    pub number: u32,
    pub title: String,
    pub body: String,
    pub author: String,
}

/// Future: Issue information from Jira
#[derive(Debug)]
pub struct IssueInfo {
    pub key: String,
    pub summary: String,
    pub description: String,
}
