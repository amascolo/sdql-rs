use prettyplease::unparse;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse2, parse_quote, parse_str, Ident, Type};

fn parse_type(t: &str) -> Type {
    match t {
        "i32" => parse_quote!(i32),
        "String" => parse_quote!(String),
        "f64" => parse_quote!(f64),
        _ => parse_str(t).unwrap(),
    }
}

fn generate_ast(type_name: &str, field_types: &[Type]) -> TokenStream {
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

fn generate_reader_function(
    function_name: &str,
    struct_name: &str,
    fields: &[(&str, Type)],
) -> TokenStream {
    let function_ident = Ident::new(function_name, Span::call_site());
    let struct_ident = Ident::new(struct_name, Span::call_site());

    let field_declarations = fields.iter().map(|(name, _ty)| {
        let field_ident = Ident::new(name, Span::call_site());
        quote! {
            let mut #field_ident = Vec::new();
        }
    });

    let field_parsing = fields.iter().enumerate().map(|(idx, (name, ty))| {
        let field_ident = Ident::new(name, Span::call_site());
        if *ty == parse_quote!(String) {
            quote! {
                #field_ident.push(record.get(#idx).unwrap().to_string());
            }
        } else {
            quote! {
                #field_ident.push(record.get(#idx).unwrap().parse()?);
            }
        }
    });

    let field_tuple = fields.iter().map(|(name, _ty)| {
        let field_ident = Ident::new(name, Span::call_site());
        quote! { #field_ident }
    });

    quote! {
        use std::error::Error;
        use csv::ReaderBuilder;

        fn #function_ident(path: &str) -> Result<#struct_ident, Box<dyn Error>> {
            let mut reader = ReaderBuilder::new()
                .has_headers(false)
                .delimiter(b'|')
                .from_path(path)?;

            #(#field_declarations)*
            let mut size = 0;

            for result in reader.records() {
                let record = result?;
                #(#field_parsing)*
                size += 1;
            }

            Ok((
                #(#field_tuple),*,
                size
            ))
        }
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
        ("shipdate", parse_quote!(i32)),
        ("commitdate", parse_quote!(i32)),
        ("receiptdate", parse_quote!(i32)),
        ("shipinstruct", parse_quote!(String)),
        ("shipmode", parse_quote!(String)),
        ("comment", parse_quote!(String)),
    ];
    let tables: &[&[(&str, Type)]] = &[&customer, &orders, &lineitem];

    for fields in tables {
        {
            let field_types: Vec<_> = fields.iter().map(|(_, ty)| ty).cloned().collect();
            let ast = generate_ast("Customer", &field_types);
            let syntax_tree = parse2(ast).unwrap();
            let formatted_code = unparse(&syntax_tree);
            println!("{formatted_code}");
        }
        {
            let ast = generate_reader_function("read_customers", "Customer", &fields);
            let syntax_tree = parse2(ast).unwrap();
            let formatted_code = unparse(&syntax_tree);
            println!("{formatted_code}");
        }
    }
}
