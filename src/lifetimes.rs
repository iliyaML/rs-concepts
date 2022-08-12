#![allow(unused)]

// generic lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // example of invalid reference
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {r}");

    // references within the same scope
    let x = 5;
    let r = &x;
    println!("r: {r}");

    // example
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());

    println!("The longest string is {result}");

    // example
    // let string1 = String::from("abcd");
    // let result;

    // {
    //     let string2 = String::from("abcd");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");

    // example
    let novel = "Call me Iliya".to_string();
    let first_sentence = novel.split('.').next().expect("Could not find .");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // static lifetime
    let name: &'static str = "Iliya";
}
