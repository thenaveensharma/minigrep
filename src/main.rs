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

struct CustomerSmartPointer{
    data:String,
}
impl Drop for CustomerSmartPointer {
    fn drop(&mut self) {
        println!("Droppinf CustomerSmartPointer with data `{}`",self.data);
    }
    
}

fn main(){
    let c=CustomerSmartPointer{
        data:String::from("My Stuff")
        
    };
    let d=CustomerSmartPointer{
        data:String::from("Other Stuff")

    };
    println!("CustomerSmartPointers created");
}