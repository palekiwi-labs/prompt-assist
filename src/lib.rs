pub mod cli;
pub mod git;
pub mod github_api;
pub mod output;
pub mod prompt;
pub mod local_repo;
pub mod pull_request;
pub mod prompt_context;
pub mod error;

use crate::cli::Cli;
use crate::error::AppError;
use crate::github_api::GitHubClient;
use crate::local_repo::LocalRepo;
use crate::prompt_context::PromptContext;

pub async fn run(cli: Cli) -> Result<(), AppError> {
    println!("Generating review prompt for PR #{}", cli.pr_number);

    // Extract local repository information
    let local_repo = LocalRepo::from_path(cli.repo_path)?;
    
    println!("Found GitHub repository: {}/{}", 
             local_repo.github_repo.owner, 
             local_repo.github_repo.name);

    // Create GitHub API client
    let github_client = GitHubClient::new();
    
    // Fetch pull request data
    println!("Fetching pull request data from GitHub API...");
    let pr = github_client
        .fetch_pull_request(&local_repo.github_repo, cli.pr_number)
        .await?;
    
    println!("Fetched PR: '{}' by {}", pr.title, pr.author.login);
    println!("  - {} comments", pr.comments.len());
    println!("  - {} review comments", pr.review_comments.len());
    println!("  - {} reviews", pr.reviews.len());

    // Create context for prompt generation
    let _context = PromptContext { repo: local_repo, pr };
    
    // TODO: Generate and output the actual prompt
    println!("\nPrompt generation complete!");

    Ok(())
}
