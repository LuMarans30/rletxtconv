// main.rs
use clap::Parser;
use std::path::PathBuf;

use rletxtconv::{Result, convert_file};

#[derive(Parser, Debug)]
#[command(version, about = "Conway's Game of Life file format converter")]
struct Args {
    /// Input file path
    #[clap(short, long, required = true)]
    input: PathBuf,

    /// Output file path
    #[clap(short, long, required = true)]
    output: PathBuf,

    /// Force overwrite of existing output file
    #[clap(short, long, default_value = "false")]
    force: bool,
}

fn main() -> Result<()> {
    // Setup better panic messages in debug mode
    #[cfg(debug_assertions)]
    color_eyre::install().expect("Cannot initialize color_eyre");

    let args = Args::parse();

    if args.output.exists() && !args.force {
        println!("Output file {} already exists.", args.output.display());
        println!("Use --force to overwrite or specify a different output path.");
        return Ok(());
    }

    println!("Converting file: {}", args.input.display());
    convert_file(&args.input, &args.output, args.force)?;

    println!(
        "Conversion complete. Output written to: {}",
        args.output.display()
    );
    Ok(())
}
