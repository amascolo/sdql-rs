use super::lexer::{DictHint, Spanned};
use super::r#type::Type;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr<'src> {
    Sym(&'src str),
    Value(Value<'src>),
    Record(Vec<Pair<'src>>),
    Dict {
        map: Vec<Pair<'src>>,
        hint: Option<DictHint>,
    },
    Let {
        lhs: &'src str,
        rhs: Box<Spanned<Self>>,
        cont: Box<Spanned<Self>>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Spanned<Self>>,
    },
    Binary {
        lhs: Box<Spanned<Self>>,
        op: BinaryOp,
        rhs: Box<Spanned<Self>>,
    },
    If {
        r#if: Box<Spanned<Self>>,
        then: Box<Spanned<Self>>,
        r#else: Option<Box<Spanned<Self>>>,
    },
    Field {
        expr: Box<Spanned<Self>>,
        field: &'src str,
    },
    Load {
        r#type: Type,
        path: &'src str,
    },
    Sum {
        key: Box<Spanned<Self>>,
        val: Box<Spanned<Self>>,
        head: Box<Spanned<Self>>,
        body: Box<Spanned<Self>>,
    },
    Range(Box<Spanned<Self>>),
    Concat(Box<Spanned<Self>>, Box<Spanned<Self>>),
    External {
        func: &'src str,
        args: Vec<Spanned<Self>>,
    },
    Promote {
        r#type: Type,
        expr: Box<Spanned<Self>>,
    },
    Unique(Box<Spanned<Self>>),
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Value<'src> {
    Bool(bool),
    Float(f64),
    Int(i32),
    Long(i64),
    String(&'src str),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pair<'src> {
    pub key: Spanned<Expr<'src>>,
    pub value: Spanned<Expr<'src>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Not,
    Neg,
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
            Self::String(x) => write!(f, "{x}"),
            Self::Bool(x) => write!(f, "{x}"),
            Self::Int(x) => write!(f, "{x}"),
            Self::Long(x) => write!(f, "{x}"),
            Self::Float(x) => write!(f, "{x}"),
        }
    }
}
