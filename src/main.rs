use clap::Parser;
use std::path::PathBuf;
use std::env;

#[derive(Parser)]
#[command(author, version, about = "Generate AI prompts for a git repo")]
struct Cli {
    pr_number: u32,

    #[arg(long, short)]
        repo_path: Option<PathBuf>
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

    Ok(())
}
