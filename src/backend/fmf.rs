use crate::frontend::lexer::Spanned;
use crate::inference::{Typed, TypedExpr};
use crate::ir::expr::{BinaryOp, DictEntry, External, RecordValue, UnaryOp};
use crate::ir::r#type::{DictHint, Field, Type};
use crate::runtime::Date;
use im_rc::vector;
use std::fmt;

#[allow(dead_code)] // TODO remove after using External
#[derive(Clone, Debug, PartialEq)]
pub enum ExprFMF<'src> {
    Sym {
        val: &'src str,
    },
    Bool {
        val: bool,
    },
    Date {
        val: Date,
    },
    Real {
        val: f64,
    },
    Int {
        val: i32,
    },
    Long {
        val: i64,
    },
    String {
        val: &'src str,
        max_len: Option<i64>,
    },
    Record {
        vals: Vec<RecordValue<'src, Typed<'src, Spanned<Self>>>>,
    },
    Dict {
        map: Vec<DictEntry<Typed<'src, Spanned<Self>>, Typed<'src, Spanned<Self>>>>,
        hint: Option<DictHint>,
    },
    Dom {
        expr: Typed<'src, Spanned<Box<Self>>>,
    },
    Let {
        lhs: &'src str,
        rhs: Typed<'src, Spanned<Box<Self>>>,
        cont: Typed<'src, Spanned<Box<Self>>>,
    },
    Unary {
        op: UnaryOp,
        expr: Typed<'src, Spanned<Box<Self>>>,
    },
    Binary {
        lhs: Typed<'src, Spanned<Box<Self>>>,
        op: BinaryOp,
        rhs: Typed<'src, Spanned<Box<Self>>>,
    },
    If {
        r#if: Typed<'src, Spanned<Box<Self>>>,
        then: Typed<'src, Spanned<Box<Self>>>,
        r#else: Option<Typed<'src, Spanned<Box<Self>>>>,
    },
    Field {
        expr: Typed<'src, Spanned<Box<Self>>>,
        field: Field<'src>,
    },
    Get {
        lhs: Typed<'src, Spanned<Box<Self>>>,
        rhs: Typed<'src, Spanned<Box<Self>>>,
    },
    Load {
        r#type: Type<'src>,
        path: &'src str,
    },
    Sum {
        key: &'src str,
        val: &'src str,
        head: Typed<'src, Spanned<Box<Self>>>,
        body: Typed<'src, Spanned<Box<Self>>>,
    },
    Range {
        expr: Typed<'src, Spanned<Box<Self>>>,
    },
    Concat {
        lhs: Typed<'src, Spanned<Box<Self>>>,
        rhs: Typed<'src, Spanned<Box<Self>>>,
    },
    External {
        func: External,
        args: Vec<Typed<'src, Spanned<Self>>>,
    },
    Promote {
        promo: Type<'src>,
        expr: Typed<'src, Spanned<Box<Self>>>,
    },
    Unique {
        expr: Typed<'src, Spanned<Box<Self>>>,
    },
    FMF {
        op: OpFMF,
        args: im_rc::Vector<&'src str>,
        inner: Typed<'src, Spanned<Box<Self>>>,
        cont: Option<Typed<'src, Spanned<Box<Self>>>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum OpFMF {
    Filter,
    Map,
    Fold,
}

impl<'src> From<Typed<'src, Spanned<ExprFMF<'src>>>> for ExprFMF<'src> {
    fn from(expr: Typed<'src, Spanned<ExprFMF<'src>>>) -> Self {
        let Typed { val, r#type: _ } = expr;
        let Spanned(unspanned, _span) = val;
        unspanned
    }
}

impl fmt::Display for ExprFMF<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        TypedExpr::from(self.clone()).fmt(f)
    }
    // TODO
    // fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //     match self {
    //         ExprFMF::FMF {
    //             op,
    //             args,
    //             inner,
    //             cont,
    //         } => {
    //             write!(
    //                 f,
    //                 ".{op}(|{}| {inner} ){}",
    //                 args.iter().join(","),
    //                 cont.as_ref().map(|c| format!(".{c}")).unwrap_or_default()
    //             )
    //         }
    //     }
    // }
}

impl fmt::Display for OpFMF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpFMF::Filter => "filter",
            OpFMF::Map => "map",
            OpFMF::Fold => "fold",
        }
        .fmt(f)
    }
}

impl<'src> From<Typed<'src, Spanned<TypedExpr<'src>>>> for Typed<'src, Spanned<ExprFMF<'src>>> {
    fn from(expr: Typed<'src, Spanned<TypedExpr<'src>>>) -> Self {
        from(expr, &Ctx::new())
    }
}

impl<'src> From<Typed<'src, Spanned<Box<TypedExpr<'src>>>>>
    for Typed<'src, Spanned<Box<ExprFMF<'src>>>>
{
    fn from(expr: Typed<'src, Spanned<Box<TypedExpr<'src>>>>) -> Self {
        let expr: Typed<Spanned<TypedExpr>> = expr.map(Spanned::unboxed);
        Typed::from(expr).map(Spanned::boxed)
    }
}

type Ctx<'src> = im_rc::Vector<&'src str>;

#[allow(unreachable_code)] // TODO remove
fn from<'src>(
    expr: Typed<'src, Spanned<TypedExpr<'src>>>,
    ctx: &Ctx<'src>,
) -> Typed<'src, Spanned<ExprFMF<'src>>> {
    let r#type = expr.r#type.clone(); // TODO avoid clone
    let span = expr.val.1;
    expr.map(|spanned| {
        spanned.map(|expr| {
            match expr {
                TypedExpr::Sum { .. } if !ctx.is_empty() => unreachable!(),
                TypedExpr::Sum {
                    key,
                    val: "_",
                    head:
                        Typed {
                            val: Spanned(range, span),
                            r#type,
                        },
                    body,
                } if ctx.is_empty() && matches!(*range, TypedExpr::Range { .. }) => {
                    let head = Typed {
                        val: Spanned(range, span),
                        r#type,
                    }
                    .into();
                    let ctx = ctx + &vector![key];
                    let body = from(body.map(Spanned::unboxed), &ctx).map(Spanned::boxed);
                    ExprFMF::Sum {
                        key,
                        val: "_",
                        head,
                        body,
                    }
                }
                TypedExpr::Sum {
                    key,
                    val,
                    head,
                    body,
                } => {
                    let ctx = ctx + &vector![key, val];
                    let head = head.into();
                    let body = from(body.map(Spanned::unboxed), &ctx).map(Spanned::boxed);
                    ExprFMF::Sum {
                        key,
                        val,
                        head,
                        body,
                    }
                }
                TypedExpr::If {
                    r#if,
                    then,
                    r#else: None,
                } if !ctx.is_empty() => ExprFMF::FMF {
                    op: OpFMF::Filter,
                    inner: r#if.into(),
                    cont: Some(from(then.map(Spanned::unboxed), &ctx).map(Spanned::boxed)),
                    args: ctx.clone(),
                },
                expr @ TypedExpr::If {
                    r#if: _,
                    then: _,
                    r#else: Some(_),
                } if !ctx.is_empty() => {
                    let inner = Typed {
                        val: Spanned(expr, span).boxed(),
                        r#type,
                    }
                    .into();
                    ExprFMF::FMF {
                        op: OpFMF::Map,
                        args: ctx.clone(),
                        inner,
                        cont: None,
                    }
                }
                TypedExpr::Let { lhs, rhs, cont } if !ctx.is_empty() => {
                    let ctx_inner = ctx + &vector![lhs];
                    ExprFMF::FMF {
                        op: OpFMF::Map,
                        inner: rhs.into(),
                        cont: Some(
                            from(cont.map(Spanned::unboxed), &ctx_inner).map(Spanned::boxed),
                        ),
                        args: ctx.clone(),
                    }
                }
                expr @ TypedExpr::Dict { .. } if !ctx.is_empty() => {
                    let inner = Typed {
                        val: Spanned(expr, span).boxed(),
                        r#type,
                    }
                    .into();
                    ExprFMF::FMF {
                        op: OpFMF::Fold,
                        args: ctx.clone(),
                        inner,
                        cont: None,
                    }
                }
                expr if !ctx.is_empty() => {
                    let inner = Typed {
                        val: Spanned(expr, span).boxed(),
                        r#type,
                    }
                    .into();
                    ExprFMF::FMF {
                        op: OpFMF::Map,
                        args: ctx.clone(),
                        inner,
                        cont: None,
                    }
                }
                // in all other cases - leave as is
                TypedExpr::Sym { val } => ExprFMF::Sym { val },
                TypedExpr::Bool { val } => ExprFMF::Bool { val },
                TypedExpr::Date { val } => ExprFMF::Date { val },
                TypedExpr::Real { val } => ExprFMF::Real { val },
                TypedExpr::Int { val } => ExprFMF::Int { val },
                TypedExpr::Long { val } => ExprFMF::Long { val },
                TypedExpr::String { val, max_len } => ExprFMF::String { val, max_len },
                TypedExpr::Record { vals } => ExprFMF::Record {
                    vals: vals.into_iter().map(|rv| rv.map(|e| e.into())).collect(),
                },
                TypedExpr::Dict { map, hint } => ExprFMF::Dict {
                    map: map.into_iter().map(|d| d.map(|e| e.into())).collect(),
                    hint,
                },
                TypedExpr::Dom { expr } => ExprFMF::Dom { expr: expr.into() },
                TypedExpr::Let { lhs, rhs, cont } => ExprFMF::Let {
                    lhs,
                    rhs: rhs.into(),
                    cont: cont.into(),
                },
                TypedExpr::Unary { op, expr } => ExprFMF::Unary {
                    op,
                    expr: expr.into(),
                },
                TypedExpr::Binary { lhs, op, rhs } => ExprFMF::Binary {
                    lhs: lhs.into(),
                    op,
                    rhs: rhs.into(),
                },
                TypedExpr::If { r#if, then, r#else } => ExprFMF::If {
                    r#if: r#if.into(),
                    then: then.into(),
                    r#else: r#else.map(|r#else| r#else.into()),
                },
                TypedExpr::Field { expr, field } => ExprFMF::Field {
                    expr: expr.into(),
                    field,
                },
                TypedExpr::Get { lhs, rhs } => ExprFMF::Get {
                    lhs: lhs.into(),
                    rhs: rhs.into(),
                },
                TypedExpr::Load { r#type, path } => ExprFMF::Load { r#type, path },
                TypedExpr::Range { expr } => ExprFMF::Range { expr: expr.into() },
                TypedExpr::Concat { lhs, rhs } => ExprFMF::Concat {
                    lhs: lhs.into(),
                    rhs: rhs.into(),
                },
                TypedExpr::External { func, args } => ExprFMF::External {
                    func,
                    args: args.into_iter().map(|arg| arg.into()).collect(),
                },
                TypedExpr::Promote { promo, expr } => ExprFMF::Promote {
                    promo,
                    expr: expr.into(),
                },
                TypedExpr::Unique { expr } => ExprFMF::Unique { expr: expr.into() },
            }
        })
    })
}

// backward conversions

impl<'src> From<ExprFMF<'src>> for TypedExpr<'src> {
    fn from(expr: ExprFMF<'src>) -> Self {
        match expr {
            ExprFMF::Sym { val } => TypedExpr::Sym { val },
            ExprFMF::Bool { val } => TypedExpr::Bool { val },
            ExprFMF::Date { val } => TypedExpr::Date { val },
            ExprFMF::Real { val } => TypedExpr::Real { val },
            ExprFMF::Int { val } => TypedExpr::Int { val },
            ExprFMF::Long { val } => TypedExpr::Long { val },
            ExprFMF::String { val, max_len } => TypedExpr::String { val, max_len },
            ExprFMF::Record { vals } => TypedExpr::Record {
                vals: vals.into_iter().map(|rv| rv.map(Typed::from)).collect(),
            },
            ExprFMF::Dict { map, hint } => TypedExpr::Dict {
                map: map.into_iter().map(|d| d.map(Typed::from)).collect(),
                hint,
            },
            ExprFMF::Dom { expr } => TypedExpr::Dom { expr: expr.into() },
            ExprFMF::Let { lhs, rhs, cont } => TypedExpr::Let {
                lhs,
                rhs: rhs.into(),
                cont: cont.into(),
            },
            ExprFMF::Unary { op, expr } => TypedExpr::Unary {
                op,
                expr: expr.into(),
            },
            ExprFMF::Binary { lhs, op, rhs } => TypedExpr::Binary {
                lhs: lhs.into(),
                op,
                rhs: rhs.into(),
            },
            ExprFMF::If { r#if, then, r#else } => TypedExpr::If {
                r#if: r#if.into(),
                then: then.into(),
                r#else: r#else.map(|r#else| r#else.into()),
            },
            ExprFMF::Field { expr, field } => TypedExpr::Field {
                expr: expr.into(),
                field,
            },
            ExprFMF::Get { lhs, rhs } => TypedExpr::Get {
                lhs: lhs.into(),
                rhs: rhs.into(),
            },
            ExprFMF::Load { r#type, path } => TypedExpr::Load { r#type, path },
            ExprFMF::Sum {
                key,
                val,
                head,
                body,
            } => TypedExpr::Sum {
                key,
                val,
                head: head.into(),
                body: body.into(),
            },
            ExprFMF::Range { expr } => TypedExpr::Range { expr: expr.into() },
            ExprFMF::Concat { lhs, rhs } => TypedExpr::Concat {
                lhs: lhs.into(),
                rhs: rhs.into(),
            },
            ExprFMF::External { func, args } => TypedExpr::External {
                func,
                args: args.into_iter().map(|arg| arg.into()).collect(),
            },
            ExprFMF::Promote { promo, expr } => TypedExpr::Promote {
                promo,
                expr: expr.into(),
            },
            ExprFMF::Unique { expr } => TypedExpr::Unique { expr: expr.into() },
            ExprFMF::FMF {
                op: OpFMF::Filter,
                args: _,
                inner: _,
                cont: None,
            } => unimplemented!(),
            ExprFMF::FMF {
                op: OpFMF::Filter,
                inner,
                cont: Some(cont),
                args: _,
            } => TypedExpr::If {
                r#if: inner.into(),
                then: cont.into(),
                r#else: None,
            },
            ExprFMF::FMF {
                op: OpFMF::Map,
                args: _,
                inner,
                cont: None,
            } => {
                let inner: Typed<Spanned<Box<TypedExpr>>> = inner.into();
                *inner.val.0
            }
            ExprFMF::FMF {
                op: OpFMF::Map,
                args: _,
                inner:
                    Typed {
                        val: Spanned(val, span),
                        r#type,
                    },
                cont: Some(cont),
            } => {
                // TODO remove clone
                let ExprFMF::FMF { args, .. } = &*val.clone() else {
                    unreachable!()
                };
                let val = Typed {
                    val: Spanned(val, span),
                    r#type,
                };
                let lhs = *args.last().unwrap();
                TypedExpr::Let {
                    lhs,
                    rhs: val.into(),
                    cont: cont.into(),
                }
            }
            ExprFMF::FMF {
                op: OpFMF::Fold,
                args: _,
                inner,
                cont: None,
            } => {
                let inner: Typed<Spanned<Box<TypedExpr>>> = inner.into();
                *inner.val.0
            }
            expr @ ExprFMF::FMF { .. } => todo!("{expr:?}"),
        }
    }
}

impl<'src> From<Typed<'src, Spanned<ExprFMF<'src>>>> for Typed<'src, Spanned<TypedExpr<'src>>> {
    fn from(expr: Typed<'src, Spanned<ExprFMF<'src>>>) -> Self {
        expr.map(|expr| expr.map(TypedExpr::from))
    }
}

impl<'src> From<Typed<'src, Spanned<Box<ExprFMF<'src>>>>>
    for Typed<'src, Spanned<Box<TypedExpr<'src>>>>
{
    fn from(expr: Typed<'src, Spanned<Box<ExprFMF<'src>>>>) -> Self {
        let expr: Typed<Spanned<ExprFMF>> = expr.map(Spanned::unboxed);
        Typed::from(expr).map(Spanned::boxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::expr::Expr;
    use crate::sdql;

    #[test]
    fn tpch_q3() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/q3.sdql"));
        let typed = Typed::from(sdql!(src));
        let fmf = Typed::<Spanned<ExprFMF>>::from(typed.clone());
        assert_eq!(Typed::from(fmf), typed);
    }

    #[test]
    fn tpch_q6() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/q6.sdql"));
        let typed = Typed::from(sdql!(src));
        let fmf = Typed::<Spanned<ExprFMF>>::from(typed.clone());
        assert_eq!(Typed::from(fmf), typed);
    }
}
