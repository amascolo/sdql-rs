#![allow(dead_code)]

use crate::frontend::lexer::Spanned;
use crate::ir::expr::{BinaryOp, DictEntry, Expr, External, RecordValue, UnaryOp};
use crate::ir::r#type::{DictHint, Field, RecordType, Type};
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
        vals: Vec<RecordValue<'src, Typed<'src, Spanned<Self>>>>,
    },
    Dict {
        map: Vec<DictEntry<Typed<'src, Spanned<Self>>>>,
        hint: Option<DictHint>,
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
        key: Typed<'src, Spanned<Box<Self>>>,
        val: Typed<'src, Spanned<Box<Self>>>,
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
}

type Ctx<'src> = im_rc::HashMap<&'src str, Type<'src>>;

pub fn infer<'src>(expr: Expr<'src>, ctx: &mut Ctx<'src>) -> Typed<'src, TypedExpr<'src>> {
    match expr {
        Expr::Sym { val } => Typed {
            val: TypedExpr::Sym { val },
            r#type: ctx.get(val).cloned().unwrap(),
        },
        Expr::Bool { val } => Typed {
            val: TypedExpr::Bool { val },
            r#type: Type::Bool,
        },
        Expr::Date { val } => Typed {
            val: TypedExpr::Date { val },
            r#type: Type::Date,
        },
        Expr::Int { val } => Typed {
            val: TypedExpr::Int { val },
            r#type: Type::Int,
        },
        Expr::Long { val } => Typed {
            val: TypedExpr::Long { val },
            r#type: Type::Long,
        },
        Expr::Real { val } => Typed {
            val: TypedExpr::Real { val },
            r#type: Type::Real,
        },
        Expr::String { val } => Typed {
            val: TypedExpr::String { val },
            r#type: Type::String { max_len: None },
        },
        Expr::Record { vals } => {
            let (record_types, record_vals): (Vec<_>, Vec<_>) = vals
                .into_iter()
                .map(|val| {
                    let RecordValue {
                        name,
                        val: Spanned(expr, span),
                    } = val;
                    let Typed { val, r#type } = infer(expr, ctx);
                    (
                        RecordType {
                            name: name.clone(),
                            r#type: r#type.clone(),
                        },
                        RecordValue {
                            name,
                            val: Typed {
                                val: Spanned(val, span),
                                r#type,
                            },
                        },
                    )
                })
                .unzip();
            let r#type = Type::Record(record_types);
            let val = TypedExpr::Record { vals: record_vals };
            Typed { val, r#type }
        }
        Expr::Dict { .. } => todo!(),
        Expr::Let { .. } => todo!(),
        Expr::Unary { op, expr } => {
            let Spanned(unspanned, span) = expr;
            let Typed { val: typed, r#type } = infer(*unspanned, ctx);
            let val = Spanned(Box::new(typed), span);
            let expr = Typed {
                val,
                r#type: r#type.clone(),
            };
            Typed {
                val: TypedExpr::Unary { op, expr },
                r#type,
            }
        }
        Expr::Binary { lhs, op, rhs } => {
            let Spanned(lhs_unspanned, lhs_span) = lhs;
            let Typed {
                val: lhs_typed,
                r#type: lhs_type,
            } = infer(*lhs_unspanned, ctx);
            let Spanned(rhs_unspanned, rhs_span) = rhs;
            let Typed {
                val: rhs_typed,
                r#type: rhs_type,
            } = infer(*rhs_unspanned, ctx);
            Typed {
                val: TypedExpr::Binary {
                    lhs: Typed {
                        val: Spanned(Box::new(lhs_typed), lhs_span),
                        r#type: lhs_type.clone(),
                    },
                    op,
                    rhs: Typed {
                        val: Spanned(Box::new(rhs_typed), rhs_span),
                        r#type: rhs_type.clone(),
                    },
                },
                r#type: promote(lhs_type, rhs_type),
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
            let Spanned(if_unspanned, if_span) = r#if;
            let Typed {
                val: if_typed,
                r#type: if_type,
            } = infer(*if_unspanned, ctx);
            let Spanned(then_unspanned, then_span) = then;
            let Typed {
                val: then_typed,
                r#type: then_type,
            } = infer(*then_unspanned, ctx);
            let Spanned(else_unspanned, else_span) = r#else;
            let Typed {
                val: else_typed,
                r#type: else_type,
            } = infer(*else_unspanned, ctx);
            Typed {
                val: TypedExpr::If {
                    r#if: Typed {
                        val: Spanned(Box::new(if_typed), if_span),
                        r#type: if_type,
                    },
                    then: Typed {
                        val: Spanned(Box::new(then_typed), then_span),
                        r#type: then_type.clone(),
                    },
                    r#else: Some(Typed {
                        val: Spanned(Box::new(else_typed), else_span),
                        r#type: else_type.clone(),
                    }),
                },
                r#type: promote(then_type, else_type),
            }
        }
        Expr::Field { expr, field } => {
            let Spanned(unspanned, _) = expr;
            let Typed { val, r#type } = infer(*unspanned, ctx);
            let Type::Record(vals) = r#type else { panic!() };
            Typed {
                val,
                r#type: vals
                    .into_iter()
                    .find(|rt| rt.name == field)
                    .map(|rt| rt.r#type)
                    .unwrap(),
            }
        }
        Expr::Get { .. } => todo!(),
        Expr::Load { r#type, path } => Typed {
            val: TypedExpr::Load {
                r#type: r#type.clone(),
                path,
            },
            r#type,
        },
        Expr::Sum { .. } => todo!(),
        Expr::Range { expr } => {
            let Spanned(unspanned, span) = expr;
            let Typed { val: typed, r#type } = infer(*unspanned, ctx);
            let val = TypedExpr::Range {
                expr: Typed {
                    val: Spanned(Box::new(typed), span),
                    r#type,
                },
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
            let Spanned(unspanned, span) = expr;
            let Typed { val: typed, r#type } = infer(*unspanned, ctx);
            let val = TypedExpr::Promote {
                promo,
                expr: Typed {
                    val: Spanned(Box::new(typed), span),
                    r#type: r#type.clone(),
                },
            };
            Typed { val, r#type }
        }
        Expr::Unique { expr } => {
            let Spanned(unspanned, span) = expr;
            let Typed { val: typed, r#type } = infer(*unspanned, ctx);
            let val = TypedExpr::Unique {
                expr: Typed {
                    val: Spanned(Box::new(typed), span),
                    r#type: r#type.clone(),
                },
            };
            Typed { val, r#type }
        }
    }
}

fn promote<'src>(t1: Type<'src>, t2: Type<'src>) -> Type<'src> {
    if t1 == t2 {
        return t1;
    }
    match (t1, t2) {
        (Type::Int, Type::Long) | (Type::Long, Type::Int) => Type::Long,
        (Type::Int | Type::Long, Type::Real) | (Type::Real, Type::Int | Type::Long) => Type::Real,
        (Type::Long | Type::Real, Type::Dict { key, val, hint })
        | (Type::Dict { key, val, hint }, Type::Long | Type::Real) => Type::Dict {
            key,
            val: Box::new(promote(Type::Real, *val)),
            hint,
        },
        (
            Type::Dict { key, val, hint },
            Type::Dict {
                key: k,
                val: v,
                hint: h,
            },
        ) if hint == h => Type::Dict {
            key: Box::new(promote(*key, *k)),
            val: Box::new(promote(*val, *v)),
            hint,
        },
        _ => panic!(),
    }
}
