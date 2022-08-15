fn main() {
    // implicit types
    let add_two_numbers = |num1, num2| num1 + num2;
    println!("{sum}", sum = add_two_numbers(1, 2));

    // explicit types
    let add_two_strings =
        |string1: String, string2: String| -> String { format!("{string1} {string2}") };
    println!(
        "{concatenated_string}",
        concatenated_string = add_two_strings("Hello".to_string(), "world".to_string())
    );
}
