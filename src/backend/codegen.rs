use super::fmf::ExprFMF;
use crate::frontend::lexer::Spanned;
use crate::inference::{Typed, TypedExpr};
use crate::ir::r#type::{DictHint, Type};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse2, parse_quote, Error};

impl From<Type<'_>> for syn::Type {
    fn from(r#type: Type) -> Self {
        match r#type {
            Type::Bool => parse_quote!(bool),
            Type::Date => parse_quote!(crate::runtime::Date),
            Type::Int => parse_quote!(i32),
            Type::Long => parse_quote!(i64),
            Type::Real => parse_quote!(f64),
            Type::String { max_len: None } => parse_quote!(String),
            Type::String {
                max_len: Some(_max_len),
            } => parse_quote!(String), // TODO
            Type::Record(_) => parse_quote!(crate::runtime::Record),
            Type::Dict {
                hint: None | Some(DictHint::HashDict),
                ..
            } => parse_quote!(crate::runtime::HashMap),
            Type::Dict {
                hint: Some(DictHint::Vec),
                ..
            } => parse_quote!(Vec),
            Type::Dict {
                hint: Some(DictHint::SortDict),
                ..
            } => parse_quote!(crate::runtime::SortDict),
            Type::Dict {
                hint: Some(DictHint::SmallVecDict),
                ..
            } => parse_quote!(crate::runtime::SmallVecDict),
        }
    }
}

impl From<ExprFMF<'_>> for TokenStream {
    fn from(expr: ExprFMF) -> Self {
        match expr {
            ExprFMF::Expr(expr) => expr.into(),
            ExprFMF::Range { .. } => todo!(),
            ExprFMF::FMF { .. } => todo!(),
        }
    }
}

impl From<Typed<'_, Spanned<TypedExpr<'_>>>> for TokenStream {
    fn from(expr: Typed<'_, Spanned<TypedExpr<'_>>>) -> Self {
        let Typed { val, r#type: _ } = expr;
        let Spanned(unspanned, _span) = val;
        match unspanned {
            TypedExpr::Load { r#type, path } => {
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
                let tokens = quote! { #load(#path) };
                debug_assert!(matches!(parse2(tokens.clone()), Ok(syn::Expr::Call(_))));
                tokens
            }
            _ => todo!(),
        }
    }
}

fn try_gen_load(fields: &[(&str, syn::Type)]) -> Result<syn::Macro, Error> {
    let field_tokens = fields.iter().map(|(name, ty)| {
        let name = format_ident!("{name}");
        quote! { #name: #ty }
    });
    let macro_tokens = quote! {
        load!(
            #(#field_tokens),*
        )
    };
    parse2(macro_tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::fmf::ExprFMF;
    use crate::frontend::lexer::Spanned;
    use crate::inference::{Typed, TypedExpr};
    use crate::ir::expr::Expr;
    use proc_macro2::TokenStream;
    use syn::parse2;

    const LOAD: &str = "load[<l_orderkey: @vec {int -> int}, l_partkey: @vec {int -> int}, l_suppkey: @vec {int -> int}, l_linenumber: @vec {int -> int}, l_quantity: @vec {int -> real}, l_extendedprice: @vec {int -> real}, l_discount: @vec {int -> real}, l_tax: @vec {int -> real}, l_returnflag: @vec {int -> varchar(1)}, l_linestatus: @vec {int -> varchar(1)}, l_shipdate: @vec {int -> date}, l_commitdate: @vec {int -> date}, l_receiptdate: @vec {int -> date}, l_shipinstruct: @vec {int -> varchar(25)}, l_shipmode: @vec {int -> varchar(10)}, l_comment: @vec {int -> varchar(44)}, size: int>](\"datasets/tpch_datasets/SF_0.01/lineitem.tbl\")";

    #[test]
    fn test_load() {
        let src = LOAD;
        let expr = Spanned::<Expr>::try_from(src).unwrap();
        let t: Typed<Spanned<TypedExpr>> = expr.into();
        let fmf: ExprFMF = t.into();

        // match fmf {
        //     ExprFMF::Expr(load @ Expr::Load { .. }) => {}
        // }

        let tokens: TokenStream = fmf.into();
        println!("{}", tokens);
        let ast: syn::Expr = parse2(tokens).unwrap();
        // println!("{:#?}", ast);
        // let s = prettyplease::unparse(&ast);
        // println!("{s}");
    }
}
