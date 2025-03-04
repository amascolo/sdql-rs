#![allow(dead_code)]
pub mod lexer;
mod parser;

use crate::ir::expr::Expr;
use chumsky::{input::Input, Parser};
use lexer::{lexer, Spanned};
use parser::expr_parser;

impl From<&str> for Expr<'_> {
    fn from(src: &str) -> Self {
        let (tokens, _lex_errs) = lexer().parse(src).into_output_errors();
        let tokens = tokens.unwrap();

        let tks_slice: &'static [_] = unsafe { std::mem::transmute(tokens.as_slice()) };

        let (ast, _parse_errs) = expr_parser()
            .map_with(|ast, e| (ast, e.span()))
            .parse(tks_slice.map((src.len()..src.len()).into(), |Spanned(t, s)| (t, s)))
            .into_output_errors();

        let (Spanned(expr, _), _) = ast.unwrap();
        expr
    }
}
