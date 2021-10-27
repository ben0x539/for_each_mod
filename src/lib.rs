#![feature(proc_macro_span)]

extern crate proc_macro;

use std::error::Error;
use std::fs;
use std::ffi::OsStr;

use proc_macro::{TokenStream, Span, TokenTree, Literal, Ident, Group};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[proc_macro]
pub fn for_each_mod(input: TokenStream) -> TokenStream {
    match for_each_mod_inner(input) {
        Ok(output) => output,
        Err(e) => panic!("{:?}", e),
    }
}

fn for_each_mod_inner(input: TokenStream) -> Result<TokenStream> {
    let caller_path = Span::call_site().source_file().path();
    let caller_dir = caller_path.parent().ok_or("no dir??")?;
    let modules_dir = caller_dir.join("modules");

    let mut output = TokenStream::new();
    
    for entry in fs::read_dir(modules_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension() != Some(&OsStr::new("rs")) { continue; }
        let file_name = path.file_stem().unwrap().to_str().unwrap();
        //eprintln!("file name is {:?}", file_name);

        output.extend(replace(input.clone(), file_name));
    }

    //eprintln!("output is: {}", output.to_string());

    Ok(output)
}

fn replace<'a>(input: TokenStream, subst: &'a str) -> impl Iterator<Item=TokenTree>+'a {
    input.into_iter().map(move |token| match token {
        TokenTree::Ident(ident) if ident.to_string() == "PLACEHOLDER" =>
            TokenTree::Ident(Ident::new(subst, ident.span())),
        TokenTree::Literal(lit) if lit.to_string().starts_with("\"") => {
            let mut new_lit: Literal = lit.to_string().replace("PLACEHOLDER", subst).parse().unwrap();
            new_lit.set_span(lit.span());
            TokenTree::Literal(new_lit)
        }
        TokenTree::Group(group) => TokenTree::Group(Group::new(group.delimiter(), replace(group.stream(), subst).collect())),
        other => other,
    })
}