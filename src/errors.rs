fn main() {
    // {
    //     // intentional panic
    //     panic!("Farewell!");
    // }

    // {
    //     // unintentional panic - accessing an index not present in an array/vector
    //     let v = vec![0, 1, 2, 3, 4];
    //     println!("{}", v[6]);
    // }

    {
        let fruits: Vec<&str> = vec!["banana", "apple", "coconut", "orange", "strawberry"];

        // pick the first item:
        let first: Option<&&str> = fruits.get(0);
        println!("{:?}", first);

        // pick the third item:
        let third: Option<&&str> = fruits.get(2);
        println!("{:?}", third);

        // pick the 99th item, which is non-existent:
        let non_existent: Option<&&str> = fruits.get(99);
        println!("{:?}", non_existent);
    }

    {
        let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
        for &index in [0, 2, 99].iter() {
            match fruits.get(index) {
                Some(&"coconut") => println!("Coconuts are awesome!!!"),
                Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
                None => println!("There is no fruit! :("),
            }
        }
    }
}
