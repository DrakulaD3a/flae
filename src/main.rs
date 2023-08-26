mod token;
mod lexer;
mod repl;

fn main() {
    println!("Welcome to the flae programming language!");
    println!("Feel free to type in commands");
    repl::start();
}
