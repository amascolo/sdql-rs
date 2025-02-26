#![allow(dead_code)]

use super::lexer::{DictHint, Spanned};
use super::r#type::Type;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr<'src> {
    Sym(&'src str),
    Value(Value<'src>),
    Record(Vec<RecordValue<'src>>),
    Dict {
        map: Vec<DictEntry<'src>>,
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
        field: Field<'src>,
    },
    Get(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Load {
        r#type: Type<'src>,
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
        r#type: Spanned<Type<'src>>,
        expr: Box<Spanned<Self>>,
    },
    Unique(Box<Spanned<Self>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value<'src> {
    Bool(bool),
    Float(f64),
    Int(i32),
    Long(i64),
    String(&'src str),
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecordValue<'src> {
    pub name: Spanned<Field<'src>>,
    pub val: Spanned<Expr<'src>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DictEntry<'src> {
    pub key: Spanned<Expr<'src>>,
    pub val: Spanned<Expr<'src>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Field<'src>(&'src str);

impl<'src> From<&'src str> for Field<'src> {
    fn from(s: &'src str) -> Self {
        Field(s)
    }
}

impl fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::String(x) => write!(f, "{x}"),
            Self::Bool(x) => write!(f, "{x}"),
            Self::Int(x) => write!(f, "{x}"),
            Self::Long(x) => write!(f, "{x}"),
            Self::Float(x) => write!(f, "{x}"),
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Not => "!",
            Self::Neg => "-",
        }
        .fmt(f)
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Eq => "==",
            Self::NotEq => "!=",
            Self::Less => "<",
            Self::Great => ">",
            Self::LessEq => "<=",
            Self::GreatEq => ">=",
            Self::And => "&&",
            Self::Or => "||",
        }
        .fmt(f)
    }
}
