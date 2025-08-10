use std::path::{PathBuf};

/// A git remote with name and URL
#[derive(Debug, Clone)]
pub struct GitRemote {
    pub name: String,
    pub url: String,
}

#[derive(Debug)]
pub enum GitError {
    NotARepository(PathBuf),
    NoGitHubRemote,
    IoError(std::io::Error),
    Git2Error(git2::Error),
}

impl std::fmt::Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitError::NotARepository(path) => write!(f, "Not a git repository: {}", path.display()),
            GitError::NoGitHubRemote => write!(f, "No github remote found"),
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
