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

use std::cell::RefCell;
use std::rc::{Rc, Weak};
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
