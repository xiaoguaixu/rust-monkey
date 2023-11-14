pub mod token;
mod lexer;
mod object;
#[macro_use]
pub mod util;
mod ast;
mod repl;
mod parser;
mod evaluator;
mod tester;


fn main() {
    repl::start();
}

