use crate::frontend::lexer::Spanned;
use crate::ir::expr::{BinaryOp, DictEntry, Expr, External, RecordValue, UnaryOp};
use crate::ir::r#type::{DictHint, Field, RecordType, Type};
use crate::runtime::Date;
use derive_more::Display;
use std::fmt;

#[derive(Clone, Debug, Display, PartialEq)]
#[display("{val}")]
pub struct Typed<'src, T> {
    pub val: T,
    pub r#type: Type<'src>,
}
impl<'src, T> Typed<'src, T> {
    pub fn map<U, F>(self, f: F) -> Typed<'src, U>
    where
        F: FnOnce(T) -> U,
    {
        Typed {
            val: f(self.val),
            r#type: self.r#type,
        }
    }
}

#[allow(dead_code)] // TODO remove after using External
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
}

impl<'src> fmt::Display for TypedExpr<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Expr::from(self.clone()).fmt(f) // TODO
    }
}

impl<'src> From<Spanned<Expr<'src>>> for Typed<'src, Spanned<TypedExpr<'src>>> {
    fn from(expr: Spanned<Expr<'src>>) -> Self {
        let Spanned(unspanned, span) = expr;
        let Typed { val, r#type } = infer(unspanned, &Ctx::new());
        Typed {
            val: Spanned(val, span),
            r#type,
        }
    }
}

type Ctx<'src> = im_rc::HashMap<&'src str, Type<'src>>;

fn infer_spanned<'src>(
    expr: Spanned<Box<Expr<'src>>>,
    ctx: &Ctx<'src>,
) -> Typed<'src, Spanned<Box<TypedExpr<'src>>>> {
    let Spanned(unspanned, span) = expr;
    let Typed { val, r#type } = infer(*unspanned, ctx);
    let val = Spanned(val, span).boxed();
    Typed { val, r#type }
}

fn infer<'src>(expr: Expr<'src>, ctx: &Ctx<'src>) -> Typed<'src, TypedExpr<'src>> {
    match expr {
        Expr::Sym { val } => Typed {
            val: TypedExpr::Sym { val },
            r#type: ctx
                .get(val)
                .cloned()
                .expect(&format!("\"{val}\" not found")),
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
        Expr::String { val, max_len } => Typed {
            val: TypedExpr::String { val, max_len },
            r#type: Type::String { max_len },
        },
        Expr::Record { vals } => {
            let (record_types, record_vals) = vals
                .into_iter()
                .map(|val| {
                    let RecordValue { name, val } = val;
                    let val = infer_spanned(val.boxed(), ctx).map(Spanned::unboxed);
                    (
                        RecordType {
                            name: name.clone(),
                            r#type: val.r#type.clone(),
                        },
                        RecordValue { name, val },
                    )
                })
                .unzip();
            Typed {
                val: TypedExpr::Record { vals: record_vals },
                r#type: Type::Record(record_types),
            }
        }
        Expr::Dict { map, hint } => {
            let map: Vec<_> = map
                .into_iter()
                .map(|DictEntry { key, val }| DictEntry {
                    key: infer_spanned(key.boxed(), ctx).map(Spanned::unboxed),
                    val: infer_spanned(val.boxed(), ctx).map(Spanned::unboxed),
                })
                .collect();
            let (key_type, val_type) = map
                .iter()
                .map(|DictEntry { key, val }| (key.r#type.clone(), val.r#type.clone()))
                .reduce(|(k1, v1), (k2, v2)| (promote(k1, k2), promote(v1, v2)))
                .unwrap();
            Typed {
                r#type: Type::Dict {
                    key: Box::new(key_type),
                    val: Box::new(val_type),
                    hint: hint.clone(),
                },
                val: TypedExpr::Dict { map, hint },
            }
        }
        Expr::Dom { expr } => {
            let expr = infer_spanned(expr, ctx);
            match &expr.r#type {
                Type::Dict { key, .. } => Typed {
                    r#type: Type::Set((**key).clone()),
                    val: TypedExpr::Dom { expr },
                },
                _ => panic!(),
            }
        }
        Expr::Let { lhs, rhs, cont } => {
            let rhs = infer_spanned(rhs, ctx);
            let new_ctx = ctx.update(lhs, rhs.r#type.clone());
            let cont = infer_spanned(cont, &new_ctx);
            Typed {
                r#type: cont.r#type.clone(),
                val: TypedExpr::Let { lhs, rhs, cont },
            }
        }
        Expr::Unary { op, expr } => {
            let expr = infer_spanned(expr, ctx);
            Typed {
                r#type: expr.r#type.clone(),
                val: TypedExpr::Unary { op, expr },
            }
        }
        Expr::Binary {
            lhs,
            op: op @ (BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div),
            rhs,
        } => {
            let lhs = infer_spanned(lhs, ctx);
            let rhs = infer_spanned(rhs, ctx);
            Typed {
                r#type: promote(lhs.r#type.clone(), rhs.r#type.clone()),
                val: TypedExpr::Binary { lhs, op, rhs },
            }
        }
        Expr::Binary {
            lhs,
            op:
                op @ (BinaryOp::Eq
                | BinaryOp::NotEq
                | BinaryOp::Less
                | BinaryOp::Great
                | BinaryOp::LessEq
                | BinaryOp::GreatEq
                | BinaryOp::And
                | BinaryOp::Or),
            rhs,
        } => {
            let lhs = infer_spanned(lhs, ctx);
            let rhs = infer_spanned(rhs, ctx);
            Typed {
                r#type: Type::Bool,
                val: TypedExpr::Binary { lhs, op, rhs },
            }
        }
        Expr::If { r#if, then, r#else } => {
            let r#if = infer_spanned(r#if, ctx);
            let then = infer_spanned(then, ctx);
            let r#else = r#else.map(|r#else| infer_spanned(r#else, ctx));
            Typed {
                r#type: r#else
                    .as_ref()
                    .map(|r#else| promote(then.r#type.clone(), r#else.r#type.clone()))
                    .unwrap_or_else(|| then.r#type.clone()),
                val: TypedExpr::If { r#if, then, r#else },
            }
        }
        Expr::Field { expr, field } => {
            let expr = infer_spanned(expr, ctx);
            let Type::Record(vals) = &expr.r#type else {
                panic!()
            };
            Typed {
                r#type: vals
                    .iter()
                    .find(|rt| rt.name == field)
                    .cloned()
                    .map(|rt| rt.r#type)
                    .unwrap(),
                val: TypedExpr::Field { expr, field },
            }
        }
        Expr::Get { lhs, rhs } => {
            let lhs = infer_spanned(lhs, ctx);
            let rhs = infer_spanned(rhs, ctx);
            Typed {
                r#type: match (&lhs.r#type, &rhs.r#type) {
                    (Type::Record(vals), Type::Int | Type::Long) => {
                        let n: usize = match *rhs.val.0 {
                            TypedExpr::Int { val } => val.try_into().unwrap(),
                            TypedExpr::Long { val } => val.try_into().unwrap(),
                            _ => unimplemented!(),
                        };
                        &vals[n].r#type
                    }
                    (Type::Dict { key, val, .. }, _) if rhs.r#type == **key => val,
                    _ => panic!(),
                }
                .clone(),
                val: TypedExpr::Get { lhs, rhs },
            }
        }
        Expr::Load { r#type, path } => Typed {
            val: TypedExpr::Load {
                r#type: r#type.clone(),
                path,
            },
            r#type,
        },
        Expr::Sum {
            key,
            val,
            head,
            body,
        } => {
            let head = infer_spanned(head, ctx);
            let local_ctx = match &head.r#type {
                Type::Dict {
                    key: kt, val: vt, ..
                } => ctx.update(key, *kt.clone()).update(val, *vt.clone()),
                _ => panic!(),
            };
            let body = infer_spanned(body, &local_ctx);
            Typed {
                r#type: body.r#type.clone(),
                val: TypedExpr::Sum {
                    key,
                    val,
                    head,
                    body,
                },
            }
        }
        Expr::Range { expr } => {
            let expr = infer_spanned(expr, ctx);
            Typed {
                val: TypedExpr::Range { expr },
                r#type: Type::Set(Type::Int),
            }
        }
        Expr::Concat { lhs, rhs } => {
            let lhs = infer_spanned(lhs, ctx);
            let rhs = infer_spanned(rhs, ctx);
            Typed {
                r#type: Type::Record(RecordType::concat(
                    if let Type::Record(lhs) = &lhs.r#type {
                        lhs.clone()
                    } else {
                        panic!()
                    },
                    if let Type::Record(rhs) = &rhs.r#type {
                        rhs.clone()
                    } else {
                        panic!()
                    },
                )),
                val: TypedExpr::Concat { lhs, rhs },
            }
        }
        Expr::External { func, args: _ } => match func {},
        Expr::Promote { promo, expr } => {
            let expr = infer_spanned(expr, ctx);
            Typed {
                r#type: expr.r#type.clone(),
                val: TypedExpr::Promote { promo, expr },
            }
        }
        Expr::Unique { expr } => {
            let expr = infer_spanned(expr, ctx);
            Typed {
                r#type: expr.r#type.clone(),
                val: TypedExpr::Unique { expr },
            }
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
        (t1, t2) => panic!("can't promote: \"{t1}\" \"{t2}\""),
    }
}

impl<'src> From<TypedExpr<'src>> for Expr<'src> {
    fn from(expr: TypedExpr<'src>) -> Self {
        match expr {
            TypedExpr::Sym { val } => Expr::Sym { val },
            TypedExpr::Bool { val } => Expr::Bool { val },
            TypedExpr::Date { val } => Expr::Date { val },
            TypedExpr::Real { val } => Expr::Real { val },
            TypedExpr::Int { val } => Expr::Int { val },
            TypedExpr::Long { val } => Expr::Long { val },
            TypedExpr::String { val, max_len } => Expr::String { val, max_len },
            TypedExpr::Record { vals } => Expr::Record {
                vals: vals.into_iter().map(|rv| rv.map(Spanned::from)).collect(),
            },
            TypedExpr::Dict { map, hint } => Expr::Dict {
                map: map.into_iter().map(|d| d.map(Spanned::from)).collect(),
                hint,
            },
            TypedExpr::Dom { expr } => Expr::Dom { expr: expr.into() },
            TypedExpr::Let { lhs, rhs, cont } => Expr::Let {
                lhs,
                rhs: rhs.into(),
                cont: cont.into(),
            },
            TypedExpr::Unary { op, expr } => Expr::Unary {
                op,
                expr: expr.into(),
            },
            TypedExpr::Binary { lhs, op, rhs } => Expr::Binary {
                lhs: lhs.into(),
                op,
                rhs: rhs.into(),
            },
            TypedExpr::If { r#if, then, r#else } => Expr::If {
                r#if: r#if.into(),
                then: then.into(),
                r#else: r#else.map(|r#else| r#else.into()),
            },
            TypedExpr::Field { expr, field } => Expr::Field {
                expr: expr.into(),
                field,
            },
            TypedExpr::Get { lhs, rhs } => Expr::Get {
                lhs: lhs.into(),
                rhs: rhs.into(),
            },
            TypedExpr::Load { r#type, path } => Expr::Load { r#type, path },
            TypedExpr::Sum {
                key,
                val,
                head,
                body,
            } => Expr::Sum {
                key,
                val,
                head: head.into(),
                body: body.into(),
            },
            TypedExpr::Range { expr } => Expr::Range { expr: expr.into() },
            TypedExpr::Concat { lhs, rhs } => Expr::Concat {
                lhs: lhs.into(),
                rhs: rhs.into(),
            },
            TypedExpr::External { func, args } => Expr::External {
                func,
                args: args.into_iter().map(|arg| arg.into()).collect(),
            },
            TypedExpr::Promote { promo, expr } => Expr::Promote {
                promo,
                expr: expr.into(),
            },
            TypedExpr::Unique { expr } => Expr::Unique { expr: expr.into() },
        }
    }
}

impl<'src> From<Typed<'src, Spanned<TypedExpr<'src>>>> for Spanned<Expr<'src>> {
    fn from(expr: Typed<'src, Spanned<TypedExpr<'src>>>) -> Self {
        let Typed { val, r#type: _ } = expr;
        let Spanned(unspanned, span) = val;
        Spanned(unspanned.into(), span)
    }
}

impl<'src> From<Typed<'src, Spanned<Box<TypedExpr<'src>>>>> for Spanned<Box<Expr<'src>>> {
    fn from(expr: Typed<'src, Spanned<Box<TypedExpr<'src>>>>) -> Self {
        Spanned::<Expr>::from(expr.map(Spanned::unboxed)).boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sdql;

    #[test]
    fn tpch_q3() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/q3.sdql"));
        let expr = sdql!(src);
        assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
    }

    #[test]
    fn tpch_q6() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/q6.sdql"));
        let expr = sdql!(src);
        assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
    }
}
