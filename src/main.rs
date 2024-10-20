mod generator;
mod parser;
mod preprocessor;
mod tokenizer;

use crate::generator::generate_c;
use crate::parser::parse;
use crate::preprocessor::preprocessor;
use crate::tokenizer::tokenize;
use anyhow::Result;
use clap::Parser;
use preprocessor::PreprocessorConfig;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// A simple Brainfuck compiler
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input Brainfuck file
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    /// Output C file
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Include directories for preprocessor (can pass multiple directories)
    #[arg(short='d', long, value_name = "DIRS", num_args = 1..)]
    include_dirs: Vec<PathBuf>,
}

fn save_to_file(filename: &str, content: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn main() -> Result<()> {
    // Parse command-line arguments
    let cli = Cli::parse();

    // Read the input Brainfuck file
    let buf: String = std::fs::read_to_string(&cli.input)?;

    // Tokenize the input
    let tokenized = tokenize(buf)?;

    // Parse the tokenized input
    let parsed = parse(&tokenized)?;

    // Preprocess the parsed input with the provided include directories
    let preprocessed = preprocessor(
        &parsed,
        &PreprocessorConfig {
            include_dirs: cli.include_dirs,
        },
    )?;

    // Generate C code
    let c_code = generate_c(&preprocessed)?;

    // Determine the output file name (use the provided output or default to "output.c")
    let output_file = cli.output.unwrap_or_else(|| PathBuf::from("output.c"));

    // Save the generated C code to the output file
    save_to_file(output_file.to_str().unwrap(), &c_code)?;

    Ok(())
}
