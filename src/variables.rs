#![allow(unused)]

fn main() {
    // variables are immutable by default
    let x: i32 = 1;
    println!("The value of x is {x}");

    // shadowing
    // mark x as mutable
    let mut x: i32 = 5;
    println!("The value of x is {x}");
    // throws an error if x is not marked as mutable
    x = 6;
    println!("The value of x is: {x}");

    // constants - must include type and value
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");

    // even more shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // changing type with shadowing
    let spaces = "    ";
    let spaces = spaces.len();

    let mut spaces = "    ";
    // not allowed to mutate a variable's type
    // spaces = spaces.len();
}
