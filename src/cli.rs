use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "Generate AI prompts for a git repo")]
pub struct Cli {
    pub pr_number: u32,

    #[arg(long, short)]
    pub repo_path: Option<PathBuf>,

    #[arg(long, short = 'o', help = "Output file path for the generated markdown")]
    pub output: Option<PathBuf>,
}
