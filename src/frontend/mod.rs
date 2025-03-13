pub mod lexer;
mod parser;

use crate::ir::expr::Expr;
use chumsky::{error::Rich, input::Input, span::SimpleSpan, Parser};
use lexer::{lexer, Spanned, Token};
use parser::expr_parser;

#[macro_export]
macro_rules! rs {
    ($src:expr) => {{
        use $crate::backend::fmf::ExprFMF;
        use $crate::frontend::lexer::Spanned;
        use $crate::inference::{Typed, TypedExpr};
        use $crate::ir::expr::Expr;

        let src: &str = $src;
        let expr = Spanned::<Expr>::try_from(src).unwrap();
        let typed = Typed::<Spanned<TypedExpr>>::from(expr);
        let fmf = Typed::<Spanned<ExprFMF>>::from(typed);
        String::from(fmf)
    }};
}

#[macro_export]
macro_rules! sdql {
    ($src:expr) => {
        Spanned::<Expr>::try_from($src).unwrap()
    };
}

#[macro_export]
macro_rules! no_span {
    ($src:expr) => {{
        let src: &str = $src;
        let expr = Spanned::<Expr>::try_from(src).unwrap();
        let Spanned(unspanned, _span) = expr;
        unspanned
    }};
}

impl<'src> TryFrom<&'src str> for Spanned<Expr<'src>> {
    type Error = Vec<Rich<'src, Token<'src>, SimpleSpan>>;

    fn try_from(src: &'src str) -> Result<Self, Self::Error> {
        let tokens = lexer().parse(src).into_result().unwrap();
        let tks_slice: &'src [_] = unsafe { std::mem::transmute(tokens.as_slice()) }; // FIXME
        let eoi = src.len();
        let (expr, span) = expr_parser()
            .map_with(|ast, e| (ast, e.span()))
            .parse(tks_slice.map((eoi..eoi).into(), |Spanned(t, s)| (t, s)))
            .into_result()?;
        debug_assert_eq!(expr.1, span);
        Ok(expr)
    }
}
