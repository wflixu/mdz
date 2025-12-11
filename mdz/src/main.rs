use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::Path;
use std::fs;

// 包含测试模块
#[cfg(test)]
mod tests;

#[derive(Parser)]
#[command(name = "mdz")]
#[command(about = "A CLI tool for MDZ (Markdown Zip) format")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Pack a Markdown file and its assets into an MDZ archive
    Pack {
        /// Input Markdown file
        #[arg(short, long)]
        input: String,

        /// Document title (optional, defaults to filename)
        #[arg(short = 't', long)]
        title: Option<String>,

        /// Document author (optional)
        #[arg(short = 'a', long)]
        author: Option<String>,
    },

    /// Unpack an MDZ archive
    Unpack {
        /// Input MDZ file
        input: String,

        /// Output directory (optional, defaults to current directory)
        #[arg(short = 'o', long)]
        output: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Pack { input, title, author } => {
            // Validate input file exists
            if !Path::new(&input).exists() {
                eprintln!("Error: Input file '{}' does not exist", input);
                std::process::exit(1);
            }

            // Generate output file name based on input file
            let input_path = Path::new(&input);
            let output_file = input_path.with_extension("mdz").to_str().unwrap().to_string();

            println!("Packing '{}' into '{}'...", input, output_file);

            // Call the pack function
            mdz_rs::pack(&input, &output_file, title, author).await?;

            println!("Successfully packed to {}", output_file);
        }

        Commands::Unpack { input, output } => {
            // Validate input file exists
            if !Path::new(&input).exists() {
                eprintln!("Error: Input file '{}' does not exist", input);
                std::process::exit(1);
            }

            // Determine output directory
            let output_dir = output.unwrap_or_else(|| ".".to_string());

            // Create output directory if it doesn't exist
            if !Path::new(&output_dir).exists() {
                fs::create_dir_all(&output_dir)?;
            }

            println!("Unpacking '{}' to '{}'...", input, output_dir);

            // Call the unpack function with output directory
            mdz_rs::unpack(&input, Some(&output_dir))?;

            println!("Successfully unpacked files to {}", output_dir);
        }
    }

    Ok(())
}