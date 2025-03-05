use prettyplease::unparse;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse2, parse_quote, Ident};

fn generate_ast(type_name: &str, field_types: &[syn::Type]) -> TokenStream {
    let type_ident = Ident::new(type_name, Span::call_site());

    let field_definitions = field_types.iter().map(|ty| {
        quote! {
            Vec<#ty>,
        }
    });

    quote! {
        pub type #type_ident = (
            #(#field_definitions)*
            usize,
        );
    }
}

fn gen_read_fn(function_name: &str, fields: &[(&str, syn::Type)]) -> TokenStream {
    let field_tokens = fields.iter().enumerate().map(|(idx, (name, ty))| {
        quote! { (#idx, #name, #ty) }
    });
    quote! {
        gen_read_fn!(
            #function_name,
            #(#field_tokens),*
        );
    }
}

fn main() {
    let customer = [
        ("custkey", parse_quote!(i32)),
        ("name", parse_quote!(String)),
        ("address", parse_quote!(String)),
        ("nationkey", parse_quote!(i32)),
        ("phone", parse_quote!(String)),
        ("acctbal", parse_quote!(f64)),
        ("mktsegment", parse_quote!(String)),
        ("comment", parse_quote!(String)),
    ];
    let orders = [
        ("orderkey", parse_quote!(i32)),
        ("custkey", parse_quote!(i32)),
        ("orderstatus", parse_quote!(String)),
        ("totalprice", parse_quote!(f64)),
        ("orderdate", parse_quote!(i32)),
        ("orderpriority", parse_quote!(String)),
        ("clerk", parse_quote!(String)),
        ("shippriority", parse_quote!(i32)),
        ("comment", parse_quote!(String)),
    ];
    let lineitem = [
        ("orderkey", parse_quote!(i32)),
        ("partkey", parse_quote!(i32)),
        ("suppkey", parse_quote!(i32)),
        ("linenumber", parse_quote!(i32)),
        ("quantity", parse_quote!(f64)),
        ("extendedprice", parse_quote!(f64)),
        ("discount", parse_quote!(f64)),
        ("tax", parse_quote!(f64)),
        ("returnflag", parse_quote!(String)),
        ("linestatus", parse_quote!(String)),
        ("shipdate", parse_quote!(Date)),
        ("commitdate", parse_quote!(Date)),
        ("receiptdate", parse_quote!(Date)),
        ("shipinstruct", parse_quote!(String)),
        ("shipmode", parse_quote!(String)),
        ("comment", parse_quote!(String)),
    ];
    let tables: &[&[(&str, syn::Type)]] = &[&customer, &orders, &lineitem];

    for fields in tables {
        {
            let field_types: Vec<_> = fields.iter().map(|(_, ty)| ty).cloned().collect();
            let ast = generate_ast("Customer", &field_types);
            let syntax_tree = parse2(ast).unwrap();
            let formatted_code = unparse(&syntax_tree);
            println!("{formatted_code}");
        }
        {
            let ast = gen_read_fn("read_customers", &fields);
            let syntax_tree = parse2(ast).unwrap();
            let formatted_code = unparse(&syntax_tree);
            println!("{formatted_code}");
        }
    }
}
