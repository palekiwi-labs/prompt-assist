use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub author: User,
    pub base: Branch,
    pub head: Branch,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub merged_at: Option<DateTime<Utc>>,
    pub comments: Vec<IssueComment>,
    pub review_comments: Vec<ReviewComment>,
    pub reviews: Vec<Review>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub label: String,
    pub ref_name: String,
    pub sha: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueComment {
    pub id: u64,
    pub user: User,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewComment {
    pub id: u64,
    pub user: User,
    pub body: String,
    pub path: String,
    pub position: Option<u32>,
    pub line: Option<u32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: u64,
    pub user: User,
    pub body: Option<String>,
    pub state: String, // "APPROVED", "CHANGES_REQUESTED", "COMMENTED"
    pub submitted_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub enum PullRequestError {
    HttpError(reqwest::Error),
    JsonError(serde_json::Error),
    NotFound(u64),
    RateLimited,
    Unauthorized,
}

impl std::fmt::Display for PullRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PullRequestError::HttpError(err) => write!(f, "HTTP error: {}", err),
            PullRequestError::JsonError(err) => write!(f, "JSON parsing error: {}", err),
            PullRequestError::NotFound(pr_number) => write!(f, "Pull request #{} not found", pr_number),
            PullRequestError::RateLimited => write!(f, "GitHub API rate limit exceeded"),
            PullRequestError::Unauthorized => write!(f, "GitHub API unauthorized - check token"),
        }
    }
}

impl std::error::Error for PullRequestError {}

impl From<reqwest::Error> for PullRequestError {
    fn from(err: reqwest::Error) -> Self {
        PullRequestError::HttpError(err)
    }
}

impl From<serde_json::Error> for PullRequestError {
    fn from(err: serde_json::Error) -> Self {
        PullRequestError::JsonError(err)
    }
}
