// use std::env;
// use std::process;

// use minigrep::Config;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let config = Config::build(&args).unwrap_or_else(|err| {
//         eprintln!("Problem parsing arguments: {err}");
//         process::exit(1);
//     });

use std::thread;

//     if let Err(e) = minigrep::run(config) {
//         eprintln!("Application error: {e}");
//         process::exit(1);
//     }
// }
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let mut list = vec![1, 2, 3];
    print!("Before defining closure: {list:?}");
    let mut only_borrows = || list.push(4);
    only_borrows();
    print!("After defining closure: {list:?}");

    thread::spawn(move || print!("From thread: {list:?}"))
        .join()
        .unwrap();

    let mut num_sort_operations = 0;
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.height
    });
    print!("{list:#?}");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
