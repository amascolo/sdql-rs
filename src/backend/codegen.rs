use super::fmf::{ExprFMF, OpFMF};
use crate::frontend::lexer::Spanned;
use crate::inference::Typed;
use crate::ir::expr::{BinOp, DictEntry, External, UnaryOp};
use crate::ir::r#type::{DictHint, Type};
use itertools::Itertools;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use std::assert_matches::debug_assert_matches;
use syn::{
    parse2, parse_quote, Error, ExprBinary, ExprField, ExprRange, Index, LitInt, Member,
    RangeLimits,
};

pub fn codegen<'src, const PARALLEL: bool>(expr: Typed<'src, Spanned<ExprFMF<'src>>>) -> String {
    let Typed { val, ref r#type } = expr;
    let r#type: syn::Type = r#type.into();
    let Spanned(expr, _span) = val;
    let tks = ts::<PARALLEL>(expr);
    let imports = quote! { use sdql_runtime::*; };
    let imports = if PARALLEL {
        quote! {
            #imports
            use rayon::prelude::*;
        }
    } else {
        quote! { #imports }
    };
    let main_tks = quote! {
        #![feature(stmt_expr_attributes)]
        #![allow(unused_variables)]
        #imports
        fn main() {
            let value: #r#type = { #tks };
            // println!("{value:?}"); // TODO have a mode from main.rs that generates this instead
            use std::io::Write;
            let encoded = bincode::serialize(&value).unwrap();
            std::io::stdout().write_all(&encoded).unwrap();
        }
    };
    let ast = parse2(main_tks).unwrap();
    prettyplease::unparse(&ast)
}

fn ts_typed<'src, const PARALLEL: bool>(expr: Typed<'src, Spanned<ExprFMF<'src>>>) -> TokenStream {
    ts::<PARALLEL>(ExprFMF::from(expr))
}

fn ts_boxed<'src, const PARALLEL: bool>(
    expr: Typed<'src, Spanned<Box<ExprFMF<'src>>>>,
) -> TokenStream {
    ts::<PARALLEL>(ExprFMF::from(expr.map(Spanned::unboxed)))
}

fn ts<const PARALLEL: bool>(expr: ExprFMF<'_>) -> TokenStream {
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
            let rhs_tks = ts_boxed::<PARALLEL>(rhs);
            let let_tks = quote! { let #lhs_tks = #rhs_tks };
            let cont_tks = ts_boxed::<PARALLEL>(cont);
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
            debug_assert_matches!(parse2(tks.clone()), Ok(syn::Expr::MethodCall(_)));
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
            let range = ExprFMF::from(expr.map(Spanned::unboxed));
            let range: syn::Expr = parse2(ts::<PARALLEL>(range)).unwrap();
            let range = gen_range(range);
            let range = if PARALLEL {
                quote! { (#range).into_par_iter() }
            } else {
                quote! { (#range) }
            };
            let body = ExprFMF::from(body.map(Spanned::unboxed));
            let body = ts::<PARALLEL>(body);
            quote! { #range #body }
        }
        ExprFMF::Sum {
            key: _,
            val: _,
            head,
            body,
        } => {
            let head = ts_boxed::<PARALLEL>(head);
            let body = ts_boxed::<PARALLEL>(body);
            if PARALLEL {
                quote! { #head.into_par_iter()#body }
            } else {
                quote! { #head.into_iter()#body }
            }
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
            let lhs = ts_boxed::<PARALLEL>(lhs);
            let rhs = ts_boxed::<PARALLEL>(rhs);
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
            let inner = ts_boxed::<PARALLEL>(inner);
            let cont = ExprFMF::from(cont.map(Spanned::unboxed));
            let cont = ts::<PARALLEL>(cont);
            let args = gen_args(args);
            quote! {.filter(|&#args| #inner)#cont}
        }
        ExprFMF::FMF {
            op: OpFMF::FlatMap,
            args,
            inner,
            cont: None,
        } => {
            let ExprFMF::Sum {
                key,
                val,
                head,
                body,
            } = *inner.val.0
            else {
                unreachable!()
            };
            let key = Ident::new(key, Span::call_site());
            let val = Ident::new(val, Span::call_site());
            let args: Vec<_> = args
                .iter()
                .map(|name| Ident::new(name, Span::call_site()))
                .collect();
            let head = ts_boxed::<PARALLEL>(head);
            let body = ts_boxed::<PARALLEL>(body);
            let fn_args = if args.len() > 1 {
                quote! { (#(#args),*) }
            } else {
                quote! { #(#args),* }
            };
            quote! {
                .flat_map(|#fn_args| {
                    #head
                    .iter()
                    .map(move |(#key, #val)| (#(#args),*, #key, #val) )
                })#body
            }
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
            let inner = ts_boxed::<PARALLEL>(inner);
            quote! {.map(|#fn_args| #inner).sum()}
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
            let inner = ts_boxed::<PARALLEL>(inner);
            let cont = ts_boxed::<PARALLEL>(cont);
            quote! {.map(|#fn_args| (#(#args),*, #inner))#cont}
        }
        ExprFMF::Unary {
            op: UnaryOp::Neg,
            expr,
        } => {
            let expr = ts_boxed::<PARALLEL>(expr);
            quote! { -#expr }
        }
        ExprFMF::Unary {
            op: UnaryOp::Not,
            expr,
        } => {
            let expr = ts_boxed::<PARALLEL>(expr);
            quote! { !#expr }
        }
        ExprFMF::Binary { lhs, op, rhs } => {
            let lhs = parse2(ts_boxed::<PARALLEL>(lhs)).unwrap();
            let rhs = parse2(ts_boxed::<PARALLEL>(rhs)).unwrap();
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
                    base: Box::new(parse2(ts_boxed::<PARALLEL>(expr)).unwrap()),
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
            let hint = match &expr.r#type {
                Type::Dict { hint, .. } => hint.clone(),
                _ => panic!(),
            };
            let lhs = ts_boxed::<PARALLEL>(expr);
            let rhs = ts_boxed::<PARALLEL>(rhs);
            match hint {
                Some(DictHint::Vec { .. }) => quote! { #lhs[#rhs as usize] != 0 },
                _ => quote! { #lhs.contains_key(&#rhs) },
            }
        }
        ExprFMF::Get { lhs, rhs } => match lhs.r#type {
            Type::Record(_) => match *rhs.val.0 {
                ExprFMF::Int { val } => {
                    let index = val.try_into().unwrap();
                    let field = syn::Expr::Field(ExprField {
                        attrs: vec![],
                        base: Box::new(parse2(ts_boxed::<PARALLEL>(lhs)).unwrap()),
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
                let lhs = ts_boxed::<PARALLEL>(lhs);
                let rhs = ts_boxed::<PARALLEL>(rhs);
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
            let r#type: syn::Type = (&inner.r#type).into();
            let init = initialise::<PARALLEL>(&inner.r#type);
            let args = gen_args(args);
            let inner = inner.map(Spanned::unboxed);
            let hints = hints(&inner);
            let (lhs, rhs) = split(inner);
            let lhs: TokenStream = lhs
                .into_iter()
                .map(ts_typed::<PARALLEL>)
                .zip_eq(hints)
                .map(|(ts, hint)| match hint {
                    Some(DictHint::SmallVecDict { .. } | DictHint::VecDict { .. }) => {
                        quote! { [#ts] }
                    }
                    Some(DictHint::Vec { .. }) => quote! { [#ts as usize] },
                    _ => quote! { [&#ts] },
                })
                .flatten()
                .collect();
            let rhs = ts_typed::<PARALLEL>(rhs);
            let fold = quote! {
                .fold(#init, |mut acc: #r#type, #args| {
                    acc #lhs += #rhs;
                    acc
                })
            };
            if PARALLEL {
                quote! { #fold.sum() }
            } else {
                quote! { #fold }
            }
        }
        ExprFMF::Record { vals } => {
            let vals = vals.into_iter().map(|rv| ts_typed::<PARALLEL>(rv.val));
            quote! { Record::new((#(#vals),*,)) }
        }
        ExprFMF::Dict { map, hint } if map.len() == 1 => {
            let [entry]: [_; 1] = map.try_into().unwrap();
            let r#type = to_type(hint);
            let key = ts_typed::<PARALLEL>(entry.key);
            let val = ts_typed::<PARALLEL>(entry.val);
            quote! { #r#type::from([(#key, #val)]) }
        }
        ExprFMF::Dom { .. } => unimplemented!(),
        ExprFMF::If { r#if, then, r#else } => {
            let r#if: Typed<Spanned<ExprFMF>> = r#if.map(Spanned::unboxed);
            let then: Typed<Spanned<ExprFMF>> = then.map(Spanned::unboxed);
            let r#else: Option<Typed<Spanned<ExprFMF>>> =
                r#else.map(|r#else| r#else.map(Spanned::unboxed));
            let r#if = ts_typed::<PARALLEL>(r#if);
            let then = ts_typed::<PARALLEL>(then);
            let r#else = r#else.map(ts_typed::<PARALLEL>);
            match r#else {
                None => quote! { if #r#if { #then } },
                Some(r#else) => quote! { if #r#if { #then } else { #r#else } },
            }
        }
        ExprFMF::Unique { expr } => {
            let ExprFMF::FMF {
                op: OpFMF::Fold,
                args,
                inner,
                cont: None,
            } = *expr.val.0
            else {
                unreachable!()
            };
            let args = gen_args(args);
            let (lhs, rhs) = split(inner.map(Spanned::unboxed));
            let [lhs]: [_; _] = lhs.try_into().unwrap();
            let lhs = ts::<PARALLEL>(lhs.val.0);
            let rhs = ts::<PARALLEL>(rhs.val.0);
            quote! {
                .map(|#args| {
                    (
                        #lhs,
                        #rhs,
                    )
                }).collect()
            }
        }
        ExprFMF::External {
            func: External::StrContains,
            args,
        } => {
            let [arg0, arg1]: [_; _] = args.try_into().unwrap();
            let arg0 = ts_typed::<PARALLEL>(arg0.clone());
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
            let arg0 = ts_typed::<PARALLEL>(arg0.clone());
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
            let arg0 = ts_typed::<PARALLEL>(arg0);
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
            func: External::FirstIndex,
            args,
        } => {
            let [arg0, arg1]: [_; _] = args.try_into().unwrap();
            let arg0 = ts_typed::<PARALLEL>(arg0.clone());
            let Typed {
                val: Spanned(ExprFMF::String { val, max_len: _ }, _),
                r#type: _,
            } = arg1
            else {
                unreachable!()
            };
            quote! { #arg0.find(&#val).map(|i| i as i32).unwrap_or(-1) }
        }
        ExprFMF::External {
            func: External::LastIndex,
            args,
        } => {
            let [arg0, arg1]: [_; _] = args.try_into().unwrap();
            let arg0 = ts_typed::<PARALLEL>(arg0);
            let Typed {
                val: Spanned(ExprFMF::String { val, max_len: _ }, _),
                r#type: _,
            } = arg1
            else {
                unreachable!()
            };
            quote! { #arg0.rfind(&#val).map(|i| i as i32).unwrap_or(-1) }
        }
        ExprFMF::External {
            func: External::SubString,
            args,
        } => {
            let [string, start, end]: [_; _] = args.try_into().unwrap();
            let string = ts_typed::<PARALLEL>(string);
            let start: usize = match start.val.0 {
                ExprFMF::Int { val } => val.try_into(),
                ExprFMF::Long { val } => val.try_into(),
                _ => unimplemented!(),
            }
            .unwrap();
            let end: usize = match end.val.0 {
                ExprFMF::Int { val } => val.try_into(),
                ExprFMF::Long { val } => val.try_into(),
                _ => unimplemented!(),
            }
            .unwrap();
            quote! { VarChar::<{ #end - #start }>::from(&(#string)[#start..#end]).unwrap() }
        }
        ExprFMF::External {
            func: External::Size,
            args,
        } => {
            let [arg]: [_; _] = args.try_into().unwrap();
            let arg = ts_typed::<PARALLEL>(arg.clone());
            quote! { #arg.len() as i32 }
        }
        ExprFMF::External {
            func: External::Year,
            args,
        } => {
            let [arg]: [_; _] = args.try_into().unwrap();
            let arg = ts_typed::<PARALLEL>(arg.clone());
            quote! { #arg.year() }
        }
        #[allow(unreachable_patterns)] // handy if you are adding more
        ExprFMF::External { func, args: _ } => todo!("{func}"),
        t => todo!("{t:?}"),
    }
}

fn gen_args(args: im_rc::Vector<&str>) -> syn::Expr {
    let len = args.len();
    let mut args = args.into_iter().map(|arg| {
        let ident = Ident::new(arg, Span::call_site());
        parse_quote! { #ident }
    });
    match len {
        0 => unimplemented!(),
        1 => args.next().unwrap(),
        _ => parse_quote! { (#(#args),*) },
    }
}

fn hints<'src>(mut expr: &Typed<'src, Spanned<ExprFMF<'src>>>) -> Vec<Option<DictHint>> {
    let mut hints = Vec::new();

    while let ExprFMF::Dict { map, hint } = &expr.val.0 {
        if map.len() != 1 {
            unimplemented!();
        }
        let DictEntry { val, .. } = map.iter().next().unwrap();
        hints.push(*hint);
        expr = val;
    }

    hints
}

fn split<'src>(
    expr: Typed<'src, Spanned<ExprFMF<'src>>>,
) -> (
    Vec<Typed<'src, Spanned<ExprFMF<'src>>>>,
    Typed<'src, Spanned<ExprFMF<'src>>>,
) {
    if let ExprFMF::Dict { map, hint: _ } = &expr.val.0
        && map.len() != 1
    {
        unimplemented!()
    }

    if let ExprFMF::Dict { map, hint } = expr.val.0 {
        let [DictEntry { key, val }]: [_; _] = map.try_into().unwrap();

        // { ... -> @!vec { ... } }
        if let ExprFMF::Dict { map: _, hint } = &val.val.0
            && !matches!(hint, Some(DictHint::Vec { .. }))
        {
            let (mut lhs, rhs) = split(val);
            lhs.insert(0, key);
            return (lhs, rhs);
        }

        // { ... -> @vec { ... -> 1 } }
        if let ExprFMF::Dict { map, hint } = &val.val.0
            && {
                let DictEntry { key: _, val } = map.iter().next().unwrap();
                matches!(val.val.0, ExprFMF::Int { val: 1 })
                    && matches!(hint, Some(DictHint::Vec { .. }))
            }
        {
            let ExprFMF::Dict { map, hint: _ } = val.val.0 else {
                unreachable!()
            };
            let [DictEntry { key: rhs, val: _ }]: [_; _] = map.try_into().unwrap();
            return (vec![key], rhs);
        }

        // @vec { < ... > -> 1 }
        if matches!(key.val.0, ExprFMF::Record { .. })
            && matches!(val.val.0, ExprFMF::Int { val: 1 })
            && matches!(hint, Some(DictHint::Vec { .. }))
        {
            return (vec![], key);
        }

        // { ... }
        return (vec![key], val);
    }

    (vec![], expr)
}

fn initialise<const PARALLEL: bool>(r#type: &Type) -> TokenStream {
    match r#type {
        Type::Dict {
            key: _,
            val: _,
            hint:
                Some(
                    DictHint::HashDict {
                        capacity: Some(capacity),
                    }
                    | DictHint::SortDict {
                        capacity: Some(capacity),
                    },
                ),
        } => {
            let t = simple_type(&r#type);
            let capacity = LitInt::new(&capacity.to_string(), Span::call_site());
            let init = quote! { #t::with_capacity(#capacity) };
            if PARALLEL {
                quote! { || #init }
            } else {
                quote! { #init }
            }
        }
        Type::Dict {
            key: _,
            val,
            hint: Some(DictHint::Vec {
                capacity: Some(capacity),
            }),
        } => {
            let t = simple_type(val);
            let capacity = LitInt::new(&capacity.to_string(), Span::call_site());
            let init = quote! { vec![#t::default(); #capacity] };
            if PARALLEL {
                quote! { || #init }
            } else {
                quote! { #init }
            }
        }
        Type::Dict {
            key: _,
            val: _,
            hint: Some(DictHint::VecDict {
                capacity: Some(_capacity),
            }),
        } => todo!(),
        _ => {
            let t = simple_type(&r#type);
            if PARALLEL {
                quote! { #t::default }
            } else {
                quote! { #t::default() }
            }
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
                key,
                val,
                hint: hint @ (None | Some(DictHint::HashDict { .. } | DictHint::SortDict { .. })),
            } => {
                let dict = to_type(*hint);
                let key = syn::Type::from(&**key);
                let val = syn::Type::from(&**val);
                parse_quote!(#dict<#key, #val>)
            }
            Type::Dict {
                key: _,
                val,
                hint: Some(hint @ DictHint::SmallVecDict { .. }),
            } => {
                let dict = to_type(Some(*hint));
                let val = syn::Type::from(&**val);
                let capacity = hint.capacity();
                parse_quote!(#dict<[#val; #capacity]>)
            }
            Type::Dict {
                key: _,
                val,
                hint: hint @ Some(DictHint::Vec { .. } | DictHint::VecDict { .. }),
            } => {
                let dict = to_type(*hint);
                let val = syn::Type::from(&**val);
                parse_quote!(#dict<#val>)
            }
        }
    }
}
fn simple_type(r#type: &Type) -> syn::Type {
    match r#type {
        Type::Bool => parse_quote!(Bool),
        Type::Date => parse_quote!(Date),
        Type::Int => parse_quote!(i32),
        Type::Long => parse_quote!(i64),
        Type::Real => parse_quote!(OrderedFloat<f64>),
        Type::String { max_len: None } => parse_quote!(String),
        Type::String { max_len: Some(_) } => parse_quote!(VarChar),
        Type::Record(_) => parse_quote!(Record),
        Type::Dict {
            key: _,
            val: _,
            hint,
        } => {
            let dict = to_type(*hint);
            parse_quote!(#dict)
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
            DictHint::HashDict { capacity: _ } => parse_quote!(HashMap),
            DictHint::SortDict { capacity: _ } => parse_quote!(SortDict),
            DictHint::SmallVecDict { capacity: _ } => parse_quote!(SmallVecDict),
            DictHint::Vec { capacity: _ } => parse_quote!(Vec),
            DictHint::VecDict { capacity: _ } => parse_quote!(VecDict),
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
    fn tpch_1() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/1.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_2() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/2.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_3() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/3.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_4() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/4.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_5() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/5.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_6() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/6.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_7() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/7.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_8() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/8.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_9() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/9.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_10() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/10.sdql"));
        let _ = rs!(src);
    }

    // FIXME
    // #[test]
    // fn tpch_11() {
    //     let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/11.sdql"));
    //     let _ = rs!(src);
    // }

    #[test]
    fn tpch_12() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/12.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_13() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/13.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_14() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/14.sdql"));
        let _ = rs!(src);
    }

    // FIXME TPCH q15 add support for max
    // #[test]
    // fn tpch_15() {
    //     let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/15.sdql"));
    //     let _ = rs!(src);
    // }

    #[test]
    fn tpch_16() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/16.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_17() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/17.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_18() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/18.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_19() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/19.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_20() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/20.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_21() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/21.sdql"));
        let _ = rs!(src);
    }

    #[test]
    fn tpch_22() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/22.sdql"));
        let _ = rs!(src);
    }
}
