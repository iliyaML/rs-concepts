#![allow(unused)]

fn get_ownership_of_vector(v: Vec<i32>) {
    println!("{v:?}");
}

fn borrow_vector(v: &Vec<i32>) {
    println!("{v:?}");
}

fn main() {
    // create an empty mutable vector
    let mut v: Vec<i32> = Vec::new();

    // push items into the vector
    v.push(1);
    v.push(2);
    v.push(3);

    println!("{v:?}");

    // indexing items
    println!("{} {} {}", v[0], v[1], v[2]);

    // intialze a vector from an array
    let arr: [String; 3] = [
        "peter".to_string(),
        "griffin".to_string(),
        "lois".to_string(),
    ];

    let mut v: Vec<String> = Vec::from(arr.clone());

    // arr and v exists independently
    println!("{v:?}");
    println!("{arr:?}");

    // convert array to vector
    let arr: [String; 3] = [
        "string_1".to_string(),
        "string_2".to_string(),
        "string_3".to_string(),
    ];

    let vec: Vec<String> = arr.to_vec();
    println!("{arr:?}");

    // initialize a vector with a capacity of 3
    let mut vec: Vec<i32> = Vec::with_capacity(3);

    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("{vec:?}");

    // if we push a new item,
    // the vector needs to be reallocated with a bigger capacity
    vec.push(4);

    println!("{vec:?}");

    // borrow a vector
    let v = vec![1, 2, 3];
    borrow_vector(&v);

    println!("{v:?}");

    get_ownership_of_vector(v);

    // line below will throw an error
    // println!("{v:?}");
}
