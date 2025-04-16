use super::fmf::{ExprFMF, OpFMF};
use crate::frontend::lexer::Spanned;
use crate::inference::Typed;
use crate::ir::expr::{BinOp, DictEntry, External};
use crate::ir::r#type::{DictHint, Type};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse2, parse_quote, Error, ExprBinary, ExprField, ExprRange, Index, LitInt, Member,
    RangeLimits,
};

impl From<ExprFMF<'_>> for String {
    fn from(expr: ExprFMF<'_>) -> Self {
        let tks: TokenStream = expr.into();
        let main_tks = quote! {
            #![feature(stmt_expr_attributes)]
            #![allow(unused_variables)]
            use sdql_runtime::*;
            fn main() {
                let value = { #tks };
                // println!("{value:?}"); // TODO default mode
                use std::io::Write;
                let encoded = bincode::serialize(&value).unwrap();
                std::io::stdout().write_all(&encoded).unwrap();
            }
        };
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
            ExprFMF::Bool { val } => {
                let val = if val { "TRUE" } else { "FALSE" };
                let ident = Ident::new(val, Span::call_site());
                quote!(#ident)
            }
            ExprFMF::Date { val } => format!(
                "date!({:04}{:02}{:02})",
                val.0.year(),
                val.0.month() as u8,
                val.0.day()
            )
            .parse()
            .unwrap(),
            ExprFMF::Int { val } => quote! { #val },
            ExprFMF::Long { val } => quote! { #val },
            ExprFMF::Real { val } => quote! { OrderedFloat(#val) },
            ExprFMF::String { val, max_len } => {
                let suffixed = match max_len {
                    None => quote! { VarChar },
                    Some(max_len) => quote! { VarChar::<#max_len> },
                };
                quote! { #suffixed::from_str(#val).unwrap() }
            }
            ExprFMF::Let { lhs, rhs, cont } => {
                let lhs_ident = Ident::new(lhs, Span::call_site());
                let mut lhs_tks = quote! { #lhs_ident };
                // TODO remove special case for load
                if !matches!(*rhs.val.0, ExprFMF::Load { .. }) {
                    let rhs_type: syn::Type = (&rhs.r#type).into();
                    lhs_tks = quote! { #lhs_tks: #rhs_type }
                }
                let rhs_tks: TokenStream = rhs.into();
                let let_tks = quote! { let #lhs_tks = #rhs_tks };
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
                                hint: Some(DictHint::Vec { capacity: None }),
                            } if matches!(*key, Type::Int) => *val,
                            _ => unreachable!(),
                        };
                        (val.name.into(), (&r#type).into())
                    })
                    .collect();
                let load = try_gen_load(&tables).unwrap();
                let tks = quote! { #load(#path).unwrap() };
                debug_assert!(matches!(parse2(tks.clone()), Ok(syn::Expr::MethodCall(_))));
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
            ExprFMF::Sum {
                key: _,
                val: _,
                head,
                body,
            } => {
                let head: TokenStream = head.into();
                let body: TokenStream = body.into();
                quote! { #head.iter()#body }
            }
            ExprFMF::Concat { lhs, rhs } => {
                let lhs_len = match lhs.r#type {
                    Type::Record(ref vals) => vals.len(),
                    _ => panic!(),
                };
                let rhs_len = match rhs.r#type {
                    Type::Record(ref vals) => vals.len(),
                    _ => panic!(),
                };
                let lhs: TokenStream = lhs.into();
                let rhs: TokenStream = rhs.into();
                let lhs = (0..lhs_len).map(Index::from).map(|i| quote! { #lhs.#i });
                let rhs = (0..rhs_len).map(Index::from).map(|i| quote! { #rhs.#i });
                quote! { Record::new((#(#lhs),*, #(#rhs),*)) }
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
                let args = gen_args(args);
                quote! {.filter(|&#args| #inner)#cont}
            }
            ExprFMF::FMF {
                op: OpFMF::Map,
                args,
                inner,
                cont: None,
            } => {
                let args = args.iter().map(|name| Ident::new(name, Span::call_site()));
                let fn_args = if args.len() > 1 {
                    quote! { (#(#args),*) }
                } else {
                    quote! { #(#args),* }
                };
                let r#type: syn::Type = (&inner.r#type).into();
                let inner: TokenStream = inner.into();
                quote! {.map(|#fn_args| #inner).sum::<#r#type>()}
            }
            ExprFMF::FMF {
                op: OpFMF::Map,
                args,
                inner,
                cont: Some(cont),
            } => {
                let args: Vec<_> = args
                    .iter()
                    .map(|name| Ident::new(name, Span::call_site()))
                    .collect();
                let fn_args = if args.len() > 1 {
                    quote! { (#(#args),*) }
                } else {
                    quote! { #(#args),* }
                };
                let inner: TokenStream = inner.into();
                let cont: TokenStream = cont.into();
                quote! {.map(|#fn_args| (#(#args),*, #inner))#cont}
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
            ExprFMF::Get {
                lhs:
                    Typed {
                        val: Spanned(val, _),
                        r#type: _,
                    },
                rhs,
            } if matches!(*val, ExprFMF::Dom { .. }) => {
                let ExprFMF::Dom { expr } = *val else {
                    unreachable!()
                };
                let lhs: TokenStream = expr.into();
                let rhs: TokenStream = rhs.into();
                quote! { #lhs.contains_key(&#rhs) }
            }
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
                Type::Dict { hint, .. } => {
                    let lhs: TokenStream = lhs.into();
                    let rhs: TokenStream = rhs.into();
                    match hint {
                        Some(DictHint::Vec { .. }) => quote! { #lhs[#rhs as usize] },
                        _ => quote! { #lhs[&#rhs] },
                    }
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
                let r#type = qualified_type(&inner.r#type);
                let capacity = match hint {
                    None => None,
                    Some(hint) => hint.capacity(),
                };
                let init = match capacity {
                    None => quote! { #r#type::new() },
                    Some(capacity) => {
                        let capacity = LitInt::new(&capacity.to_string(), Span::call_site());
                        let Some(hint) = hint else { unreachable!() };
                        match hint {
                            DictHint::Vec { .. } => {
                                let r#type = syn::Type::from(&map.val.r#type);
                                quote! { vec![#r#type::default(); #capacity] }
                            }
                            _ => quote! { #r#type::with_capacity(#capacity) },
                        }
                    }
                };
                let args = gen_args(args);
                let key: TokenStream = map.key.into();
                let val: TokenStream = map.val.into();
                let key = if let Type::Dict {
                    hint: Some(DictHint::Vec { .. }),
                    ..
                } = inner.r#type
                {
                    quote! { #key as usize }
                } else {
                    quote! { &#key }
                };
                quote! {
                    .fold(#init, |mut acc, #args| {
                        acc[#key] += #val;
                        acc
                    })
                }
            }
            ExprFMF::Record { vals } => {
                let vals = vals.into_iter().map(|rv| TokenStream::from(rv.val));
                quote! { Record::new((#(#vals),*,)) }
            }
            ExprFMF::Dict { map, hint } if map.len() == 1 => {
                let [entry]: [_; 1] = map.try_into().unwrap();
                let r#type = to_type(hint);
                let key: TokenStream = entry.key.into();
                let val: TokenStream = entry.val.into();
                quote! { #r#type::from([(#key, #val)]) }
            }
            ExprFMF::Dom { .. } => unimplemented!(),
            ExprFMF::If { r#if, then, r#else } => {
                let r#if: Typed<Spanned<ExprFMF>> = r#if.map(Spanned::unboxed);
                let then: Typed<Spanned<ExprFMF>> = then.map(Spanned::unboxed);
                let r#else: Option<Typed<Spanned<ExprFMF>>> =
                    r#else.map(|r#else| r#else.map(Spanned::unboxed));
                let r#if: TokenStream = r#if.into();
                let then: TokenStream = then.into();
                let r#else: Option<TokenStream> = r#else.map(|r#else| r#else.into());
                match r#else {
                    None => quote! { if #r#if { #then } },
                    Some(r#else) => quote! { if #r#if { #then } else { #r#else } },
                }
            }
            ExprFMF::External {
                func: External::StrContains,
                args,
            } => {
                let [arg0, arg1]: [_; _] = args.try_into().unwrap();
                let arg0: TokenStream = arg0.clone().into();
                let Typed {
                    val: Spanned(ExprFMF::String { val, max_len: _ }, _),
                    r#type: _,
                } = arg1
                else {
                    unreachable!()
                };
                quote! { #arg0.contains(&#val) }
            }
            ExprFMF::External {
                func: External::StrStartsWith,
                args,
            } => {
                let [arg0, arg1]: [_; _] = args.try_into().unwrap();
                let arg0: TokenStream = arg0.clone().into();
                let Typed {
                    val: Spanned(ExprFMF::String { val, max_len: _ }, _),
                    r#type: _,
                } = arg1
                else {
                    unreachable!()
                };
                quote! { #arg0.starts_with(&#val) }
            }
            ExprFMF::External {
                func: External::StrEndsWith,
                args,
            } => {
                let [arg0, arg1]: [_; _] = args.try_into().unwrap();
                let arg0: TokenStream = arg0.into();
                let Typed {
                    val: Spanned(ExprFMF::String { val, max_len: _ }, _),
                    r#type: _,
                } = arg1
                else {
                    unreachable!()
                };
                quote! { #arg0.ends_with(&#val) }
            }
            ExprFMF::External {
                func: External::Year,
                args,
            } => {
                let [arg]: [_; _] = args.try_into().unwrap();
                let arg: TokenStream = arg.clone().into();
                quote! { #arg.year() }
            }
            #[allow(unreachable_patterns)] // handy if you are adding more
            ExprFMF::External { func, args: _ } => todo!("{func}"),
            ExprFMF::Unique { expr } => expr.into(), // TODO
            t => todo!("{t:?}"),
        }
    }
}

fn gen_args(args: im_rc::Vector<&str>) -> syn::Expr {
    let len = args.len();
    let mut args = args.into_iter().map(|arg| {
        let ident = Ident::new(arg, Span::call_site());
        // FIXME hardcoded - should be determined by arg type?
        if arg == "i"
            || arg == "avg"
            || arg == "orderdate"
            || arg == "orderyear"
            || arg == "volume"
            || arg == "brazil_volume"
        {
            parse_quote! { #ident }
        } else {
            parse_quote! { &#ident }
        }
    });
    match len {
        0 => unimplemented!(),
        1 => args.next().unwrap(),
        _ => parse_quote! { (#(#args),*) },
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

impl From<&Type<'_>> for syn::Type {
    fn from(r#type: &Type) -> Self {
        match r#type {
            Type::Bool => parse_quote!(Bool),
            Type::Date => parse_quote!(Date),
            Type::Int => parse_quote!(i32),
            Type::Long => parse_quote!(i64),
            Type::Real => parse_quote!(OrderedFloat<f64>),
            Type::String { max_len: None } => parse_quote!(String),
            Type::String {
                max_len: Some(max_len),
            } => {
                let max_len = LitInt::new(&max_len.to_string(), Span::call_site());
                parse_quote!(VarChar<#max_len>)
            }
            Type::Record(tps) => {
                let tps: Vec<syn::Type> =
                    tps.iter().map(|rt| syn::Type::from(&rt.r#type)).collect();
                parse_quote!(Record<(#(#tps),*,)>)
            }
            Type::Dict {
                key: _,
                val,
                hint: hint @ Some(DictHint::Vec { .. }),
            } => {
                let dict = to_type(*hint);
                let val = syn::Type::from(&**val);
                parse_quote!(#dict::<#val>)
            }
            Type::Dict { key, val, hint } => {
                let dict = to_type(*hint);
                let key = syn::Type::from(&**key);
                let val = syn::Type::from(&**val);
                parse_quote!(#dict<#key, #val>)
            }
        }
    }
}
// TODO simplify / avoid code duplication
fn qualified_type(r#type: &Type) -> syn::Type {
    match r#type {
        Type::String {
            max_len: Some(max_len),
        } => {
            let max_len = LitInt::new(&max_len.to_string(), Span::call_site());
            parse_quote!(VarChar::<#max_len>)
        }
        Type::Record(tps) => {
            let tps: Vec<syn::Type> = tps.iter().map(|rt| syn::Type::from(&rt.r#type)).collect();
            parse_quote!(Record::<(#(#tps),*,)>)
        }
        Type::Dict {
            key: _,
            val,
            hint: hint @ Some(DictHint::Vec { .. }),
        } => {
            let dict = to_type(*hint);
            let val = syn::Type::from(&**val);
            parse_quote!(#dict::<#val>)
        }
        Type::Dict { key, val, hint } => {
            let dict = to_type(*hint);
            let key = syn::Type::from(&**key);
            let val = syn::Type::from(&**val);
            parse_quote!(#dict::<#key, #val>)
        }
        _ => r#type.into(),
    }
}

fn to_type(hint: Option<DictHint>) -> syn::Type {
    hint.map(syn::Type::from)
        .unwrap_or_else(|| parse_quote!(HashMap))
}

impl From<DictHint> for syn::Type {
    fn from(hint: DictHint) -> Self {
        match hint {
            DictHint::HashDict { capacity: _ } => parse_quote!(HashMap),
            DictHint::SortDict { capacity: _ } => parse_quote!(SortDict),
            DictHint::SmallVecDict { capacity: _ } => parse_quote!(SmallVecDict),
            DictHint::Vec { capacity: _ } => parse_quote!(Vec),
        }
    }
}

impl From<BinOp> for syn::BinOp {
    fn from(op: BinOp) -> Self {
        match op {
            BinOp::Add => Self::Add(Default::default()),
            BinOp::Sub => Self::Sub(Default::default()),
            BinOp::Mul => Self::Mul(Default::default()),
            BinOp::Div => Self::Div(Default::default()),
            BinOp::Eq => Self::Eq(Default::default()),
            BinOp::Ne => Self::Ne(Default::default()),
            BinOp::Lt => Self::Lt(Default::default()),
            BinOp::Gt => Self::Gt(Default::default()),
            BinOp::Le => Self::Le(Default::default()),
            BinOp::Ge => Self::Ge(Default::default()),
            BinOp::And => Self::And(Default::default()),
            BinOp::Or => Self::Or(Default::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rs;

    #[test]
    fn tpch_q1() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/1.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_q3() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/3.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_q5() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/5.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_q6() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/6.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_q9() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/9.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_q18() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/18.sdql"));
        let _ = rs!(src);
    }
}
