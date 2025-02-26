use crate::frontend::expr::Field;
use crate::frontend::lexer::DictHint;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Type<'src> {
    Bool,
    Int,
    Long,
    Real,
    String {
        max_len: Option<i32>,
    },
    Record(Vec<RecordType<'src>>),
    Dict {
        key: Box<Self>,
        val: Box<Self>,
        hint: Option<DictHint>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecordType<'src> {
    pub name: Field<'src>,
    pub r#type: Type<'src>,
}

impl fmt::Display for Type<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bool => write!(f, "bool"),
            Self::Int => write!(f, "int"),
            Self::Long => write!(f, "long"),
            Self::Real => write!(f, "real"),
            Self::String {
                max_len: Some(max_len),
            } => write!(f, "string({max_len})"),
            Self::String { max_len: None } => write!(f, "string"),
            Self::Record(fields) => {
                write!(
                    f,
                    "<{}>",
                    fields
                        .iter()
                        .map(|rt| format!("{}: {}", rt.name, rt.r#type))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Self::Dict { key, val, hint } => match hint {
                Some(hint) => write!(f, "@{hint} {{{key} -> {val}}}"),
                None => write!(f, "{{{key} -> {val}}}"),
            },
        }
    }
}

impl fmt::Display for DictHint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::HashDict => "hasdict",
            Self::SortDict => "sortdict",
            Self::SmallVecDict => "smallvecdict",
            Self::Vec => "vec",
        }
        .fmt(f)
    }
}
