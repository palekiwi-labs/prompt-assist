use clap::Parser;
use std::path::{Path, PathBuf};
use std::env;
use std::fs;

fn generate_review_prompt(pr_number: u32, repo_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let content = format!(
        r#"# Code Review Prompt for PR #{}

## Repository
**Path:** {}

## Instructions
This is a placeholder for the AI code review prompt.

## Context
- PR Number: {}
- Repository: {}
- Generated at: {}

## Next Steps
- [ ] Fetch PR details from GitHub API
- [ ] Generate git diff
- [ ] Include file modifications
- [ ] Add linked issue context
"#,
        pr_number,
        repo_path.display(),
        pr_number,
        repo_path.display(),
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );
    
    Ok(content)
}

#[derive(Parser)]
#[command(author, version, about = "Generate AI prompts for a git repo")]
struct Cli {
    pr_number: u32,

    #[arg(long, short)]
    repo_path: Option<PathBuf>,

    #[arg(long, short = 'o', help = "Output file path for the generated markdown")]
    output: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("Generating review prompt for PR #{}", cli.pr_number);

    let repo_path = cli.repo_path.unwrap_or_else(|| env::current_dir().unwrap());
    if !repo_path.join(".git").exists() {
        eprintln!("Error: Not a git repository: {}", repo_path.display());
        std::process::exit(1);
    }

    println!("Found git repository at: {}", repo_path.display());

    // Generate markdown content
    let markdown_content = generate_review_prompt(cli.pr_number, &repo_path)?;

    // Handle output
    match cli.output {
        Some(output_path) => {
            fs::write(&output_path, markdown_content)?;
            println!("Generated prompt saved to: {}", output_path.display());
        }
        None => {
            println!("\n{}", markdown_content);
        }
    }

    Ok(())
}
