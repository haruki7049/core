#[derive(Debug)]
pub struct AST(pub Vec<Constant>);

#[derive(Debug, Clone)]
pub struct Constant {
    pub name: Value,
    pub value: Value,
}

#[derive(Debug, Clone)]
pub enum Value {
    SExpression(Vec<Value>),
    Constant(Box<Constant>),
    List(Vec<Value>),
    Number(u64),
    String(String),
    Boolean(Boolean),
    Word(String),
    BuiltinWord(BuiltinWord),
}

#[derive(Debug, Clone)]
pub enum Boolean {
    T,
    Nil,
}

#[derive(Debug, Clone)]
pub enum BuiltinWord {
    Main,
    Cli,

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
