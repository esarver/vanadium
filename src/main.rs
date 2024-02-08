use std::{fs, path::PathBuf};

use clap::Parser;

use thiserror::Error;

#[derive(Error, Debug)]
enum VanadiumErrors {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The file to open in the editor
    file: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let contents = fs::read_to_string(cli.file)?;

    println!("{contents}");

    Ok(())
}
