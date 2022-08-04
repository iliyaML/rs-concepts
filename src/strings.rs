#![allow(unused)]

fn my_function(a: &str) -> String {
    format!("{a}")
}

fn main() {
    // string slice type - stack allocated
    let s1 = "i am s1";

    // String type - heap allocated
    let s2: String = String::from("i am s2");
    let s3: String = "i am s3".to_string();
    let s4: String = "i am s4".to_owned();
    let s5: String = "i am s5".into(); // must declare type

    // string slice
    let s5: &str = &s4[..];

    println!("{s5}");

    // string manipulation
    let mut s = String::from("foo");
    println!("{s}");
    s.push_str("bar");
    println!("{s}");
    s.replace_range(.., "baz");
    println!("{s}");

    // concatenate Strings
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = s1 + " " + &s2;
    println!("{s3}");
    // s1 is no longer valid here as its value has been moved to s3

    // String concatention using format macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = format!("{s1}-{s2}-{s3}");
    println!("{s3}");

    // other string concatenation methods
    let s1: String = ["first", "second"].concat();
    let s2: &str = concat!("first", "second");
    println!("{s1}");
    println!("{s2}");

    // indexing into strings are not allowed

    let s = "Hello world".as_bytes();
    println!("{s:?}");

    // looping through bytes in a String
    let s = "Hello world".to_string();
    for b in s.as_bytes() {
        println!("{b}");
    }

    // looping through characters in a str slice
    for c in "Hello world".chars() {
        println!("{c}");
    }

    // example - deref coercion
    let s1: &str = "Hello world";
    let s2: String = String::from("Hello world");

    let s3: String = my_function(s1);
    let s4: String = my_function(&s2);

    println!("{s1} {s2} {s3} {s4}");
}
