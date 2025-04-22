use std::marker::ConstParamTy;

#[derive(Clone, Copy, PartialEq, Eq, ConstParamTy)]
pub enum Addition {
    Plus,
    Min,
    Max,
}
