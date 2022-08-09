#![allow(unused)]
use std::collections::HashMap;

fn main() {
    {
        // let compiler infer type of HashMap from usage below
        let mut map: HashMap<String, String> = HashMap::new();

        // insert key and value into the map
        map.insert("rawr".to_owned(), "rawr".to_owned());

        println!("{map:#?}");
    }

    {
        // initialize a map from an array
        let mut map: HashMap<String, String> = HashMap::from([
            ("key1".to_string(), "val1".to_string()),
            ("key2".to_string(), "val2".to_string()),
            ("key3".to_string(), "val3".to_string()),
        ]);

        // iterate through the map (.iter())
        for (key, val) in &map {
            println!("{} {}", key, val);
        }

        // check if the key exists
        match map.get(&"rawr".to_string()) {
            Some(val) => println!("this is the value: {}", val),
            None => println!("key does not exist"),
        }
    }

    {
        let mut letters: HashMap<char, i32> = HashMap::new();

        for ch in "some text".chars() {
            let counter: &mut i32 = letters.entry(ch).or_insert(0);

            *counter += 1;
        }

        println!("{letters:#?}");
    }
}
