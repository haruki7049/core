#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    SExpression(Vec<Token>),
    Word(String),
    Number(u64),
    String(String),
    Literal(Literal),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Literal {
    Cons,
    Car,
    Cdr,
    If,
    Lambda,
    Begin,
    Define,
    DefineSyntax,
    CallCc,
}
