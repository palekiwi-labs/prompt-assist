use std::path::{Path,PathBuf};
use std::env;

#[derive(Debug)]
pub enum GitError {
    NotARepository(PathBuf),
    IoError(std::io::Error),
}

impl std::fmt::Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitError::NotARepository(path) => write!(f, "Not a git repository: {}", path.display()),
            GitError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for GitError {}

impl From<std::io::Error> for GitError {
    fn from(err: std::io::Error) -> Self {
        GitError::IoError(err)
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
