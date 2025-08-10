pub mod cli;
pub mod git;
pub mod models;
pub mod output;
pub mod prompt;

use crate::cli::Cli;
use crate::git::{resolve_repo_path, extract_git_info};
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

    // Extract git information
    let git_info = extract_git_info(&repo_path)?;
    println!("Repository root: {}", git_info.repository_root.display());
    if let Some(ref remote_url) = git_info.remote_url {
        println!("GitHub remote: {}", remote_url);
    }
    if let Some(ref branch) = git_info.current_branch {
        println!("Current branch: {}", branch);
    }

    // Create repository info with git information
    let repo_info = RepoInfo::new(repo_path)
        .with_git_info(git_info);

    // Generate markdown content
    let markdown_content = generate_review_prompt(cli.pr_number, &repo_info);

    // Write output
    write_output(&markdown_content, cli.output)?;

    Ok(())
}
