#![feature(proc_macro_span)]

extern crate proc_macro;

use std::error::Error;
use std::fs;
use std::ffi::OsStr;

use proc_macro::{TokenStream, Span};

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

    let input_str = input.to_string();
    let mut output = String::new();
    
    for entry in fs::read_dir(modules_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension() != Some(&OsStr::new("rs")) { continue; }
        let file_name = path.file_stem().unwrap().to_str().unwrap();

        output += &input_str.replace("PLACEHOLDER", file_name);
    }

    //eprintln!("output is: {}", output);

    Ok(output.parse()?)
}