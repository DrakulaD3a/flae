mod token;
mod lexer;
mod repl;
mod parser;
mod ast;

fn main() {
    println!("Welcome to the flae programming language!");
    println!("Feel free to type in commands");
    repl::start();
}
