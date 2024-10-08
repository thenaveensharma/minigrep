// // use std::env;
// // use std::process;

// // use naveengrep::Config;

// // fn main() {
// //     let config = Config::build(env::args()).unwrap_or_else(|err| {
// //         eprintln!("Problem parsing arguments: {err}");
// //         process::exit(1);
// //     });

// //     if let Err(e) = naveengrep::run(config) {
// //         eprintln!("Application error: {e}");
// //         process::exit(1);
// //     }
// // }

// use std::ops::Deref;

// // fn main() {
// //     let x = 5;
// //     let y = MyBox::new(x);
// //     assert_eq!(5, x);
// //     assert_eq!(5, *y);
// // }

// // Defining our own smart pointer

// struct MyBox<T>(T);
// impl<T> Deref for  MyBox<T> {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }

// }
// impl <T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// fn hello(name:&str){
//     print!("Hello, {name}");
// }

// fn main(){
//     let m= MyBox::new(String::from("Rusht"));
//     hello(&m);
// }

// struct CustomerSmartPointer {
//     data: String,
// }
// impl Drop for CustomerSmartPointer {
//     fn drop(&mut self) {
//         println!("Droppinf CustomerSmartPointer with data `{}`", self.data);
//     }
// }

// fn main() {
//     let c = CustomerSmartPointer {
//         data: String::from("My Stuff"),
//     };
//     let d = CustomerSmartPointer {
//         data: String::from("Other Stuff"),
//     };
//     drop(c);
//     println!("CustomerSmartPointers created");
// }

// use crate::List::{Cons, Nil};
// use std::cell::RefCell;
// use std::rc::Rc;

// #[derive(Debug)]
// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil,
// }

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match self {
//             Cons(_, item) => Some(item),
//             Nil => None,
//         }
//     }
// }

// fn main() {
//     let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

//     println!("a initial rc count = {}", Rc::strong_count(&a));
//     println!("a next item = {:?}", a.tail());

//     let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

//     println!("a rc count after b creation = {}", Rc::strong_count(&a));
//     println!("b initial rc count = {}", Rc::strong_count(&b));
//     println!("b next item = {:?}", b.tail());

//     if let Some(link) = a.tail() {
//         *link.borrow_mut() = Rc::clone(&b);
//     }

//     println!("b rc count after changing a = {}", Rc::strong_count(&b));
//     println!("a rc count after changing a = {}", Rc::strong_count(&a));

//     // Uncomment the next line to see that we have a cycle;
//     // it will overflow the stack
//     // println!("a next item = {:?}", a.tail());
// }

// use std::cell::RefCell;
// use std::rc::{Rc, Weak};
// #[derive(Debug)]
// struct Node {
//     value: i32,
//     parent: RefCell<Weak<Node>>,
//     children: RefCell<Vec<Rc<Node>>>,
// }

// fn main() {
//     let leaf = Rc::new(Node {
//         value: 3,
//         parent: RefCell::new(Weak::new()),
//         children: RefCell::new(vec![]),
//     });

//     println!(
//         "leaf strong = {}, weak = {}",
//         Rc::strong_count(&leaf),
//         Rc::weak_count(&leaf),
//     );

//     {
//         let branch = Rc::new(Node {
//             value: 5,
//             parent: RefCell::new(Weak::new()),
//             children: RefCell::new(vec![Rc::clone(&leaf)]),
//         });

//         *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

//         println!(
//             "branch strong = {}, weak = {}",
//             Rc::strong_count(&branch),
//             Rc::weak_count(&branch),
//         );

//         println!(
//             "leaf strong = {}, weak = {}",
//             Rc::strong_count(&leaf),
//             Rc::weak_count(&leaf),
//         );
//     }

//     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
//     println!(
//         "leaf strong = {}, weak = {}",
//         Rc::strong_count(&leaf),
//         Rc::weak_count(&leaf),
//     );
// }

// use std::{collections::btree_map::Values, sync::mpsc, thread};

// fn main() {
// let handle=    thread::spawn(|| {
//         for i in 1..10 {
//             println!("Hi number {i} from the spawn thread!");
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//     handle.join().unwrap();

//     for i in 1..5 {
//         println!("Hi number {i} from the main thread!");
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// fn main() {
//     let vector = vec![1, 2, 3, 4];
//     let handle = thread::spawn(move || {
//         print!("{vector:?}");
//     });
//     handle.join().unwrap();
// }

// fn main(){
//     let (tx,rx)=mpsc::channel();
//     thread::spawn(move||{
//         let val=String::from("Naveen");
//         tx.send(val).unwrap();
//         println!("val is {val}");
//     });
//     let received=rx.recv().unwrap();
//     println!("Got: {received}");
// }
// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let tx1 = tx.clone();
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];

//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("more"),
//             String::from("messages"),
//             String::from("for"),
//             String::from("you"),
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for received in rx {
//         println!("Got: {received}");
//     }
// }

// use std::sync::{Arc, Mutex};
// use std::{result, thread};

// fn main() {
//     let counter = Arc::new( Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let counter=Arc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }

// pub struct AverageCollection{
//     list:Vec<i32>,
//     average:f64
// }
// impl AverageCollection {
//     pub fn add (&mut self,value:i32){
//         self.list.push(value);
//         self.update_average();
//     }
// pub fn remove(&mut self)->Option<i32> {
//     let result=self.list.pop();
//     match result {
//         Some(value)=>{
//             self.update_average();
//             Some(value)
//         }
//         None=>None,
//     }
// }
// pub fn average(&self)->f64{
//     self.average
// }
// pub fn update_average(&mut self) {
//     let total:i32=self.list.iter().sum();
//     self.average=total as f64 /self.list.len() as f64;
// }
// }
// use naveengrep::Post;
// fn main() {
//     let mut post = Post::new();
//     post.add_text("I ate a salad for lunch today");
//     assert_eq!("", post.content());
//     // post.request_review();
//     // assert_eq!("", post.content());
//     // post.approve();
//     // assert_eq!("I ate a salad for lunch today", post.content());
// }

// fn main() {
// let favourite_color: Option<&str> = None;
// let is_tuesday = false;
// let age: Result<u8, _> = "34".parse();

// if let Some(color) = favourite_color {
//     println!("Using your favorite color, {color}, as the background");
// } else if is_tuesday {
//     println!("Tuesday is green day!");
// } else if let Ok(age) = age {
//     if age > 30 {
//         println!("Using purple as the background color");
//     } else {
//         println!("Using orange as the background color");
//     }
// } else {
//     println!("Using blue as the background color");
// }
// let x = 1;
// match x {
//     1..=5 => println!("{x}"),
//     _ => println!("Everything"),
// }
// let x = Some(5);
// let y = 10;

// match x {
//     Some(50) => println!("Got 50"),
//     Some(y) => println!("Matched, y = {y}"),
//     _ => println!("Default case, x = {x:?}"),
// }

// println!("at the end: x = {x:?}, y = {y}");

// let x=1;
// match x {
//     1|2 =>println!("one or two"),
//     _=> println!("other")
// }
// enum Message {
//     Hello { id: i32 },
// }
// let msg = Message::Hello { id: 5 };
// match msg {
//     Message::Hello {
//         id: id_variable @ 3..=7,
//     } => println!(
//         "
//     Found an id in range: {id_variable}"
//     ),
//     Message::Hello { id } => print!("Found some other id: {id}"),
// }
// }

use std::slice;

// fn main() {
//     // learn unsafe rust

//     // ----------Superpowers hold by unsafe rust-------------

//     // Dereference a raw pointer
//     // Call an unsafe function or method
//     // Access or modify a mutable static variable
//     // Implement an unsafe trait
//     // Access fields of a union

//     // let mut num = 5;
//     // let r1 = &num as *const i32;
//     // let r2 = &mut num as *mut i32;
//     // unsafe {
//     //     print!("{:?},{:?}",*r1,*r2);
//     //     dangerous();
//     // }

//     let mut v=vec![1,2,3,4,5,6];
//     let r=&mut v[..];
//     let (a,b)=r.split_at_mut(3);
//     assert_eq!(a, &mut [1, 2, 3]);
//     assert_eq!(b, &mut [4, 5, 6]);
// }

// unsafe fn dangerous(){}

// fn split_at_mut(values:&mut [i32],mid:usize)->(&mut [i32],&mut [i32]){
//     let len=values.len();
//     let ptr=values.as_mut_ptr();
//     assert!(mid<=len);
//     unsafe {
//         (
//             slice::from_raw_parts_mut(ptr, mid),
//             slice::from_raw_parts_mut(ptr.add(mid), len-mid)
//         )
//     }
// }

use std::ops::Add;

// fn main() {
//     assert_eq!(
//         Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
//         Point { x: 3, y: 3 }
//     );
// }

// #[derive(Debug, Copy, Clone, PartialEq)]
// struct Point {
//     x: i32,
//     y: i32,
// }
// impl Add for Point {
//     type Output = Point;
//     fn add(self, other: Point) -> Point {
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() {
        print!("Animal")
    }
}
struct Dog;
impl Animal for Dog {
    fn baby_name() {
        print!("Puppy");
    }
}
fn main() {
    let person = Human;
    person.fly();
    Wizard::fly(&person);
    Pilot::fly(&person);

    Dog::baby_name();
    <Dog as Animal>::baby_name();
    let p = Point { x: 5, y: 10 };
    println!("point = {p}");
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}

use std::fmt::{self, write};

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn generic<T: ?Sized>(t: &T) {
    
}

#[macro_export]
macro_rules! vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec=Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}