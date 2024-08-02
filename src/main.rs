use minigrep::Config;
use std::collections::HashMap;
use std::env;
use std::i16;
use std::process;
use std::vec::Vec;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    // if let Err(e) = minigrep::run(config) {
    //     eprintln!("Application error: {e}");
    //     process::exit(1);
    // }
    make_vector()
}

//made and array from 1 to 100
fn make_vector() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let mut scores:HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team = String::from("Blue");
let score=    scores.get(&team).unwrap_or(&0);
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for a in row {
        // print!("{:?}",a)
    }
}
