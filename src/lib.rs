pub mod cli;
pub mod git;
pub mod output;
pub mod prompt;
pub mod local_repo;
pub mod pull_request;
pub mod prompt_context;
pub mod error;

use crate::cli::Cli;
use crate::error::AppError;
use crate::local_repo::LocalRepo;
use crate::pull_request::PullRequest;
use crate::prompt_context::PromptContext;

pub fn run(cli: Cli) -> Result<(), AppError> {
    println!("Generating review prompt for PR #{}", cli.pr_number);

    let local_repo = LocalRepo::from_path(cli.repo_path)?;
    let pr = PullRequest {};

    let _context = PromptContext { repo: local_repo, pr };

    Ok(())
}
