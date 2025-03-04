#![allow(dead_code)] // TODO remove

use crate::inference::TypedExpr;

pub type ExprFMF<'src> = FilterMapFold<'src, TypedExpr<'src>>;

#[derive(Clone, Debug, PartialEq)]
pub enum FilterMapFold<'src, T> {
    Expr(T),
    FMF {
        op: OpFMF,
        args: Vec<&'src str>,
        expr: T,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum OpFMF {
    Filter,
    Map,
    Fold,
}
