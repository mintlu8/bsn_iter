#![doc = include_str!("../README.md")]
#![allow(clippy::collapsible_if)]
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned};
const SYMBOL: char = '$';

fn delimiter_allowed(delimiter: Delimiter) -> bool {
    delimiter == Delimiter::Parenthesis
}

/// `bsn!` macro with iteration syntax.
#[proc_macro]
pub fn bsni(token_stream: TokenStream1) -> TokenStream1 {
    let token_stream: TokenStream = token_stream.into();
    let result = root(token_stream.into_iter().collect());
    quote! {bsn!{#result}}.into()
}

/// `bsn_list!` macro with iteration syntax.
#[proc_macro]
pub fn bsni_list(token_stream: TokenStream1) -> TokenStream1 {
    let token_stream: TokenStream = token_stream.into();
    let result = root(token_stream.into_iter().collect());
    quote! {bsn_list!{#result}}.into()
}

fn parse_iter(token_stream: Vec<TokenTree>, prev: &mut Vec<Ident>, span: Span) -> TokenStream {
    let prev_len = prev.len();
    let is_scene_list = is_scene_list(&token_stream);
    let mut idents = Vec::new();
    let mut content = inner(token_stream, prev, &mut idents);
    if is_scene_list {
        content = quote! {{__result.push(Box::new(bsn_list!(#content)) as Box<dyn SceneList>)}};
    } else {
        content = quote! {{__result.push(bsn!(#content))}};
    }
    prev.truncate(prev_len);
    let err = if idents.is_empty() {
        quote_spanned! {span => compile_error!("Expected at least one iterator at this level.");}
    } else {
        quote! {}
    };
    quote! {{#[allow(unused)]{
        let mut __result = Vec::new();
        #(let mut #idents = #idents.into_iter();)*
        loop {
            #err
            #(let Some(#idents) = #idents.next() else {break};)*
            #content
        }
        __result
    }}}
}

fn is_scene_list(stream: &[TokenTree]) -> bool {
    stream.iter().any(|x| {
        if let TokenTree::Punct(p) = x
            && p.as_char() == ','
        {
            true
        } else {
            false
        }
    })
}

fn root(tokens: Vec<TokenTree>) -> TokenStream {
    let mut result = Vec::new();
    let mut idx = 0;
    while let Some(item) = tokens.get(idx) {
        if let TokenTree::Punct(p) = &item
            && p.as_char() == SYMBOL
        {
            if let Some(TokenTree::Group(g)) = tokens.get(idx + 1)
                && delimiter_allowed(g.delimiter())
                && let Some(TokenTree::Punct(p)) = tokens.get(idx + 2)
                && p.as_char() == '*'
            {
                result.extend(parse_iter(
                    g.stream().into_iter().collect(),
                    &mut Vec::new(),
                    g.span(),
                ));
                idx += 3;
                continue;
            }
        } else if let TokenTree::Group(g) = item {
            let new_stream = root(g.stream().into_iter().collect());
            let new_group = TokenTree::Group(Group::new(g.delimiter(), new_stream));
            result.push(new_group);
            idx += 1;
            continue;
        }
        result.push(item.clone());
        idx += 1;
    }

    result.into_iter().collect()
}

fn inner(tokens: Vec<TokenTree>, prev: &mut Vec<Ident>, idents: &mut Vec<Ident>) -> TokenStream {
    let mut result = Vec::new();
    let mut idx = 0;
    while let Some(item) = tokens.get(idx) {
        if let TokenTree::Punct(p) = &item
            && p.as_char() == SYMBOL
        {
            if let Some(TokenTree::Group(g)) = tokens.get(idx + 1)
                && delimiter_allowed(g.delimiter())
                && let Some(TokenTree::Punct(p)) = tokens.get(idx + 2)
                && p.as_char() == '*'
            {
                result.extend(parse_iter(g.stream().into_iter().collect(), prev, g.span()));
                idx += 3;
                continue;
            } else if let Some(TokenTree::Ident(i)) = tokens.get(idx + 1) {
                if !prev.contains(i) {
                    idents.push(i.clone());
                    prev.push(i.clone());
                }
                result.push(TokenTree::Ident(i.clone()));
                idx += 2;
                continue;
            }
        } else if let TokenTree::Group(g) = item {
            let new_stream = inner(g.stream().into_iter().collect(), prev, idents);
            let new_group = TokenTree::Group(Group::new(g.delimiter(), new_stream));
            result.push(new_group);
            idx += 1;
            continue;
        }
        result.push(item.clone());
        idx += 1;
    }

    result.into_iter().collect()
}
