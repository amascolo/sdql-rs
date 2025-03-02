use crate::frontend::lexer::Spanned;
use crate::ir::r#type::{DictHint, Field, Type};
use derive_more::Display;
use std::fmt;
use time::Date;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr<'src> {
    Sym {
        val: &'src str,
    },
    Bool {
        val: bool,
    },
    Date {
        val: Date,
    },
    Real {
        val: f64,
    },
    Int {
        val: i32,
    },
    Long {
        val: i64,
    },
    String {
        val: &'src str,
    },
    Record {
        vals: Vec<RecordValue<'src, Spanned<Self>>>,
    },
    Dict {
        map: Vec<DictEntry<Spanned<Self>, Spanned<Self>>>,
        hint: Option<DictHint>,
    },
    Let {
        lhs: &'src str,
        rhs: Spanned<Box<Self>>,
        cont: Spanned<Box<Self>>,
    },
    Unary {
        op: UnaryOp,
        expr: Spanned<Box<Self>>,
    },
    Binary {
        lhs: Spanned<Box<Self>>,
        op: BinaryOp,
        rhs: Spanned<Box<Self>>,
    },
    If {
        r#if: Spanned<Box<Self>>,
        then: Spanned<Box<Self>>,
        r#else: Option<Spanned<Box<Self>>>,
    },
    Field {
        expr: Spanned<Box<Self>>,
        field: Field<'src>,
    },
    Get {
        lhs: Spanned<Box<Self>>,
        rhs: Spanned<Box<Self>>,
    },
    Load {
        r#type: Type<'src>,
        path: &'src str,
    },
    Sum {
        key: &'src str,
        val: &'src str,
        head: Spanned<Box<Self>>,
        body: Spanned<Box<Self>>,
    },
    Range {
        expr: Spanned<Box<Self>>,
    },
    Concat {
        lhs: Spanned<Box<Self>>,
        rhs: Spanned<Box<Self>>,
    },
    External {
        func: External,
        args: Vec<Spanned<Self>>,
    },
    Promote {
        promo: Type<'src>,
        expr: Spanned<Box<Self>>,
    },
    Unique {
        expr: Spanned<Box<Self>>,
    },
}

#[derive(Clone, Debug, Display, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum External {}

#[derive(Clone, Debug, Display, PartialEq)]
#[display("{name} = {val}")]
pub struct RecordValue<'src, T> {
    pub name: Field<'src>,
    pub val: T,
}

#[derive(Clone, Debug, Display, PartialEq)]
#[display("{key} -> {val}")]
pub struct DictEntry<T, U> {
    pub key: T,
    pub val: U,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

impl<'src> Expr<'src> {
    #[allow(non_snake_case)]
    pub fn Set(vals: Vec<Spanned<Self>>) -> Self {
        Self::Dict {
            map: vals
                .into_iter()
                .map(|key @ Spanned(_, span)| DictEntry {
                    key,
                    val: Spanned(Expr::Bool { val: true }, (span.end..span.end).into()),
                })
                .collect::<Vec<_>>(),
            hint: None,
        }
    }
}

impl fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Sym { val } => write!(f, "{val}"),
            Self::Bool { val } => write!(f, "{val}"),
            Self::Date { val } => write!(f, "{val}"),
            Self::Real { val } => write!(f, "{val}"),
            Self::Int { val } => write!(f, "{val}"),
            Self::Long { val } => write!(f, "{val}"),
            Self::String { val } => write!(f, "\"{val}\""),
            Self::Record { vals } => write!(
                f,
                "<{}>",
                vals.iter()
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
            Self::Let { lhs, rhs, cont } => write!(f, "let {lhs} = {rhs} in {cont}"),
            Self::Unary { op, expr } => write!(f, "{op}({expr})"),
            Self::Binary { lhs, op, rhs } => write!(f, "{op}({lhs}, {rhs})"),
            Self::If { r#if, then, r#else } => match r#else {
                Some(r#else) => write!(f, "if {if} then {then} else {else}"),
                None => write!(f, "if {if} then {then}"),
            },
            Self::Field { expr, field } => write!(f, "{expr}.{field}"),
            Self::Get { lhs, rhs } => write!(f, "{lhs}({rhs})"),
            Self::Load { r#type, path } => write!(f, "load[{type}]({path})"),
            Self::Sum {
                key,
                val,
                head,
                body,
            } => write!(f, "sum(<{key}, {val}> <- {head}) {body}"),
            Self::Range { expr } => write!(f, "range({expr})"),
            Self::Concat { lhs, rhs } => write!(f, "concat({lhs}, {rhs})"),
            Self::External { func, args } => write!(
                f,
                "ext(`{func}`, {})",
                args.iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Promote { promo, expr } => write!(f, "promote[{promo}]({expr})"),
            Self::Unique { expr } => write!(f, "unique({expr})"),
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
