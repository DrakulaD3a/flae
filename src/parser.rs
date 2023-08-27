use crate::{
    ast::{Expression, Identifier, LetStatement, Literal, Program, Statement},
    lexer::Lexer,
    token::Token,
};

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<String>,
}

impl Parser {
    /// Creates new parser
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,
            current_token: None,
            peek_token: None,
            errors: Vec::new(),
        };

        // Read two tokens so `current_token` and `peek_token` are both Some
        parser.next_token();
        parser.next_token();
        parser
    }

    /// Reads the next token into `peek_token` and sets `current_token` as the last `peek_token`
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    /// Parses program
    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while let Some(_) = &self.current_token {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
            self.next_token();
        }

        program
    }

    /// Parses individual statements
    fn parse_statement(&mut self) -> Option<Statement> {
        // TODO: Add all branches
        match &self.current_token {
            Some(Token::Let) => self.parse_let_statement(),
            Some(Token::Return) => self.parse_return_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        self.next_token();
        let identifier = if let Some(Token::Ident(name)) = &self.current_token {
            Identifier { name: name.clone() }
        } else {
            self.errors.push("Expected identifier".to_string());
            return None;
        };

        let mut args = Vec::new();
        self.next_token();
        match &self.current_token {
            Some(Token::Assign) => {}
            Some(Token::Ident(name)) => {
                args.push(Identifier { name: name.clone() });
                self.next_token();
                while let Some(Token::Ident(name)) = self.current_token.clone() {
                    self.next_token();
                    args.push(Identifier { name: name.clone() });
                }
            }
            _ => {
                self.errors
                    .push("Unexpected character after identifier".to_string());
                return None;
            }
        }

        // FIXME: Does only work for variables with set integer literal
        self.next_token();
        let expr = match &self.current_token {
            Some(Token::Int(value)) => Expression::Literal(Literal {
                value: value.clone(),
            }),
            _ => {
                self.errors.push("Expected integer".to_string());
                return None;
            }
        };

        Some(Statement::Let(LetStatement {
            identifier,
            args: if args.is_empty() { None } else { Some(args) },
            value: expr,
        }))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();
        match &self.current_token {
            Some(Token::Int(value)) => Some(Statement::Return(Expression::Literal(Literal {
                value: value.clone(),
            }))),
            Some(Token::Ident(name)) => {
                Some(Statement::Return(Expression::Identifier(Identifier {
                    name: name.clone(),
                })))
            }
            _ => None,
        }
    }
}

#[test]
fn test_multiple_let_statements() {
    let input = "let five = 5;
    let ten = 10;
    let foobar = 838383;
    let func a b = 8;#
    ";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    assert_eq!(
        program.statements.len(),
        4,
        "Program.statements does not contain 4 statements"
    );

    let tests = vec![
        ("five", None, "5"),
        ("ten", None, "10"),
        ("foobar", None, "838383"),
        (
            "func",
            Some(vec![
                Identifier {
                    name: "a".to_string(),
                },
                Identifier {
                    name: "b".to_string(),
                },
            ]),
            "8",
        ),
    ];

    for (i, (name, args, value)) in tests.iter().enumerate() {
        match &program.statements[i] {
            Statement::Let(let_statement) => {
                assert_eq!(let_statement.identifier.name, *name);
                assert_eq!(let_statement.args, *args);

                match &let_statement.value {
                    Expression::Literal(literal) => {
                        assert_eq!(literal.value, *value);
                    }
                    _ => panic!("Expected a literal"),
                }
            }
            _ => panic!("Expected a let statement"),
        }
    }
}

#[test]
fn test_multiple_return_statements() {
    let input = "return 5;
    return a;
    ";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    assert_eq!(
        program.statements.len(),
        2,
        "Program.statements does not contain 2 statements"
    );

    let tests = vec!["5", "a"];

    for (i, value) in tests.iter().enumerate() {
        match &program.statements[i] {
            Statement::Return(ret_statement) => match &ret_statement {
                Expression::Literal(literal) => {
                    assert_eq!(literal.value, *value);
                }
                Expression::Identifier(identifier) => {
                    assert_eq!(identifier.name, *value);
                }
                _ => panic!("Expected a literal"),
            },
            _ => panic!("Expected a let statement"),
        }
    }
}
