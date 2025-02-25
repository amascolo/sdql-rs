use crate::frontend::lexer::DictHint;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Type {
    String(Option<i32>),
    Bool,
    Int,
    Long,
    Real,
    Record(Vec<Self>),
    Dict {
        key: Box<Self>,
        value: Box<Self>,
        hint: Option<DictHint>,
    },
}
