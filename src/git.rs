use std::path::{PathBuf};

/// A git remote with name and URL
#[derive(Debug, Clone)]
pub struct GitRemote {
    pub name: String,
    pub url: String,
}

/// GitHub repository information extracted from a remote URL
#[derive(Debug, Clone)]
pub struct GitHubRepo {
    pub owner: String,
    pub name: String,
}

impl GitHubRepo {
    /// Parse GitHub owner/repo from a remote URL
    /// Handles both SSH and HTTPS URLs
    pub fn from_url(url: &str) -> Result<Self, GitError> {
        let path = extract_github_path(url)
            .ok_or_else(|| GitError::InvalidGitHubUrl(url.to_string()))?;
        
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() != 2 {
            return Err(GitError::InvalidGitHubUrl(url.to_string()));
        }
        
        let owner = parts[0].to_string();
        let mut repo_name = parts[1].to_string();
        
        // Remove .git suffix if present
        if repo_name.ends_with(".git") {
            repo_name.truncate(repo_name.len() - 4);
        }
        
        Ok(GitHubRepo {
            owner,
            name: repo_name,
        })
    }
}

/// Extract the owner/repo path from various GitHub URL formats
fn extract_github_path(url: &str) -> Option<&str> {
    if let Some(path) = url.strip_prefix("https://github.com/") {
        Some(path)
    } else if let Some(path) = url.strip_prefix("git@github.com:") {
        Some(path)
    } else if let Some(path) = url.strip_prefix("ssh://git@github.com/") {
        Some(path)
    } else {
        None
    }
}

#[derive(Debug)]
pub enum GitError {
    NotARepository(PathBuf),
    NoGitHubRemote,
    InvalidGitHubUrl(String),
    IoError(std::io::Error),
    Git2Error(git2::Error),
}

impl std::fmt::Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitError::NotARepository(path) => write!(f, "Not a git repository: {}", path.display()),
            GitError::NoGitHubRemote => write!(f, "No github remote found"),
            GitError::InvalidGitHubUrl(url) => write!(f, "Invalid GitHub URL: {}", url),
            GitError::IoError(err) => write!(f, "IO error: {}", err),
            GitError::Git2Error(err) => write!(f, "Git error: {}", err),
        }
    }
}

impl std::error::Error for GitError {}

impl From<std::io::Error> for GitError {
    fn from(err: std::io::Error) -> Self {
        GitError::IoError(err)
    }
}

impl From<git2::Error> for GitError {
    fn from(err: git2::Error) -> Self {
        GitError::Git2Error(err)
    }
}
