// use std::env;
// use std::process;

// use naveengrep::Config;

// fn main() {
//     let config = Config::build(env::args()).unwrap_or_else(|err| {
//         eprintln!("Problem parsing arguments: {err}");
//         process::exit(1);
//     });

//     if let Err(e) = naveengrep::run(config) {
//         eprintln!("Application error: {e}");
//         process::exit(1);
//     }
// }

use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}
fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
