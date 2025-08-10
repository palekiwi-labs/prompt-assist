pub mod cli;
pub mod git;
pub mod models;
pub mod output;
pub mod prompt;

use crate::cli::Cli;
use crate::git::resolve_repo_path;
use crate::models::RepoInfo;
use crate::output::write_output;
use crate::prompt::generate_review_prompt;

/// Main application error type
#[derive(Debug)]
pub enum AppError {
    Git(git::GitError),
    Output(output::OutputError),
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

impl From<git::GitError> for AppError {
    fn from(err: git::GitError) -> Self {
        AppError::Git(err)
    }
}

impl From<output::OutputError> for AppError {
    fn from(err: output::OutputError) -> Self {
        AppError::Output(err)
    }
}

/// Main application logic
pub fn run(cli: Cli) -> Result<(), AppError> {
    println!("Generating review prompt for PR #{}", cli.pr_number);

    // Resolve and validate repository path
    let repo_path = resolve_repo_path(cli.repo_path)?;
    println!("Found git repository at: {}", repo_path.display());

    // Create repository info
    let repo_info = RepoInfo::new(repo_path);

    // Generate markdown content
    let markdown_content = generate_review_prompt(cli.pr_number, &repo_info);

    // Write output
    write_output(&markdown_content, cli.output)?;

    Ok(())
}
