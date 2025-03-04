#![allow(dead_code)] // TODO remove

use crate::frontend::lexer::Spanned;
use crate::inference::{Typed, TypedExpr};
use im_rc;
use im_rc::vector;

pub type ExprFMF<'src> = FilterMapFold<'src, Typed<'src, Spanned<TypedExpr<'src>>>>;

#[derive(Clone, Debug, PartialEq)]
pub enum FilterMapFold<'src, T> {
    Expr(T),
    Range {
        range: T,
        cont: Box<Self>,
    },
    FMF {
        op: OpFMF,
        args: im_rc::Vector<&'src str>,
        inner: T,
        cont: Option<Box<Self>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum OpFMF {
    Filter,
    Map,
    Fold,
}

impl<'src> From<Typed<'src, Spanned<TypedExpr<'src>>>> for ExprFMF<'src> {
    fn from(expr: Typed<'src, Spanned<TypedExpr<'src>>>) -> Self {
        from(expr, &Ctx::new())
    }
}

type Ctx<'src> = im_rc::Vector<&'src str>;

fn from<'src>(expr: Typed<'src, Spanned<TypedExpr<'src>>>, ctx: &Ctx<'src>) -> ExprFMF<'src> {
    let Typed { val, r#type } = expr;
    let Spanned(unspanned, span) = val.unboxed();
    match unspanned {
        TypedExpr::Sum {
            key,
            val: "_",
            head:
                Typed {
                    val: Spanned(range, span),
                    r#type,
                },
            body,
        } if matches!(*range, TypedExpr::Range { .. }) => {
            let range = Typed {
                val: Spanned(range, span).unboxed(),
                r#type,
            };
            let ctx = ctx + &vector![key];
            let cont = Box::new(from(body.map(Spanned::unboxed), &ctx));
            ExprFMF::Range { range, cont }
        }
        TypedExpr::If {
            r#if: _,
            then: _,
            r#else: Some(_),
        } => {
            let val = Spanned(unspanned, span);
            let expr = Typed { val, r#type };
            ExprFMF::FMF {
                op: OpFMF::Map,
                args: ctx.clone(),
                inner: expr,
                cont: None,
            }
        }
        TypedExpr::If {
            r#if,
            then,
            r#else: None,
        } => ExprFMF::FMF {
            op: OpFMF::Filter,
            inner: r#if.map(Spanned::unboxed),
            cont: Some(Box::new(from(then.map(Spanned::unboxed), &ctx))),
            args: ctx.clone(),
        },
        TypedExpr::Let { lhs, rhs, cont } => {
            let ctx = ctx + &vector![lhs];
            ExprFMF::FMF {
                op: OpFMF::Map,
                inner: rhs.map(Spanned::unboxed),
                cont: Some(Box::new(from(cont.map(Spanned::unboxed), &ctx))),
                args: ctx,
            }
        }
        _ => {
            let val = Spanned(unspanned, span);
            let expr = Typed { val, r#type };
            ExprFMF::Expr(expr)
        }
    }
}

// // TODO simplify something like this?
// //  let val = val.unboxed().map(ExprFMF::from).boxed();
// fn from_typed<'src>(
//     expr: Typed<'src, Spanned<Box<TypedExpr<'src>>>>,
// ) -> Typed<'src, Spanned<Box<ExprFMF<'src>>>> {
//     let Typed { val, r#type } = expr;
//     let Spanned(unspanned, span) = val;
//     let unspanned = Box::new(ExprFMF::from(*unspanned));
//     let val = Spanned(unspanned, span);
//     Typed { val, r#type }
// }
