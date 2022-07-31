use std::io;

fn main() {
    // create an empty mutable string
    let mut name = String::new();

    println!("Please enter your name: ");

    // get user input
    io::stdin()
        .read_line(&mut name) // bind user input to name
        .expect("Please enter a text");

    println!("My name is {name}");
}
