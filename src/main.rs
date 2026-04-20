use brainfuckrs::interpreter::Simulator;
use brainfuckrs::tokenizer::tokenize;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

/// A Simple Brainfuck Interpreter
#[derive(Parser, Debug)]
struct Args {
    /// Path to the Brainfuck file
    #[arg(value_name = "FILE")]
    input: PathBuf,
}

fn main() {
    let args = Args::parse();

    match fs::read_to_string(&args.input) {
        Ok(content) => {
            let tokens = tokenize(content);
            Simulator::new(tokens).simulate();
        }
        Err(error) => {
            eprintln!("Error reading {}: {}", args.input.display(), error);
        }
    }
}
