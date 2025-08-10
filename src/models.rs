use std::path::PathBuf;
use crate::git::GitData;

/// Application-level context for prompt generation
/// Composes data from various domains (git, github, jira, etc.)
#[derive(Debug, Clone)]
pub struct PromptContext {
    pub repo_path: PathBuf,
    pub git: Option<GitData>,
    // Future: github: Option<GitHubData>,
    // Future: jira: Option<JiraData>,
}

impl PromptContext {
    pub fn new(repo_path: PathBuf) -> Self {
        Self {
            repo_path,
            git: None,
        }
    }
    
    pub fn with_git(mut self, git: GitData) -> Self {
        self.git = Some(git);
        self
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
