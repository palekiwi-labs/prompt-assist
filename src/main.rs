use clap::Parser;
use prompt_assist::cli::Cli;
use prompt_assist::error::AppError;

fn main() -> Result<(), AppError> {
    let cli = Cli::parse();

    prompt_assist::run(cli)?;

    Ok(())
}
