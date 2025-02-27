#![allow(dead_code)]

use crate::frontend::lexer::Spanned;
use crate::ir::expr::{BinaryOp, DictEntry, Expr, External, RecordValue, UnaryOp};
use crate::ir::r#type::{DictHint, Field, Type};
use time::Date;

#[derive(Clone, Debug, PartialEq)]
pub struct Typed<'src, T> {
    val: T,
    r#type: Type<'src>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypedExpr<'src> {
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
    },
    Record {
        vals: Vec<RecordValue<'src>>,
    },
    Dict {
        map: Vec<DictEntry<'src>>,
        hint: Option<DictHint>,
    },
    Let {
        lhs: &'src str,
        rhs: Box<Typed<'src, Spanned<Self>>>,
        cont: Box<Typed<'src, Spanned<Self>>>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Typed<'src, Spanned<Self>>>,
    },
    Binary {
        lhs: Box<Typed<'src, Spanned<Self>>>,
        op: BinaryOp,
        rhs: Box<Typed<'src, Spanned<Self>>>,
    },
    If {
        r#if: Box<Typed<'src, Spanned<Self>>>,
        then: Box<Typed<'src, Spanned<Self>>>,
        r#else: Option<Box<Typed<'src, Spanned<Self>>>>,
    },
    Field {
        expr: Box<Typed<'src, Spanned<Self>>>,
        field: Field<'src>,
    },
    Get {
        lhs: Box<Typed<'src, Spanned<Self>>>,
        rhs: Box<Typed<'src, Spanned<Self>>>,
    },
    Load {
        r#type: Type<'src>,
        path: &'src str,
    },
    Sum {
        key: Box<Typed<'src, Spanned<Self>>>,
        val: Box<Typed<'src, Spanned<Self>>>,
        head: Box<Typed<'src, Spanned<Self>>>,
        body: Box<Typed<'src, Spanned<Self>>>,
    },
    Range {
        expr: Box<Typed<'src, Spanned<Self>>>,
    },
    Concat {
        lhs: Box<Typed<'src, Spanned<Self>>>,
        rhs: Box<Typed<'src, Spanned<Self>>>,
    },
    External {
        func: External,
        args: Vec<Typed<'src, Spanned<Self>>>,
    },
    Promote {
        promo: Type<'src>,
        expr: Box<Typed<'src, Spanned<Self>>>,
    },
    Unique {
        expr: Box<Typed<'src, Spanned<Self>>>,
    },
}

type Ctx<'src> = im_rc::HashMap<&'src str, Type<'src>>;

pub fn infer<'src>(expr: &Expr<'src>, ctx: &mut Ctx<'src>) -> Typed<'src, TypedExpr<'src>> {
    match expr {
        Expr::Sym { val } => Typed {
            val: TypedExpr::Sym { val },
            r#type: ctx.get(val).cloned().unwrap(),
        },
        &Expr::Bool { val } => Typed {
            val: TypedExpr::Bool { val },
            r#type: Type::Bool,
        },
        &Expr::Date { val } => Typed {
            val: TypedExpr::Date { val },
            r#type: Type::Date,
        },
        &Expr::Int { val } => Typed {
            val: TypedExpr::Int { val },
            r#type: Type::Int,
        },
        &Expr::Long { val } => Typed {
            val: TypedExpr::Long { val },
            r#type: Type::Long,
        },
        &Expr::Real { val } => Typed {
            val: TypedExpr::Real { val },
            r#type: Type::Real,
        },
        Expr::String { val } => Typed {
            val: TypedExpr::String { val },
            r#type: Type::String { max_len: None },
        },
        Expr::Record { .. } => todo!(),
        Expr::Dict { .. } => todo!(),
        Expr::Let { .. } => todo!(),
        Expr::Unary { op, expr } => {
            let (unspanned, span) = Box::into_inner(expr.clone());
            let Typed { val: typed, r#type } = infer(&unspanned, ctx);
            let val = TypedExpr::Unary {
                op: *op,
                expr: Box::new(Typed {
                    val: (typed, span),
                    r#type: r#type.clone(),
                }),
            };
            Typed { val, r#type }
        }
        Expr::Binary { lhs, op, rhs } => {
            let (lhs_unspanned, lhs_span) = Box::into_inner(lhs.clone());
            let Typed {
                val: lhs_typed,
                r#type: lhs_type,
            } = infer(&lhs_unspanned, ctx);
            let (rhs_unspanned, rhs_span) = Box::into_inner(rhs.clone());
            let Typed {
                val: rhs_typed,
                r#type: rhs_type,
            } = infer(&rhs_unspanned, ctx);
            Typed {
                val: TypedExpr::Binary {
                    lhs: Box::new(Typed {
                        val: (lhs_typed, lhs_span),
                        r#type: lhs_type.clone(),
                    }),
                    op: *op,
                    rhs: Box::new(Typed {
                        val: (rhs_typed, rhs_span),
                        r#type: rhs_type.clone(),
                    }),
                },
                r#type: promote(&lhs_type, &rhs_type),
            }
        }
        Expr::If {
            r#if: _,
            then: _,
            r#else: None,
        } => todo!(),
        Expr::If {
            r#if,
            then,
            r#else: Some(r#else),
        } => {
            let (if_unspanned, if_span) = Box::into_inner(r#if.clone());
            let Typed {
                val: if_typed,
                r#type: if_type,
            } = infer(&if_unspanned, ctx);
            let (then_unspanned, then_span) = Box::into_inner(then.clone());
            let Typed {
                val: then_typed,
                r#type: then_type,
            } = infer(&then_unspanned, ctx);
            let (else_unspanned, else_span) = Box::into_inner(r#else.clone());
            let Typed {
                val: else_typed,
                r#type: else_type,
            } = infer(&else_unspanned, ctx);
            Typed {
                val: TypedExpr::If {
                    r#if: Box::new(Typed {
                        val: (if_typed, if_span),
                        r#type: if_type,
                    }),
                    then: Box::new(Typed {
                        val: (then_typed, then_span),
                        r#type: then_type.clone(),
                    }),
                    r#else: Some(Box::new(Typed {
                        val: (else_typed, else_span),
                        r#type: else_type.clone(),
                    })),
                },
                r#type: promote(&then_type, &else_type),
            }
        }
        Expr::Field { .. } => todo!(),
        Expr::Get { .. } => todo!(),
        Expr::Load { .. } => todo!(),
        Expr::Sum { .. } => todo!(),
        Expr::Range { expr } => {
            let (unspanned, span) = Box::into_inner(expr.clone());
            let Typed { val: typed, r#type } = infer(&unspanned, ctx);
            let val = TypedExpr::Range {
                expr: Box::new(Typed {
                    val: (typed, span),
                    r#type,
                }),
            };
            Typed {
                val,
                r#type: Type::Dict {
                    key: Box::new(Type::Int),
                    val: Box::new(Type::Bool),
                    hint: None,
                },
            }
        }
        Expr::Concat { .. } => todo!(),
        Expr::External { .. } => todo!(),
        Expr::Promote { promo, expr } => {
            let (unspanned, span) = Box::into_inner(expr.clone());
            let Typed { val: typed, r#type } = infer(&unspanned, ctx);
            let val = TypedExpr::Promote {
                promo: promo.clone(),
                expr: Box::new(Typed {
                    val: (typed, span),
                    r#type: r#type.clone(),
                }),
            };
            Typed { val, r#type }
        }
        Expr::Unique { expr } => {
            let (unspanned, span) = Box::into_inner(expr.clone());
            let Typed { val: typed, r#type } = infer(&unspanned, ctx);
            let val = TypedExpr::Unique {
                expr: Box::new(Typed {
                    val: (typed, span),
                    r#type: r#type.clone(),
                }),
            };
            Typed { val, r#type }
        }
    }
}

fn promote<'src>(t1: &Type<'src>, t2: &Type<'src>) -> Type<'src> {
    match (t1, t2) {
        (Type::Int, Type::Long) | (Type::Long, Type::Int) => Type::Long,
        (Type::Int | Type::Long, Type::Real) | (Type::Real, Type::Int | Type::Long) => Type::Real,
        (Type::Long | Type::Real, Type::Dict { key, val, hint })
        | (Type::Dict { key, val, hint }, Type::Long | Type::Real) => Type::Dict {
            key: key.clone(),
            val: Box::new(promote(&Type::Real, val)),
            hint: hint.clone(),
        },
        (
            Type::Dict { key, val, hint },
            Type::Dict {
                key: k,
                val: v,
                hint: h,
            },
        ) if hint == h => Type::Dict {
            key: Box::new(promote(key, k)),
            val: Box::new(promote(val, v)),
            hint: hint.clone(),
        },
        _ if t1 == t2 => t1.clone(),
        _ => panic!("Cannot promote incompatible types"),
    }
}
