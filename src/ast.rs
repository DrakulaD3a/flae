#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Identifier(Identifier),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub identifier: Identifier,
    pub args: Option<Vec<Identifier>>,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug)]
pub struct Literal {
    pub value: String,
}
