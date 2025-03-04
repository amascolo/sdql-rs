#![allow(dead_code)]
pub mod lexer;
mod parser;

use crate::ir::expr::Expr;
use chumsky::{error::Rich, input::Input, span::SimpleSpan, Parser};
use lexer::{lexer, Spanned, Token};
use parser::expr_parser;

#[macro_export]
macro_rules! sdql {
    ($src:expr) => {{
        let src: &str = $src;
        crate::ir::expr::Expr::try_from(src).unwrap()
    }};
}

impl<'src> TryFrom<&'src str> for Expr<'src> {
    type Error = Vec<Rich<'src, Token<'src>, SimpleSpan>>;

    fn try_from(src: &'src str) -> Result<Self, Self::Error> {
        let tokens = lexer().parse(src).into_result().unwrap();
        let tks_slice: &'src [_] = unsafe { std::mem::transmute(tokens.as_slice()) }; // FIXME
        let eoi = (src.len()..src.len()).into();
        let (Spanned(expr, _), _) = expr_parser()
            .map_with(|ast, e| (ast, e.span()))
            .parse(tks_slice.map(eoi, |Spanned(t, s)| (t, s)))
            .into_result()?;
        Ok(expr)
    }
}
