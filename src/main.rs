use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "Generate AI prompts for a git repo")]
struct Cli {
    pr_number: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("Generating review prompt for PR #{}", cli.pr_number);

    Ok(())
}
