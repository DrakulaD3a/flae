use std::io::Write;
use crate::lexer::Lexer;

const PROMPT: &str = ">>>";

/// Starts the repl
pub fn start() {
    loop {
        print!("{PROMPT}");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut lexer = Lexer::new(&input);
        while let Some(token) = lexer.next_token() {
            println!("{token:?}");
        }
    }
}
