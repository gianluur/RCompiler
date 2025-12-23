use clap::Parser;
use std::{self, fs};

mod error;
use error::Diagnostic;

mod tokenizer;
use tokenizer::{Token, Tokenizer};

mod parser;
use parser::{Parser as MyParser};

fn get_source_code() -> (String, String) {
    #[derive(Parser, Debug)]
    #[command(author, about = "gianluur's compiler for his shell (RShell).", long_about = None)]
    struct Args {
        pub input: String,
    }

    let args: Args = Args::parse();
    println!("--- Compiler Settings ---");
    println!("Input File:  {}", args.input);
    println!("-------------------------");

    match fs::read_to_string(&args.input) {
        Ok(contents) => {
            (args.input, contents)
        },
        Err(e) => panic!("Error reading input file: {}", e),
    }
}

fn main() {
    let (file, contents) = get_source_code();

    println!("=== Tokenizer Start ===");
    let tokens: Vec<Token<'_>> = match Tokenizer::new(&contents).tokenize() {
        Ok(tokens) => {
            for token in &tokens { 
                println!("{}", token) 
            }
            tokens
        },
        Err(error) => {
            let diagnostic: Diagnostic = error.to_diagnostic(&file);
            diagnostic.print();
            return;
        }
    };
    println!("=== Tokenizer End ===");    

    println!();

    println!("=== Parser Start ===");
    match MyParser::new(tokens).parse() {
        Ok(statements) => {
            for statement in &statements { 
                println!("{}", statement) 
            }
            statements
        },
        Err(error) => {
            let diagnostic: Diagnostic = error.to_diagnostic(&file);
            diagnostic.print();
            return;
        }
    };
    println!("=== Parser End ===");    
}