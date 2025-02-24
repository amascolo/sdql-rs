use prettyplease::unparse;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, parse_str, File, Type};

fn parse_type(t: &str) -> Type {
    match t {
        "i32" => parse_quote!(i32),
        "String" => parse_quote!(String),
        "f64" => parse_quote!(f64),
        _ => parse_str(t).unwrap(),
    }
}

fn generate_ast(type_name: &str, field_types: &[Type]) -> TokenStream {
    let type_ident = syn::Ident::new(type_name, proc_macro2::Span::call_site());
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

fn main() {
    let dynamic_types = [
        "i32", "String", "String", "i32", "String", "f64", "String", "String",
    ];
    let field_types: Vec<Type> = dynamic_types.iter().map(|t| parse_type(t)).collect();
    let ast = generate_ast("Customer", &field_types);
    let syntax_tree: File = syn::parse2(ast).unwrap();
    let formatted_code = unparse(&syntax_tree);
    println!("{formatted_code}");
}
