mod ast;
mod compiler;
mod error;
mod interner;
mod parser;
mod scope;
mod semantics;
mod tokenizer;
mod tokens;

use compiler::Compiler;

fn main() {
    Compiler::new().run();
}
