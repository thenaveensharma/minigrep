use core::hash;
use core::num;
use minigrep::Config;
use std::collections::HashMap;
use std::env;
use std::i16;
use std::process;
use std::vec;
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
    // make_vector()
    // let vect = Vec::from([155, 1513, 62, 11, 18, 18, 18]);
    // let median_of_vector = find_median(vect.clone());
    // print!("Median of vector is {}", median_of_vector);
    // let median_of_vector = find_mode(vect.clone());
    // print!("Mode of vector is {}", median_of_vector);
    let string = String::from("apple naveen sharma");
    let res = convert_to_pig_latin(string);
    print!("{res}");
}

//made and array from 1 to 100
fn make_vector() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team = String::from("Blue");
    let score = scores.get(&team).unwrap_or(&0);
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for a in row {
        // print!("{:?}",a)
    }
}

// Solving exercises

//Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map
//will be helpful here) of the list.
fn find_median(mut vector: Vec<i32>) -> i32 {
    vector.sort();
    let length_of_vector = vector.len();
    print!("{:?} {length_of_vector}", vector);

    if length_of_vector % 2 == 0 {
        return vector[length_of_vector / 2 + 1];
    } else {
        return vector[length_of_vector / 2];
    }
}

fn find_mode(vector: Vec<i32>) -> i32 {
    let mut hashmap = HashMap::new();

    for element in vector {
        if hashmap.contains_key(&element) {
            if let Some(count) = hashmap.get_mut(&element) {
                *count += 1;
            }
        } else {
            hashmap.insert(element, 1);
        }
    }
    let mut max_value = 0;
    let mut max_key = 0;
    for (&key, &value) in hashmap.iter() {
        if value > max_value {
            max_value = value;
            max_key = key;
        }
    }

    max_key
}
/*
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added,
so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay).
Keep in mind the details about UTF-8 encoding! */

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn convert_to_pig_latin(s: String) -> String {
    let mut result = String::new();

    for word in s.split_whitespace() {
        let mut chars = word.chars();
        if let Some(first_char) = chars.next() {
            if VOWELS.contains(&first_char) {
                // Words that start with a vowel
                result.push_str(&format!("{}-hay", word));
            } else {
                // Words that start with a consonant
                let mut rest_of_word = word.to_owned();
                let first_consonant = rest_of_word.remove(0); // Remove the first character
                result.push_str(&format!("{}-{}ay", rest_of_word, first_consonant));
            }
        }
        result.push(' '); // Add a space after each word
    }

    // Trim the trailing space and return the result
    result.trim_end().to_string()
}
