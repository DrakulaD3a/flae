use crate::lexer::Lexer;
use std::io::{self, Write};

const PROMPT: &str = ">>>";

/// Starts the repl
pub fn start() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        write!(&stdout, "{PROMPT}").unwrap();
        stdout.flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let mut lexer = Lexer::new(&input);
        while let Some(token) = lexer.next_token() {
            println!("{token:?}");
        }
    }
}
