use std::path::PathBuf;
use std::fs;

#[derive(Debug)]
pub enum OutputError {
    FileWriteError(std::io::Error),
}

impl std::fmt::Display for OutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputError::FileWriteError(err) => write!(f, "Failed to write file: {}", err),
        }
    }
}

impl std::error::Error for OutputError {}

impl From<std::io::Error> for OutputError {
    fn from(err: std::io::Error) -> Self {
        OutputError::FileWriteError(err)
    }
}

/// Write content to either a file or stdout
pub fn write_output(content: &str, output_path: Option<PathBuf>) -> Result<(), OutputError> {
    match output_path {
        Some(path) => {
            fs::write(&path, content)?;
            println!("Generated prompt saved to: {}", path.display());
        }
        None => {
            println!("\n{}", content);
        }
    }
    Ok(())
}
