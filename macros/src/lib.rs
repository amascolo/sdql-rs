// TODO auto-generated, rewrite more robustly (to avoid stack overflows)

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
    token, Error as SynError, ExprMacro, Ident, LitBool, LitFloat, LitInt, LitStr, Token,
};

#[proc_macro]
pub fn sdql_static(input: TokenStream) -> TokenStream {
    let ts2: TokenStream2 = input.clone().into();
    let expanded = try_expand_include(&ts2).unwrap_or(ts2);
    match syn::parse2::<SdqlValue>(expanded.clone()) {
        Ok(ast) => ast.into_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn try_expand_include(ts: &TokenStream2) -> Option<TokenStream2> {
    let expr_mac: ExprMacro = syn::parse2(ts.clone()).ok()?;
    if !expr_mac.mac.path.is_ident("include") {
        return None;
    }
    let inner = expr_mac.mac.tokens.clone();
    let path = parse_path_literal_or_concat(inner).ok()?;
    read_file_to_tokens(&path).ok()
}

fn parse_path_literal_or_concat(ts: TokenStream2) -> syn::Result<String> {
    if let Ok(litstr) = syn::parse2::<LitStr>(ts.clone()) {
        return Ok(litstr.value());
    }
    let mac: ExprMacro = syn::parse2(ts)?;
    if !mac.mac.path.is_ident("concat") {
        return Err(SynError::new(mac.mac.path.span(), "expected `concat!(…)`"));
    }
    let parser = Punctuated::<LitStr, Token![,]>::parse_terminated;
    let groups = parser.parse2(mac.mac.tokens.clone()).map_err(|e| {
        SynError::new(
            mac.mac.tokens.span(),
            format!("invalid `concat!(…)` args: {}", e),
        )
    })?;
    Ok(groups.into_iter().map(|s| s.value()).collect())
}

fn read_file_to_tokens(path: &str) -> Result<TokenStream2, SynError> {
    use std::fs;
    let content = fs::read_to_string(path).map_err(|e| {
        SynError::new(
            Span::call_site(),
            format!("failed to read file `{}`: {}", path, e),
        )
    })?;
    content
        .parse::<TokenStream2>()
        .map_err(|e| SynError::new(Span::call_site(), format!("token parse error: {}", e)))
}

enum SdqlValue {
    Map(Vec<(SdqlValue, SdqlValue)>),
    Record(Vec<SdqlValue>),
    Date(LitInt),
    Bool(bool),
    Int(LitInt),
    Float(LitFloat),
    NegativeInt(LitInt),
    NegativeFloat(LitFloat),
    StringLit(LitStr),
}

impl Parse for SdqlValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(token::Brace) {
            return parse_map(input);
        }
        if input.peek(Token![<]) {
            return parse_record(input);
        }
        if input.peek(Token![-]) {
            let _minus: Token![-] = input.parse()?;
            if input.peek(LitFloat) {
                let lf: LitFloat = input.parse()?;
                return Ok(SdqlValue::NegativeFloat(lf));
            } else if input.peek(LitInt) {
                let li: LitInt = input.parse()?;
                return Ok(SdqlValue::NegativeInt(li));
            } else {
                return Err(syn::Error::new(
                    input.span(),
                    "expected numeric literal after '-'",
                ));
            }
        }
        if input.peek(LitBool) {
            let lb: LitBool = input.parse()?;
            return Ok(SdqlValue::Bool(lb.value()));
        }
        if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            match ident.to_string().as_str() {
                "date" => {
                    let content;
                    syn::parenthesized!(content in input);
                    let inner: LitInt = content.parse()?;
                    return Ok(SdqlValue::Date(inner));
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        ident,
                        "expected `date(...)` or recognized keyword",
                    ));
                }
            }
        }
        if input.peek(LitFloat) {
            let lf: LitFloat = input.parse()?;
            return Ok(SdqlValue::Float(lf));
        }
        if input.peek(LitInt) {
            let li: LitInt = input.parse()?;
            return Ok(SdqlValue::Int(li));
        }
        if input.peek(LitStr) {
            let ls: LitStr = input.parse()?;
            return Ok(SdqlValue::StringLit(ls));
        }
        Err(syn::Error::new(
            input.span(),
            "unrecognized token in sdql_static!()",
        ))
    }
}

fn parse_map(input: ParseStream) -> syn::Result<SdqlValue> {
    let content;
    let _brace = syn::braced!(content in input);

    let mut pairs = Vec::new();
    while !content.is_empty() {
        let key = SdqlValue::parse(&content)?;
        let _arrow: Token![->] = content.parse()?;
        let val = SdqlValue::parse(&content)?;
        pairs.push((key, val));

        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
        } else {
            break;
        }
    }

    Ok(SdqlValue::Map(pairs))
}

fn parse_record(input: ParseStream) -> syn::Result<SdqlValue> {
    let _lt: Token![<] = input.parse()?;
    let mut values = Vec::new();

    while !input.peek(Token![>]) {
        values.push(SdqlValue::parse(input)?);
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        } else {
            break;
        }
    }

    let _gt: Token![>] = input.parse()?;
    Ok(SdqlValue::Record(values))
}

impl SdqlValue {
    fn into_token_stream(self) -> proc_macro2::TokenStream {
        match self {
            SdqlValue::Map(pairs) => {
                let inserts = pairs.into_iter().map(|(k, v)| {
                    let k_ts = k.into_token_stream();
                    let v_ts = v.into_token_stream();
                    quote! {
                        __map.insert(#k_ts, #v_ts);
                    }
                });
                quote! {{
                    let mut __map = ::sdql_runtime::HashMap::new();
                    #( #inserts )*
                    __map
                }}
            }
            SdqlValue::Record(vals) => {
                let elems = vals.into_iter().map(|v| v.into_token_stream());
                quote! {
                    ::sdql_runtime::Record::new(( #( #elems ),* ))
                }
            }
            SdqlValue::Date(li) => quote! {
                ::sdql_runtime::date!(#li)
            },
            SdqlValue::Bool(true) => quote! { ::sdql_runtime::TRUE },
            SdqlValue::Bool(false) => quote! { ::sdql_runtime::FALSE },
            SdqlValue::Int(li) => quote! { #li },
            SdqlValue::Float(lf) => quote! { ::sdql_runtime::OrderedFloat(#lf) },
            SdqlValue::NegativeInt(li) => quote! { -(#li) },
            SdqlValue::NegativeFloat(lf) => quote! { ::sdql_runtime::OrderedFloat(-#lf) },
            SdqlValue::StringLit(ls) => quote! {
                ::sdql_runtime::VarChar::from(#ls).unwrap()
            },
        }
    }
}
