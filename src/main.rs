// fn main() {
//     let tweet = Tweet {
//         username: String::from("the_naveen_sharma"),
//         content: String::from("Follow me to learn rust"),
//         reply: false,
//         retweet: false,
//     };

//     print!("1 new tweet {}", tweet.summarize());

//     let article = NewsArticle {
//         headline: String::from("Penguins win the Stanley Cup Championship!"),
//         location: String::from("Pittsburgh, PA, USA"),
//         author: String::from("Iceburgh"),
//         content: String::from(
//             "The Pittsburgh Penguins once again are the best \
//              hockey team in the NHL.",
//         ),
//     };

//     println!("New article available! {}", article.summarize());
//     notify(&article);
//     let x = std::f64::consts::PI;
//     let r = 8.0;
//     println!("the area of the circle is {}", x * r * r);
// }

// pub trait Summary {
//     fn summarize_autor(&self) -> String;
//     fn summarize(&self) -> String {
//         format!("(Read more from {}...)", self.summarize_autor())
//     }
// }
// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }

//     fn summarize_autor(&self) -> String {
//         todo!()
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }
// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }

//     fn summarize_autor(&self) -> String {
//         todo!()
//     }
// }

// type Meters = u32;
// impl Summary for Meters {
//     fn summarize(&self) -> String {
//         format!("{}", self)
//     }

//     fn summarize_autor(&self) -> String {
//         todo!()
//     }
// }

// pub fn notify<T:Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// preventind dangling references with lifetimes

// fn main() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     }
//     print!("r: {r}");
// }

//generic lifetimes in functions
// fn main() {
//     let string1 = String::from("long string");
//     {
//         let string2 = "xyz";
//         let result = longest(string1.as_str(), string2);
//         print!("The longest string is {result}");
//     }
// }

// fn longest<'a,'b>(str1: &'a str, str2: &'b str) -> & str {
//     if str1.len() > str2.len() {
//         str1
//     } else {
//         str2
//     }
// }

// Lifetime annotations in struct definations

// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }
// impl<'a> ImportantExcerpt<'a> {
//     fn level(&self)->i32{
//         3
//     }
//     fn announance_and_return_part(&self,announcement:&str)->&str{
//         print!("Attention please: {announcement}");
//         self.part
//     }
// }
// fn main() {
//     let novel = String::from("Call me Naveen. Some years ago...");
//     let first_sentence = novel.split('.').next().unwrap();
//     print!("{first_sentence}");
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
// }

// fn main() {
// first_word("The Naveen Sharma");
// }

// fn first_word<'a>(s: &'a str) -> &'a str {
//     let bytes = s.as_bytes();
//     for (i, &val) in bytes.iter().enumerate() {
//         if val == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

// use std::fmt::Display;

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
// fn main() {
//     let string1 = String::from("long string");
//     {
//         let string2 = "xyz";
//         let result = longest_with_an_announcement(string1.as_str(), string2, "The Naveen");
//         print!("The longest string is {result}");
//     }
// }

// fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
// where
//     T: Display,
// {
//     println!("Announcemen! {ann}");
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn main() {}
