use crate::backend::fmf::ExprFMF;
use crate::frontend::lexer::Spanned;
use crate::inference::Typed;
use crate::ir::expr::Expr;

pub mod codegen;
pub mod fmf;

impl From<Spanned<Expr<'_>>> for String {
    fn from(expr: Spanned<Expr<'_>>) -> Self {
        String::from(ExprFMF::from(Typed::from(expr)))
    }
}
