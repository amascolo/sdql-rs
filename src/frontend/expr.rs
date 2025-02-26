#![allow(dead_code)]

use super::lexer::{DictHint, Spanned};
use super::r#type::Type;
use crate::frontend::r#type::Field;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr<'src> {
    Sym(&'src str),
    Bool(bool),
    Float(f64),
    Int(i32),
    Long(i64),
    String(&'src str),
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
        func: &'src str, // TODO enum
        args: Vec<Spanned<Self>>,
    },
    Promote {
        r#type: Spanned<Type<'src>>,
        expr: Box<Spanned<Self>>,
    },
    Unique(Box<Spanned<Self>>),
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

impl fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Sym(x) => write!(f, "{x}"),
            Self::Bool(x) => write!(f, "{x}"),
            Self::Float(x) => write!(f, "{x}"),
            Self::Int(x) => write!(f, "{x}"),
            Self::Long(x) => write!(f, "{x}"),
            Self::String(x) => write!(f, "\"{x}\""),
            Self::Record(fields) => write!(
                f,
                "<{}>",
                fields
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Dict { map, hint } => {
                let map_str = map
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                match hint {
                    Some(hint) => write!(f, "@{hint} {{{map_str}}}"),
                    None => write!(f, "{{{map_str}}}"),
                }
            }
            Self::Let { lhs, rhs, cont } => write!(f, "let {} = {} in {}", lhs, rhs.0, cont.0),
            Self::Unary { op, expr } => write!(f, "{}({})", op, expr.0),
            Self::Binary { lhs, op, rhs } => write!(f, "{}({}, {})", op, lhs.0, rhs.0),
            Self::If { r#if, then, r#else } => match r#else {
                Some(r#else) => write!(f, "if {} then {} else {}", r#if.0, then.0, r#else.0),
                None => write!(f, "if {} then {}", r#if.0, then.0),
            },
            Self::Field { expr, field } => write!(f, "{}.{}", expr.0, field),
            Self::Get(lhs, rhs) => write!(f, "{}({})", lhs.0, rhs.0),
            Self::Load { r#type, path } => write!(f, "load[{}]({})", r#type, path),
            Self::Sum {
                key,
                val,
                head,
                body,
            } => write!(f, "sum(<{}, {}> <- {}) {}", key.0, val.0, head.0, body.0),
            Self::Range(e) => write!(f, "range({})", e.0),
            Self::Concat(lhs, rhs) => write!(f, "concat({}, {})", lhs.0, rhs.0),
            Self::External { func, args } => write!(
                f,
                "ext(`{}`, {})",
                func,
                args.iter()
                    .map(|a| a.0.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Promote { r#type, expr } => write!(f, "promote[{}]({})", r#type.0, expr.0),
            Self::Unique(e) => write!(f, "unique({})", e.0),
        }
    }
}

impl fmt::Display for RecordValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.name.0, self.val.0)
    }
}

impl fmt::Display for DictEntry<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.key.0, self.val.0)
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
