use prettyplease::unparse;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{File, Ident};

fn generate_ast(type_name: &str, field_types: &[&str]) -> TokenStream {
    let type_ident = Ident::new(type_name, Span::call_site());

    let field_definitions = field_types.iter().map(|ty| {
        let ty_ident = Ident::new(ty, Span::call_site());
        quote! {
            Vec<#ty_ident>,
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
    let field_types = vec![
        "i32", "String", "String", "i32", "String", "f64", "String", "String",
    ];
    let ast = generate_ast("Customer", &field_types);
    let syntax_tree: File = syn::parse2(ast).unwrap();
    let formatted_code = unparse(&syntax_tree);
    println!("{formatted_code}");
}
