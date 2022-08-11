#![allow(unused)]

// declare a module
pub mod some_mod {
    // declare a generic function
    pub fn some_function<T>(input: T) -> T {
        input
    }
}

fn main() {
    println!("{}", some_mod::some_function(45.0));
}
