use {
    crate::ast::Statement,
    crate::error::Diagnostic,
    crate::interner::Interner,
    crate::parser::Parser as MyParser,
    crate::semantics::SemanticAnalyzer,
    crate::tokenizer::Tokenizer,
    crate::tokens::Token,
    clap::Parser,
    std::{self, fs, process::exit},
};

pub struct Compiler {
    filename: String,
    content: String,
    interner: Interner,
}

impl Compiler {
    pub fn new() -> Self {
        let (filename, content) = Self::get_source_code();
        let interner = Interner::new();

        Compiler {
            filename,
            content,
            interner,
        }
    }

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
            Ok(contents) => (args.input, contents),
            Err(e) => panic!("Error reading input file: {}", e),
        }
    }

    fn tokenize(&mut self) -> Vec<Token> {
        println!("=== Tokenizer Start ===");
        let tokens: Vec<Token> = match Tokenizer::new(&self.content, &mut self.interner).tokenize()
        {
            Ok(tokens) => {
                for token in &tokens {
                    let text = self.interner.lookup(token.span.literal);
                    println!(
                        "Kind: {:<15} | Literal: {}",
                        format!("{:?}", token.kind),
                        text
                    );
                }
                tokens
            }
            Err(error) => {
                let diagnostic: Diagnostic = error.to_diagnostic(&self.filename);
                diagnostic.print();
                exit(1);
            }
        };
        println!("=== Tokenizer End ===");
        return tokens;
    }

    fn parse(&self, tokens: Vec<Token>) -> Vec<Statement> {
        println!("=== Parser Start ===");
        let statements: Vec<Statement> = match MyParser::new(tokens).parse() {
            Ok(statements) => {
                for statement in &statements {
                    println!("{:#?}", statement)
                }
                println!();
                statements
            }
            Err(error) => {
                let diagnostic: Diagnostic = error.to_diagnostic(&self.filename);
                diagnostic.print();
                exit(1);
            }
        };
        println!("=== Parser End ===");
        return statements;
    }

    fn analyze(&mut self, statements: Vec<Statement>) {
        println!("=== Semantic analysys in place... ===");
        match SemanticAnalyzer::new(&mut self.interner).analyze(statements) {
            Ok(_) => println!("=== Semantic analysys End ==="),
            Err(error) => {
                let diagnostic: Diagnostic = error.to_diagnostic(&self.filename);
                diagnostic.print();
                return;
            }
        };
    }

    pub fn run(&mut self) {
        let tokens = self.tokenize();
        let statements = self.parse(tokens);
        self.analyze(statements);
    }
}
