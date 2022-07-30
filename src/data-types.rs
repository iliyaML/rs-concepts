#![allow(unused)]

fn main() {
    // scalar types
    let status: bool = true;

    let num: i32 = 32; // integers default to i32
    let num: f64 = 32.0; // floating numbers default to f64
    let usize_example: usize = 1;
    let isize_example: isize = -1;

    // character
    let c = 'z';
    let z: char = 'Z';
    let cat = '😻';
    println!("{c} {z} {cat}");

    // str slice and String
    let str_example: &str = "Hello world";
    let string_example: String = String::from("Hello world");

    // compound types - tuple and array
    let tup: (i32, f64, u32) = (500, 6.4, 1);
    let arr: [i32; 3] = [1, 2, 3];

    println!("{} {} {}", tup.0, tup.1, tup.2);
    println!("{} {} {}", arr[0], arr[1], arr[2]);

    // destructuring
    let (num1, num2, num3) = tup;

    println!("{num1}, {num2}, {num3}");
}
