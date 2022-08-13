#![allow(unused)]

fn another_function() {
    println!("Another function");
}

fn take_ownership(item: String) {
    println!("Took ownership of this variable: {item}");
}

fn own_vector(vector: Vec<String>) {
    println!("Own Vector: {vector:?}");
}

fn borrow_vector(slice: &[String]) {
    println!("Vector: {slice:?}");
}

fn generic_function<T: std::fmt::Display>(item: T) {
    println!("Generic function: {item}");
}

fn main() {
    println!("Main function");

    another_function();

    {
        let string_1 = String::from("hello world");
        take_ownership(string_1);
        // string_1 is no longer valid down here
        // println!("{string_1}");
    }

    {
        generic_function("me");
    }

    {
        let v = vec!["this".to_string(), "is".to_string(), "me".to_string()];
        own_vector(v);
        // v is no longer valid down here
        // println!("v from here: {v:?}");
    }

    {
        let v = vec!["this".to_string(), "is".to_string(), "me".to_string()];
        borrow_vector(&v[..]);
        // v is still valid here
        println!("v from here: {v:?}");
    }
}
