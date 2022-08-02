#![allow(unused)]

fn get_ownership_of_array(arr: [String; 3]) {
    println!("{arr:?}");
}

fn borrow_array(arr: &[String]) {
    println!("{arr:?}");
}

fn main() {
    // intialize an array with default values of 1
    let brand_new_arr: [i32; 10] = [1; 10];
    println!("{length}", length = brand_new_arr.len());
    println!("{brand_new_arr:?}");

    // mutable array of i32 - has fixed type and size
    let mut nums_arr: [i32; 3] = [1, 2, 3];

    for &i in &nums_arr {
        println!("{i}");
    }

    println!("{nums_arr:?}");

    // change the values of the array using index
    nums_arr[0] = 3;
    nums_arr[1] = 4;
    nums_arr[2] = 5;

    println!("{nums_arr:?}");

    // array of strings
    let mut string_arr: [String; 3] = [
        "string 1".to_string(),
        "string 2".to_string(),
        "string 3".to_string(),
    ];

    for string in &string_arr {
        println!("{string}");
    }

    for (index, item) in string_arr.iter().enumerate() {
        println!("{index}. {item}");
    }

    println!("{string_arr:?}");

    // pass ownership of array
    get_ownership_of_array(string_arr);

    // no longer valid
    // println!("{string_arr:?}");

    let mut string_arr: [String; 3] = [
        "string 1".to_string(),
        "string 2".to_string(),
        "string 3".to_string(),
    ];

    // slice
    borrow_array(&string_arr[..]);

    println!("{string_arr:?}");

    // destructuring an array
    let new_arr: [String; 2] = ["string_1".to_string(), "string_2".to_string()];
    let [string_1, string_2] = new_arr;

    println!("{string_1} {string_2}");

    // new_arr is no longer valid here
}
