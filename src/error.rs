use crate::git::GitError;
use crate::output::OutputError;

#[derive(Debug)]
pub enum AppError {
    Git(GitError),
    Output(OutputError),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Git(err) => write!(f, "Git error: {}", err),
            AppError::Output(err) => write!(f, "Output error: {}", err),
        }
    }
}

impl std::error::Error for AppError {}

impl From<GitError> for AppError {
    fn from(err: GitError) -> Self {
        AppError::Git(err)
    }
}

impl From<OutputError> for AppError {
    fn from(err: OutputError) -> Self {
        AppError::Output(err)
    }
}

impl From<PullRequestError> for AppError {
    fn from(err: PullRequestError) -> Self {
        AppError::PullRequest(err)
    }
}

