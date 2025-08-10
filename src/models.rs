use std::path::PathBuf;
use crate::git::GitInfo;

/// Repository information for prompt generation
#[derive(Debug, Clone)]
pub struct RepoInfo {
    pub path: PathBuf,
    pub git_info: Option<GitInfo>,
}

impl RepoInfo {
    pub fn new(path: PathBuf) -> Self {
        Self { 
            path,
            git_info: None,
        }
    }
    
    pub fn with_git_info(mut self, git_info: GitInfo) -> Self {
        self.git_info = Some(git_info);
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
