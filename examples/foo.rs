extern crate for_each_mod;
use for_each_mod::*;

for_each_mod! {
    #[path = "modules/PLACEHOLDER.rs"]
    mod PLACEHOLDER;
}

fn main() {
    let mut v = Vec::new();
    for_each_mod! {
        v.push(("PLACEHOLDER", PLACEHOLDER::lol()));
    }

    eprintln!("{:?}", v);
}