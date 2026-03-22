use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::Path;
use std::fs;

#[derive(Parser)]
#[command(name = "mdz")]
#[command(about = "A CLI tool for MDZ (Markdown Zip) format")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Pack a Markdown file and its assets into an MDZ archive
    Pack {
        /// Input Markdown file
        input: String,

        /// Output MDZ file (optional, defaults to input file with .mdz extension)
        #[arg(short, long)]
        output: Option<String>,
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
        Commands::Pack { input, output } => {
            // Validate input file exists
            if !Path::new(&input).exists() {
                eprintln!("Error: Input file '{}' does not exist", input);
                std::process::exit(1);
            }

            // Generate output file name based on input file if not provided
            let output_file = if let Some(output) = output {
                output
            } else {
                let input_path = Path::new(&input);
                input_path.with_extension("mdz").to_str().unwrap().to_string()
            };

            // Create output directory if needed
            let output_path = Path::new(&output_file);
            if let Some(parent) = output_path.parent()
                && !parent.exists() {
                    fs::create_dir_all(parent)?;
                }

            println!("Packing '{}' into '{}'...", input, output_file);

            // Call the pack function
            mdz_rs::pack(&input, &output_file).await?;

            println!("Successfully packed to {}", output_file);
        }

        Commands::Unpack { input, output } => {
            // Validate input file exists
            let input_path = Path::new(&input);
            if !input_path.exists() {
                eprintln!("Error: Input file '{}' does not exist", input);
                std::process::exit(1);
            }

            // Determine output directory
            let output_dir = if let Some(output) = output {
                output
            } else {
                // Default to MDZ file's parent directory
                input_path
                    .parent()
                    .and_then(|p| p.to_str())
                    .unwrap_or(".")
                    .to_string()
            };

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