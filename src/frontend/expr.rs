use super::lexer::{DictHint, Spanned};
use super::r#type::Type;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr<'src> {
    Value(Value<'src>),
    Record(Vec<Pair<'src>>),
    Dict(Dict<'src>),
    Local(&'src str),
    Let(&'src str, Box<Spanned<Self>>, Box<Spanned<Self>>),
    Not(Box<Spanned<Self>>),
    Neg(Box<Spanned<Self>>),
    Binary(Box<Spanned<Self>>, BinaryOp, Box<Spanned<Self>>),
    If(Box<Spanned<Self>>, Box<Spanned<Self>>, Box<Spanned<Self>>),
    Sum(Box<Sum<'src>>),
    Field {
        expr: Box<Spanned<Self>>,
        field: &'src str,
    },
    Load {
        r#type: Option<Type>,
        path: &'src str,
    },
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Value<'src> {
    Null,
    Bool(bool),
    Num(f64),
    Str(&'src str),
    List(Vec<Self>),
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Dict<'src> {
    pub map: Vec<Pair<'src>>,
    pub hint: Option<DictHint>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pair<'src> {
    pub key: Spanned<Expr<'src>>,
    pub value: Spanned<Expr<'src>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Sum<'src> {
    pub key: Spanned<Expr<'src>>,
    pub value: Spanned<Expr<'src>>,
    pub head: Spanned<Expr<'src>>,
    pub body: Spanned<Expr<'src>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    Less,
    Great,
    LessEq,
    GreatEq,
    And,
    Or,
}

impl std::fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Bool(x) => write!(f, "{}", x),
            Self::Num(x) => write!(f, "{}", x),
            Self::Str(x) => write!(f, "{}", x),
            Self::List(xs) => write!(
                f,
                "[{}]",
                xs.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}
