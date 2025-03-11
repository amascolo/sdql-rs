use super::fmf::{ExprFMF, OpFMF};
use crate::frontend::lexer::Spanned;
use crate::inference::Typed;
use crate::ir::expr::{BinaryOp, DictEntry};
use crate::ir::r#type::{DictHint, Type};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse2, parse_quote, BinOp, Error, ExprBinary, ExprField, ExprRange, Index, Member, RangeLimits,
};

impl From<ExprFMF<'_>> for String {
    fn from(expr: ExprFMF<'_>) -> Self {
        let tks: TokenStream = expr.into();
        let main_tks = quote! { fn main() { #tks } };
        let ast = parse2(main_tks).unwrap();
        prettyplease::unparse(&ast)
    }
}

impl<'src> From<Typed<'src, Spanned<ExprFMF<'src>>>> for String {
    fn from(expr: Typed<'src, Spanned<ExprFMF<'src>>>) -> Self {
        ExprFMF::from(expr).into()
    }
}

impl<'src> From<Typed<'src, Spanned<ExprFMF<'src>>>> for TokenStream {
    fn from(expr: Typed<'src, Spanned<ExprFMF<'src>>>) -> Self {
        ExprFMF::from(expr).into()
    }
}

impl<'src> From<Typed<'src, Spanned<Box<ExprFMF<'src>>>>> for TokenStream {
    fn from(expr: Typed<'src, Spanned<Box<ExprFMF<'src>>>>) -> Self {
        ExprFMF::from(expr.map(Spanned::unboxed)).into()
    }
}

impl From<ExprFMF<'_>> for TokenStream {
    fn from(expr: ExprFMF<'_>) -> Self {
        match expr {
            ExprFMF::Sym { val } => {
                let ident = Ident::new(val, Span::call_site());
                quote!(#ident)
            }
            ExprFMF::Bool { val } => quote! { #val },
            ExprFMF::Date { val } => {
                let val = val.to_string();
                quote!(date!(#val))
            }
            ExprFMF::Int { val } => quote! { #val },
            ExprFMF::Long { val } => quote! { #val },
            ExprFMF::Real { val } => quote! { #val },
            ExprFMF::String { val, max_len: None } => quote! { #val },
            ExprFMF::String {
                val: _,
                max_len: Some(_),
            } => todo!(),
            ExprFMF::Let { lhs, rhs, cont } => {
                let lhs_ident = syn::Ident::new(lhs, Span::call_site());
                let lhs_tks = quote! { #lhs_ident };
                let rhs_tks: TokenStream = rhs.into();
                let let_tks = quote! { let #lhs_tks = #rhs_tks };
                debug_assert!(matches!(parse2(let_tks.clone()), Ok(syn::Expr::Let(_))));
                let cont_tks: TokenStream = cont.into();
                quote! { #let_tks;  #cont_tks }
            }
            ExprFMF::Load { r#type, path } => {
                let Type::Record(vals) = r#type else {
                    unreachable!()
                };
                let tables: Vec<_> = vals
                    .into_iter()
                    .filter(|rt| rt.name != "size".into())
                    .map(|val| {
                        let r#type = match val.r#type {
                            Type::Dict {
                                key,
                                val,
                                hint: Some(DictHint::Vec),
                            } if matches!(*key, Type::Int) => *val,
                            _ => unreachable!(),
                        };
                        (val.name.into(), r#type.into())
                    })
                    .collect();
                let load = try_gen_load(&tables).unwrap();
                let tks = quote! { #load(#path) };
                debug_assert!(matches!(parse2(tks.clone()), Ok(syn::Expr::Call(_))));
                tks
            }
            ExprFMF::Sum {
                key: _,
                val: "_",
                head:
                    Typed {
                        val: Spanned(range, _span),
                        r#type: _,
                    },
                body,
            } if matches!(*range, ExprFMF::Range { .. }) => {
                let ExprFMF::Range { expr } = *range else {
                    unreachable!()
                };
                let expr = ExprFMF::from(expr.map(Spanned::unboxed));
                let expr: syn::Expr = parse2(expr.into()).unwrap();
                let expr = gen_range(expr);
                let body = ExprFMF::from(body.map(Spanned::unboxed));
                let body: TokenStream = body.into();
                quote! { (#expr)#body }
            }
            ExprFMF::FMF {
                op: OpFMF::Filter,
                args: _,
                inner: _,
                cont: None,
            } => unimplemented!(),
            ExprFMF::FMF {
                op: OpFMF::Filter,
                args,
                inner,
                cont: Some(cont),
            } => {
                let inner: TokenStream = inner.into();
                let cont = ExprFMF::from(cont.map(Spanned::unboxed));
                let cont: TokenStream = cont.into();
                let args = args.iter().map(|name| Ident::new(name, Span::call_site()));
                quote! {.filter(|#(#args),*| #inner)#cont}
            }
            ExprFMF::FMF {
                op: OpFMF::Map,
                args,
                inner,
                cont: None,
            } => {
                let args = args.iter().map(|name| Ident::new(name, Span::call_site()));
                let inner: TokenStream = inner.into();
                quote! {.map(|#(#args),*| #inner).sum()}
            }
            ExprFMF::Binary { lhs, op, rhs } => {
                let lhs = parse2(lhs.into()).unwrap();
                let rhs = parse2(rhs.into()).unwrap();
                let expr = syn::Expr::Binary(ExprBinary {
                    attrs: vec![],
                    left: Box::new(lhs),
                    op: op.into(),
                    right: Box::new(rhs),
                });
                quote! { #expr }
            }
            ExprFMF::Field { expr, field } => match expr.r#type {
                Type::Record(ref vals) => {
                    let index = vals.iter().position(|rt| rt.name == field).unwrap();
                    let index = index.try_into().unwrap();
                    let field = syn::Expr::Field(ExprField {
                        attrs: vec![],
                        base: Box::new(parse2(expr.into()).unwrap()),
                        dot_token: Default::default(),
                        member: Member::Unnamed(Index {
                            index,
                            span: Span::call_site(),
                        }),
                    });
                    quote! { #field }
                }
                _ => panic!(),
            },
            ExprFMF::Get { lhs, rhs } => match lhs.r#type {
                Type::Record(_) => match *rhs.val.0 {
                    ExprFMF::Int { val } => {
                        let index = val.try_into().unwrap();
                        let field = syn::Expr::Field(ExprField {
                            attrs: vec![],
                            base: Box::new(parse2(lhs.into()).unwrap()),
                            dot_token: Default::default(),
                            member: Member::Unnamed(Index {
                                index,
                                span: Span::call_site(),
                            }),
                        });
                        quote! { #field }
                    }
                    _ => unimplemented!(),
                },
                Type::Dict { .. } => {
                    let lhs: TokenStream = lhs.into();
                    let rhs: TokenStream = rhs.into();
                    quote! { #lhs[#rhs] }
                }
                _ => panic!(),
            },
            ExprFMF::FMF {
                op: OpFMF::Fold,
                args,
                inner,
                cont: None,
            } => {
                let ExprFMF::Dict { map, hint } = *inner.val.0 else {
                    unimplemented!()
                };
                let map: Result<[DictEntry<_, _>; _], _> = map.try_into();
                let Ok([map]) = map else { unimplemented!() };
                let key: TokenStream = map.key.into();
                let val: TokenStream = map.val.into();
                let hint = to_type(hint);
                let args = args.iter().map(|name| Ident::new(name, Span::call_site()));
                quote! {
                    .fold(#hint::new(), |mut acc, #(#args),*| {
                        acc[&#key] += #val;
                        acc
                    })
                }
            }
            ExprFMF::Record { vals } => {
                let vals = vals.into_iter().map(|rv| TokenStream::from(rv.val));
                quote! { Record::new((#(#vals),*,)) }
            }
            t => todo!("{t:?}"),
        }
    }
}

fn gen_range(end: syn::Expr) -> syn::Expr {
    syn::Expr::Range(ExprRange {
        attrs: Vec::new(),
        start: Some(Box::new(parse_quote!(0))),
        limits: RangeLimits::HalfOpen(Default::default()),
        end: Some(Box::new(end)),
    })
}

fn try_gen_load(fields: &[(&str, syn::Type)]) -> Result<syn::Macro, Error> {
    let field_tks = fields.iter().map(|(name, ty)| {
        let name = format_ident!("{name}");
        quote! { #name: #ty }
    });
    let macro_tks = quote! {
        load!(
            #(#field_tks),*
        )
    };
    parse2(macro_tks)
}

impl From<Type<'_>> for syn::Type {
    fn from(r#type: Type) -> Self {
        match r#type {
            Type::Bool => parse_quote!(bool),
            Type::Date => parse_quote!(Date),
            Type::Int => parse_quote!(i32),
            Type::Long => parse_quote!(i64),
            Type::Real => parse_quote!(f64),
            Type::String { max_len: None } => parse_quote!(String),
            Type::String {
                max_len: Some(_max_len),
            } => parse_quote!(String), // TODO
            Type::Record(_) => parse_quote!(Record),
            Type::Dict { hint, .. } => to_type(hint),
        }
    }
}

fn to_type(hint: Option<DictHint>) -> syn::Type {
    hint.map(syn::Type::from)
        .unwrap_or_else(|| parse_quote!(HashMap))
}

impl From<DictHint> for syn::Type {
    fn from(hint: DictHint) -> Self {
        match hint {
            DictHint::HashDict => parse_quote!(HashMap),
            DictHint::SortDict => parse_quote!(SortDict),
            DictHint::SmallVecDict => parse_quote!(SmallVecDict),
            DictHint::Vec => parse_quote!(Vec),
        }
    }
}

impl From<BinaryOp> for BinOp {
    fn from(op: BinaryOp) -> Self {
        match op {
            BinaryOp::Add => Self::Add(Default::default()),
            BinaryOp::Sub => Self::Sub(Default::default()),
            BinaryOp::Mul => Self::Mul(Default::default()),
            BinaryOp::Div => Self::Div(Default::default()),
            BinaryOp::Eq => Self::Eq(Default::default()),
            BinaryOp::NotEq => Self::Ne(Default::default()),
            BinaryOp::Less => Self::Lt(Default::default()),
            BinaryOp::Great => Self::Gt(Default::default()),
            BinaryOp::LessEq => Self::Le(Default::default()),
            BinaryOp::GreatEq => Self::Ge(Default::default()),
            BinaryOp::And => Self::And(Default::default()),
            BinaryOp::Or => Self::Or(Default::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::fmf::ExprFMF;
    use crate::frontend::lexer::Spanned;
    use crate::inference::{Typed, TypedExpr};
    use crate::ir::expr::Expr;
    use proc_macro2::TokenStream;

    const LOAD: &str = "load[<l_orderkey: @vec {int -> int}, l_partkey: @vec {int -> int}, l_suppkey: @vec {int -> int}, l_linenumber: @vec {int -> int}, l_quantity: @vec {int -> real}, l_extendedprice: @vec {int -> real}, l_discount: @vec {int -> real}, l_tax: @vec {int -> real}, l_returnflag: @vec {int -> varchar(1)}, l_linestatus: @vec {int -> varchar(1)}, l_shipdate: @vec {int -> date}, l_commitdate: @vec {int -> date}, l_receiptdate: @vec {int -> date}, l_shipinstruct: @vec {int -> varchar(25)}, l_shipmode: @vec {int -> varchar(10)}, l_comment: @vec {int -> varchar(44)}, size: int>](\"datasets/tpch_datasets/SF_0.01/lineitem.tbl\")";

    #[test]
    fn test_load() {
        let src: &str = &format!("let _ = {LOAD} in 0");
        let expr = Spanned::<Expr>::try_from(src).unwrap();
        let t: Typed<Spanned<TypedExpr>> = expr.into();
        let fmf: Typed<Spanned<ExprFMF>> = t.into();
        let tks: TokenStream = fmf.into();
        // println!("{tks}");
        let main_tks = quote! { fn main() { #tks } };
        let ast = parse2(main_tks).unwrap();
        // println!("{ast:#?}");
        let s = prettyplease::unparse(&ast);
        println!("{s}");
    }
}
