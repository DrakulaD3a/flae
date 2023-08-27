use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    /// Creates new lexer and sets it up for use
    pub fn new(input: &str) -> Self {
        let input: Vec<char> = input.chars().collect();

        let mut lex = Self {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };

        lex.read_char();
        lex
    }

    /// Return the next token
    pub fn next_token(&mut self) -> Option<Token> {
        self.consume_whitespace();

        let token = match self.ch {
            'a'..='z' | 'A'..='Z' | '_' => match self.read_identifier().as_str() {
                "let" => Some(Token::Let),
                "true" => Some(Token::True),
                "false" => Some(Token::False),
                "if" => Some(Token::If),
                "then" => Some(Token::Then),
                "else" => Some(Token::Else),
                "return" => Some(Token::Return),
                ident => Some(Token::Ident(ident.into())),
            },
            '0'..='9' => Some(Token::Int(self.read_int())),
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Asterisk),
            '/' => Some(Token::Slash),
            '=' => Some(if self.peek() == '=' {
                self.read_char();
                Token::Equal
            } else {
                Token::Assign
            }),
            '!' => Some(if self.peek() == '=' {
                self.read_char();
                Token::NotEqual
            } else {
                Token::Bang
            }),
            '<' => Some(if self.peek() == '=' {
                self.read_char();
                Token::LessOrEqual
            } else {
                Token::LessThan
            }),
            '>' => Some(if self.peek() == '=' {
                self.read_char();
                Token::GreaterOrEqual
            } else {
                Token::GreaterThan
            }),
            ',' => Some(Token::Comma),
            ';' => Some(Token::Semicolon),
            '#' => Some(Token::Hash),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '\0' | _ => None,
        };

        self.read_char();
        token
    }

    /// Sets `self.ch` as the next character and advances `self.read_position` by 1
    fn read_char(&mut self) {
        self.ch = if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        };

        self.position = self.read_position;
        self.read_position += 1;
    }

    /// Returns string starting at current `self.ch` and ending at first non alphanumeric character, consumes all those characters
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_alphanumeric() || self.ch == '_' {
            self.read_char();
        }

        self.read_position -= 1;
        self.input[position..self.position].iter().collect()
    }

    /// Returns integer starting at current `self.ch` and ending at first non numeric character, consumes all those characters
    fn read_int(&mut self) -> String {
        let position = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }

        self.read_position -= 1;
        self.input[position..self.position]
            .iter()
            .collect::<String>()
    }

    /// Consumes characters until non whitespace character occurs
    fn consume_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    /// Returns the next character without consuming it
    fn peek(&self) -> char {
        if self.read_position < self.input.len() {
            self.input[self.read_position]
        } else {
            '\0'
        }
    }
}

#[test]
fn test_next_token() {
    let input = "=+(),;";

    let expected = vec![
        Token::Assign,
        Token::Plus,
        Token::LParen,
        Token::RParen,
        Token::Comma,
        Token::Semicolon,
    ];

    let mut lexer = Lexer::new(input.into());

    let mut result = Vec::new();
    while let Some(token) = lexer.next_token() {
        result.push(token);
    }

    assert_eq!(result, expected);
}

#[test]
fn more_complex() {
    let input = "let five = 5;
    let ten = 10;
    
    let add a b = a + b;#

    let result = add five ten;

    !+-*/5;
    5 < 10 > 5;

    4 == 4 != 5;
    3 <= 4 >= 4;

    if 5 < 10
    then
        return true;
    else
        return false;
    ";

    let expected = vec![
        Token::Let,
        Token::Ident("five".into()),
        Token::Assign,
        Token::Int("5".into()),
        Token::Semicolon,
        Token::Let,
        Token::Ident("ten".into()),
        Token::Assign,
        Token::Int("10".into()),
        Token::Semicolon,
        Token::Let,
        Token::Ident("add".into()),
        Token::Ident("a".into()),
        Token::Ident("b".into()),
        Token::Assign,
        Token::Ident("a".into()),
        Token::Plus,
        Token::Ident("b".into()),
        Token::Semicolon,
        Token::Hash,
        Token::Let,
        Token::Ident("result".into()),
        Token::Assign,
        Token::Ident("add".into()),
        Token::Ident("five".into()),
        Token::Ident("ten".into()),
        Token::Semicolon,
        Token::Bang,
        Token::Plus,
        Token::Minus,
        Token::Asterisk,
        Token::Slash,
        Token::Int("5".into()),
        Token::Semicolon,
        Token::Int("5".into()),
        Token::LessThan,
        Token::Int("10".into()),
        Token::GreaterThan,
        Token::Int("5".into()),
        Token::Semicolon,
        Token::Int("4".into()),
        Token::Equal,
        Token::Int("4".into()),
        Token::NotEqual,
        Token::Int("5".into()),
        Token::Semicolon,
        Token::Int("3".into()),
        Token::LessOrEqual,
        Token::Int("4".into()),
        Token::GreaterOrEqual,
        Token::Int("4".into()),
        Token::Semicolon,
        Token::If,
        Token::Int("5".into()),
        Token::LessThan,
        Token::Int("10".into()),
        Token::Then,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::Else,
        Token::Return,
        Token::False,
        Token::Semicolon,
    ];

    let mut lexer = Lexer::new(input.into());

    let mut result = Vec::new();
    while let Some(token) = lexer.next_token() {
        result.push(token);
    }

    assert_eq!(result, expected);
}
