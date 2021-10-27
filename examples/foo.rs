extern crate for_each_mod;
use for_each_mod::*;

for_each_mod! {
    mod PLACEHOLDER;
}

fn main() {
    let mut v = Vec::new();
    for_each_mod! {
        v.push(("PLACEHOLDER", PLACEHOLDER::lol()));
    }

    eprintln!("{:?}", v);
}