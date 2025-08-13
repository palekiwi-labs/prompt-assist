use clap::Parser;
use prompt_assist::cli::Cli;
use prompt_assist::error::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let cli = Cli::parse();

    prompt_assist::run(cli).await?;

    Ok(())
}
