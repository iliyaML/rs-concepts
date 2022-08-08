fn main() {
    let names = vec![
        "peter".to_owned(),
        "lois".to_owned(),
        "chris".to_owned(),
        "meg".to_owned(),
        "stewie".to_owned(),
    ];

    // .iter() take as reference
    for name in names.iter() {
        println!("{name}");
    }

    // .into_iter() take as value
    for name in names.into_iter() {
        println!("{name}");
    }

    // names no longer valid here

    let names = vec![
        "peter".to_owned(),
        "lois".to_owned(),
        "chris".to_owned(),
        "meg".to_owned(),
        "stewie".to_owned(),
    ];

    let ans: Vec<_> = names
        .iter()
        .map(|string| string.to_owned() + &" said hello world")
        .collect();

    println!("{names:?}");
    println!("{ans:?}");

    // map-filter-fold
    let v = [1, 2, 3, 4, 5];

    let item = v
        .iter()
        .map(|x| *x + 3)
        .filter(|x| *x > 2)
        .fold(0, |total, next| total + next);
    println!("{item}");
}
