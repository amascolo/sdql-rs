use crate::frontend::lexer::Spanned;
use crate::ir::expr::{BinaryOp, DictEntry, Expr, External, RecordValue, UnaryOp};
use crate::ir::r#type::{DictHint, Field, RecordType, Type};
use time::Date;

#[derive(Clone, Debug, PartialEq)]
pub struct Typed<'src, T> {
    val: T,
    r#type: Type<'src>,
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

type Ctx<'src> = im_rc::HashMap<&'src str, Type<'src>>;

impl<'src> From<Expr<'src>> for Typed<'src, TypedExpr<'src>> {
    fn from(expr: Expr<'src>) -> Self {
        infer(expr, &Ctx::new())
    }
}

fn infer<'src>(expr: Expr<'src>, ctx: &Ctx<'src>) -> Typed<'src, TypedExpr<'src>> {
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
        Expr::Binary { lhs, op, rhs } => {
            let lhs = infer_spanned(lhs, ctx);
            let rhs = infer_spanned(rhs, ctx);
            Typed {
                r#type: promote(lhs.r#type.clone(), rhs.r#type.clone()),
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
                        let n = match *lhs.val.0 {
                            TypedExpr::Int { val } => val.try_into().unwrap(),
                            TypedExpr::Long { val } => val.try_into().unwrap(),
                            _ => unimplemented!(),
                        };
                        vals.iter().map(|rt| &rt.r#type).nth(n).unwrap()
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

fn infer_spanned<'src>(
    expr: Spanned<Box<Expr<'src>>>,
    ctx: &Ctx<'src>,
) -> Typed<'src, Spanned<Box<TypedExpr<'src>>>> {
    let Spanned(unspanned, span) = expr;
    let Typed { val, r#type } = infer(*unspanned, ctx);
    let val = Spanned(val, span).boxed();
    Typed { val, r#type }
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
