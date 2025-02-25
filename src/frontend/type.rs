#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Type<'src> {
    String(Option<i32>),
    Bool,
    Int,
    Long,
    Real,
    Record(Vec<&'src str, Self>),
    Dict {
        key: Box<Self>,
        value: Box<Self>,
        hint: Option<crate::frontend::lexer::DictHint>,
    },
}
