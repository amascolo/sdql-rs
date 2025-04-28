use crate::frontend::lexer::Spanned;
use crate::ir::r#type::{DictHint, Field, Type};
use derive_more::Display;
use itertools::Itertools;
use sdql_runtime::Date;
use std::fmt;
use strum_macros::EnumString;

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
        max_len: Option<i64>,
    },
    Record {
        vals: Vec<RecordValue<'src, Spanned<Self>>>,
    },
    Dict {
        map: Vec<DictEntry<Spanned<Self>, Spanned<Self>>>,
        hint: Option<DictHint>,
    },
    Dom {
        expr: Spanned<Box<Self>>,
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
        op: BinOp,
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
    Decat {
        lhs: Vec<Field<'src>>,
        rhs: Spanned<Box<Self>>,
        cont: Spanned<Box<Self>>,
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

#[derive(Clone, Debug, strum_macros::Display, EnumString, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum External {
    #[strum(serialize = "FirstIndex")]
    FirstIndex,
    #[strum(serialize = "LastIndex")]
    LastIndex,
    #[strum(serialize = "StrContains")]
    StrContains,
    #[strum(serialize = "StrStartsWith")]
    StrStartsWith,
    #[strum(serialize = "StrEndsWith")]
    StrEndsWith,
    #[strum(serialize = "SubString")]
    SubString,
    #[strum(serialize = "Size")]
    Size,
    #[strum(serialize = "Year")]
    Year,
}

#[derive(Clone, Debug, Display, PartialEq)]
#[display("{name} = {val}")]
pub struct RecordValue<'src, T> {
    pub name: Field<'src>,
    pub val: T,
}
impl<'src, T> RecordValue<'src, T> {
    pub fn map<U, F>(self, f: F) -> RecordValue<'src, U>
    where
        F: FnOnce(T) -> U,
    {
        RecordValue {
            name: self.name,
            val: f(self.val),
        }
    }
}

#[derive(Clone, Debug, Display, PartialEq)]
#[display("{key} -> {val}")]
pub struct DictEntry<KT, VT> {
    pub key: KT,
    pub val: VT,
}
impl<T> DictEntry<T, T> {
    pub fn map<U, F>(self, f: F) -> DictEntry<U, U>
    where
        F: Fn(T) -> U,
    {
        DictEntry {
            key: f(self.key),
            val: f(self.val),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
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
            Self::String { val, max_len: None } => write!(f, "\"{val}\""),
            Self::String {
                val,
                max_len: Some(max_len),
            } => write!(f, "@varchar({max_len}) \"{val}\""),
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
            Self::Dom { expr } => write!(f, "dom({expr})"),
            Self::Let { lhs, rhs, cont } => write!(f, "let {lhs} = {rhs} in {cont}"),
            Self::Unary { op, expr } => write!(f, "{op}({expr})"),
            Self::Binary { lhs, op, rhs } => write!(f, "({lhs} {op} {rhs})"),
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
            Self::Decat { lhs, rhs, cont } => {
                let record = lhs.into_iter().join(", ");
                write!(f, "let <{record}> = {rhs} in {cont}")
            }
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

// TODO pretty printing with indentation
// impl<'src> Expr<'src> {
//     fn indented(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
//         todo!()
//     }
// }

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Not => "!",
            Self::Neg => "-",
        }
        .fmt(f)
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Eq => "==",
            Self::Ne => "!=",
            Self::Lt => "<",
            Self::Gt => ">",
            Self::Le => "<=",
            Self::Ge => ">=",
            Self::And => "&&",
            Self::Or => "||",
        }
        .fmt(f)
    }
}
