use crate::backend::fmf::ExprFMF;
use crate::frontend::lexer::Spanned;
use crate::inference::Typed;
use crate::ir::expr::Expr;

pub mod codegen;
pub mod fmf;

impl From<Spanned<Expr<'_>>> for String {
    fn from(expr: Spanned<Expr<'_>>) -> Self {
        let typed = Typed::from(expr);
        let fmf = Typed::<Spanned<ExprFMF>>::from(typed);
        fmf.to_string()
    }
}
