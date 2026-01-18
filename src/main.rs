use clap::Parser;
use std::{self, fs};

mod error;
use error::Diagnostic;

mod tokenizer;
use tokenizer::{Token, Tokenizer};

mod parser;
use parser::{Parser as MyParser};

mod interner;
use interner::Interner;

mod semantics;
use semantics::SemanticAnalyzer;

mod ast;
use ast::Statement;

mod scope;

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
    let mut interner: Interner = interner::Interner::new();

    println!("=== Tokenizer Start ===");
    let tokens: Vec<Token> = match Tokenizer::new(&contents, &mut interner).tokenize() {
        Ok(tokens) => {
            for token in &tokens { 
                let text = interner.lookup(token.span.literal);
                println!("Kind: {:<15} | Literal: {}", format!("{:?}", token.kind), text);
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
    let statements: Vec<Statement> = match MyParser::new(tokens).parse() {
        Ok(statements) => {
            for statement in &statements { 
                println!("{:#?}", statement) 
            }
            println!();
            statements  
        },
        Err(error) => {
            let diagnostic: Diagnostic = error.to_diagnostic(&file);
            diagnostic.print();
            return;
        }
    };
    println!("=== Parser End ===");    

    println!();

    println!("=== Semantic analysys in place... ===");
    match SemanticAnalyzer::new(&statements).analyze() {
        Ok(_) => println!("=== Semantic analysys End ==="),
        Err(error) => {
            let diagnostic: Diagnostic = error.to_diagnostic(&file);
            diagnostic.print();
            return;
        }
    };


}