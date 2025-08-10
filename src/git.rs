use git2::Repository;
use std::env;
use std::path::{Path, PathBuf};

/// Information extracted from a git repository
#[derive(Debug, Clone)]
pub struct GitInfo {
    pub repository_root: PathBuf,
    pub remote_url: Option<String>,
    pub current_branch: Option<String>,
    pub remotes: Vec<GitRemote>,
}

/// A git remote with name and URL
#[derive(Debug, Clone)]
pub struct GitRemote {
    pub name: String,
    pub url: String,
}

#[derive(Debug)]
pub enum GitError {
    NotARepository(PathBuf),
    IoError(std::io::Error),
    Git2Error(git2::Error),
}

impl std::fmt::Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitError::NotARepository(path) => write!(f, "Not a git repository: {}", path.display()),
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

/// Resolve the repository path, defaulting to current directory if none provided
pub fn resolve_repo_path(repo_path: Option<PathBuf>) -> Result<PathBuf, GitError> {
    let path = repo_path.unwrap_or_else(|| env::current_dir().unwrap());
    validate_git_repository(&path)?;
    Ok(path)
}

/// Validate that the given path contains a git repository
pub fn validate_git_repository(path: &Path) -> Result<(), GitError> {
    if !path.join(".git").exists() {
        return Err(GitError::NotARepository(path.to_path_buf()));
    }
    Ok(())
}

/// Extract git information from a repository
pub fn extract_git_info(repo_path: &Path) -> Result<GitInfo, GitError> {
    // Open the repository using git2
    let repo = Repository::open(repo_path)?;

    // Get the repository root directory
    let repository_root = repo
        .workdir()
        .ok_or_else(|| GitError::NotARepository(repo_path.to_path_buf()))?;

    // Extract all remotes
    let remotes = extract_remotes(&repo)?;

    // Find the best GitHub remote URL
    let remote_url = find_github_remote(&remotes);

    // Get current branch name
    let current_branch = get_current_branch(&repo)?;

    Ok(GitInfo {
        repository_root: repository_root.to_path_buf(),
        remote_url,
        current_branch,
        remotes,
    })
}

/// Extract all remotes from the repository
fn extract_remotes(repo: &Repository) -> Result<Vec<GitRemote>, GitError> {
    let mut remotes = Vec::new();
    let remote_names = repo.remotes()?;
    
    for name in remote_names.iter().flatten() {
        if let Ok(remote) = repo.find_remote(name)
            && let Some(url) = remote.url() {
            remotes.push(GitRemote {
                name: name.to_string(),
                url: url.to_string(),
            });
        }
    }
    
    Ok(remotes)
}

/// Find the best GitHub remote URL (prefer "origin", then any GitHub URL)
fn find_github_remote(remotes: &[GitRemote]) -> Option<String> {
    // First, try to find "origin" remote that's a GitHub URL
    if let Some(origin) = remotes.iter().find(|r| r.name == "origin") {
        if is_github_url(&origin.url) {
            return Some(origin.url.clone());
        }
    }

    // If no GitHub origin, find any GitHub remote
    remotes
        .iter()
        .find(|r| is_github_url(&r.url))
        .map(|r| r.url.clone())
}

/// Check if a URL is a GitHub URL
fn is_github_url(url: &str) -> bool {
    url.contains("github.com")
        && (url.starts_with("https://github.com")
            || url.starts_with("git@github.com")
            || url.starts_with("ssh://git@github.com"))
}

/// Get the current branch name
fn get_current_branch(repo: &Repository) -> Result<Option<String>, GitError> {
    let head = repo.head()?;

    if let Some(branch_name) = head.shorthand() {
        Ok(Some(branch_name.to_string()))
    } else {
        // Might be in detached HEAD state
        Ok(None)
    }
}
