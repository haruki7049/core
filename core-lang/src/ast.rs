#[derive(Debug)]
pub struct AST(pub Vec<Constant>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constant {
    pub name: Value,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    SExpression(Vec<Value>),
    Constant(Box<Constant>),
    List(Vec<Value>),
    Number(u64),
    Lambda((Vec<Value>, Vec<Value>)),
    String(String),
    Boolean(Boolean),
    Word(String),
    BuiltinWord(BuiltinWord),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Boolean {
    T,
    Nil,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Constant {
    pub fn name(&self) -> Value {
        self.name.clone()
    }

    pub fn value(&self) -> Value {
        self.value.clone()
    }
}

impl Value {
    pub fn car(&self) -> Result<Value, Box<dyn std::error::Error>> {
        match self {
            Value::SExpression(v) => Ok(v[0].clone()),
            Value::List(v) => Ok(v[0].clone()),
            _ => panic!(),
        }
    }

    pub fn cdr(&self) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        let mut value: Vec<Value> = match self {
            Value::SExpression(v) => v.clone(),
            _ => todo!(),
        };

        value.reverse();
        value.pop();
        value.reverse();

        Ok(value)
    }

    pub fn is_atom(&self) -> bool {
        matches!(
            self,
            Value::Number(_) | Value::String(_) | Value::Boolean(_) | Value::BuiltinWord(_)
        )
    }
}
