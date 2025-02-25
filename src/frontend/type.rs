use crate::frontend::lexer::DictHint;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Type {
    Bool,
    Int,
    Long,
    Real,
    String {
        max_len: Option<i32>,
    },
    Record(Vec<Self>),
    Dict {
        key: Box<Self>,
        value: Box<Self>,
        hint: Option<DictHint>,
    },
}

impl fmt::Display for Type {
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
            _ => todo!(),
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
