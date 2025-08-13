use crate::git::GitHubRepo;
use crate::pull_request::{PullRequest, PullRequestError, IssueComment, ReviewComment, Review, User, Branch};
use reqwest::{Client, StatusCode};
use serde_json::Value;
use std::env;

/// GitHub API client for fetching pull request data
pub struct GitHubClient {
    client: Client,
    token: Option<String>,
}

impl GitHubClient {
    /// Create a new GitHub API client
    /// Looks for GITHUB_TOKEN environment variable for authentication
    pub fn new() -> Self {
        let client = Client::new();
        let token = env::var("GITHUB_TOKEN").ok();
        
        Self { client, token }
    }

    /// Fetch complete pull request data including comments and reviews
    pub async fn fetch_pull_request(
        &self,
        github_repo: &GitHubRepo,
        pr_number: u32,
    ) -> Result<PullRequest, PullRequestError> {
        // Fetch basic PR data
        let pr_data = self.fetch_pr_data(github_repo, pr_number).await?;
        
        // Fetch additional data in parallel
        let (comments, review_comments, reviews) = tokio::try_join!(
            self.fetch_issue_comments(github_repo, pr_number),
            self.fetch_review_comments(github_repo, pr_number),
            self.fetch_reviews(github_repo, pr_number)
        )?;

        // Parse the main PR data
        let mut pull_request = self.parse_pull_request(&pr_data, pr_number)?;
        
        // Add the fetched comments and reviews
        pull_request.comments = comments;
        pull_request.review_comments = review_comments;
        pull_request.reviews = reviews;

        Ok(pull_request)
    }

    /// Fetch basic pull request data
    async fn fetch_pr_data(
        &self,
        github_repo: &GitHubRepo,
        pr_number: u32,
    ) -> Result<Value, PullRequestError> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls/{}",
            github_repo.owner, github_repo.name, pr_number
        );

        let response = self.make_request(&url).await?;
        Ok(response)
    }

    /// Fetch issue comments (general PR comments)
    async fn fetch_issue_comments(
        &self,
        github_repo: &GitHubRepo,
        pr_number: u32,
    ) -> Result<Vec<IssueComment>, PullRequestError> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/issues/{}/comments",
            github_repo.owner, github_repo.name, pr_number
        );

        let response = self.make_request(&url).await?;
        self.parse_issue_comments(&response)
    }

    /// Fetch review comments (line-specific comments)
    async fn fetch_review_comments(
        &self,
        github_repo: &GitHubRepo,
        pr_number: u32,
    ) -> Result<Vec<ReviewComment>, PullRequestError> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls/{}/comments",
            github_repo.owner, github_repo.name, pr_number
        );

        let response = self.make_request(&url).await?;
        self.parse_review_comments(&response)
    }

    /// Fetch reviews
    async fn fetch_reviews(
        &self,
        github_repo: &GitHubRepo,
        pr_number: u32,
    ) -> Result<Vec<Review>, PullRequestError> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls/{}/reviews",
            github_repo.owner, github_repo.name, pr_number
        );

        let response = self.make_request(&url).await?;
        self.parse_reviews(&response)
    }

    /// Make a HTTP request to the GitHub API with proper headers and error handling
    async fn make_request(&self, url: &str) -> Result<Value, PullRequestError> {
        let mut request = self.client.get(url)
            .header("User-Agent", "prompt-assist/0.1.0")
            .header("Accept", "application/vnd.github.v3+json");

        // Add authentication if token is available
        if let Some(ref token) = self.token {
            request = request.header("Authorization", format!("token {}", token));
        }

        let response = request.send().await?;

        match response.status() {
            StatusCode::OK => {
                let json: Value = response.json().await?;
                Ok(json)
            }
            StatusCode::NOT_FOUND => {
                // Extract PR number from URL for better error message
                let pr_number = url.split('/').last()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                Err(PullRequestError::NotFound(pr_number))
            }
            StatusCode::FORBIDDEN => Err(PullRequestError::RateLimited),
            StatusCode::UNAUTHORIZED => Err(PullRequestError::Unauthorized),
            _ => {
                // For other status codes, return a generic HTTP error
                let status = response.status();
                Err(PullRequestError::HttpError(
                    reqwest::Error::from(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("HTTP {}", status)
                    ))
                ))
            }
        }
    }

    /// Parse the main pull request data from GitHub API response
    fn parse_pull_request(&self, data: &Value, pr_number: u32) -> Result<PullRequest, PullRequestError> {
        Ok(PullRequest {
            number: pr_number,
            title: data["title"].as_str().unwrap_or("").to_string(),
            body: data["body"].as_str().map(|s| s.to_string()),
            state: data["state"].as_str().unwrap_or("").to_string(),
            author: User {
                login: data["user"]["login"].as_str().unwrap_or("").to_string(),
                id: data["user"]["id"].as_u64().unwrap_or(0) as u32,
            },
            base: Branch {
                label: data["base"]["label"].as_str().unwrap_or("").to_string(),
                ref_name: data["base"]["ref"].as_str().unwrap_or("").to_string(),
                sha: data["base"]["sha"].as_str().unwrap_or("").to_string(),
            },
            head: Branch {
                label: data["head"]["label"].as_str().unwrap_or("").to_string(),
                ref_name: data["head"]["ref"].as_str().unwrap_or("").to_string(),
                sha: data["head"]["sha"].as_str().unwrap_or("").to_string(),
            },
            created_at: data["created_at"].as_str()
                .and_then(|s| s.parse().ok())
                .unwrap_or_else(|| chrono::Utc::now()),
            updated_at: data["updated_at"].as_str()
                .and_then(|s| s.parse().ok())
                .unwrap_or_else(|| chrono::Utc::now()),
            merged_at: data["merged_at"].as_str()
                .and_then(|s| s.parse().ok()),
            comments: Vec::new(), // Will be populated by caller
            review_comments: Vec::new(), // Will be populated by caller
            reviews: Vec::new(), // Will be populated by caller
        })
    }

    /// Parse issue comments from GitHub API response
    fn parse_issue_comments(&self, data: &Value) -> Result<Vec<IssueComment>, PullRequestError> {
        let comments = data.as_array().unwrap_or(&vec![]);
        
        Ok(comments.iter().map(|comment| {
            IssueComment {
                id: comment["id"].as_u64().unwrap_or(0) as u32,
                user: User {
                    login: comment["user"]["login"].as_str().unwrap_or("").to_string(),
                    id: comment["user"]["id"].as_u64().unwrap_or(0) as u32,
                },
                body: comment["body"].as_str().unwrap_or("").to_string(),
                created_at: comment["created_at"].as_str()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or_else(|| chrono::Utc::now()),
                updated_at: comment["updated_at"].as_str()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or_else(|| chrono::Utc::now()),
            }
        }).collect())
    }

    /// Parse review comments from GitHub API response
    fn parse_review_comments(&self, data: &Value) -> Result<Vec<ReviewComment>, PullRequestError> {
        let comments = data.as_array().unwrap_or(&vec![]);
        
        Ok(comments.iter().map(|comment| {
            ReviewComment {
                id: comment["id"].as_u64().unwrap_or(0) as u32,
                user: User {
                    login: comment["user"]["login"].as_str().unwrap_or("").to_string(),
                    id: comment["user"]["id"].as_u64().unwrap_or(0) as u32,
                },
                body: comment["body"].as_str().unwrap_or("").to_string(),
                path: comment["path"].as_str().unwrap_or("").to_string(),
                position: comment["position"].as_u64().map(|p| p as u32),
                line: comment["line"].as_u64().map(|l| l as u32),
                created_at: comment["created_at"].as_str()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or_else(|| chrono::Utc::now()),
                updated_at: comment["updated_at"].as_str()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or_else(|| chrono::Utc::now()),
            }
        }).collect())
    }

    /// Parse reviews from GitHub API response
    fn parse_reviews(&self, data: &Value) -> Result<Vec<Review>, PullRequestError> {
        let reviews = data.as_array().unwrap_or(&vec![]);
        
        Ok(reviews.iter().map(|review| {
            Review {
                id: review["id"].as_u64().unwrap_or(0) as u32,
                user: User {
                    login: review["user"]["login"].as_str().unwrap_or("").to_string(),
                    id: review["user"]["id"].as_u64().unwrap_or(0) as u32,
                },
                body: review["body"].as_str().map(|s| s.to_string()),
                state: review["state"].as_str().unwrap_or("").to_string(),
                submitted_at: review["submitted_at"].as_str()
                    .and_then(|s| s.parse().ok()),
            }
        }).collect())
    }
}

impl Default for GitHubClient {
    fn default() -> Self {
        Self::new()
    }
}
