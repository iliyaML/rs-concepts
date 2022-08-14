#![allow(unused)]

fn main() {
    let name = "iliya".to_owned();
    let b = &name;
    // let &c = b;

    let x = ["hi".to_owned(), "my".to_owned(), "name".to_owned()];

    for i in &x {
        println!("{i}");
    }

    // println!("{x:?}");
}
