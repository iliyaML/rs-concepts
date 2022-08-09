#![allow(unused)]

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct GenericPoint<T, U> {
    x: T,
    y: U,
}

fn main() {
    let p1: Point = Point { x: 10, y: 5 };

    let p2: GenericPoint<f64, i32> = GenericPoint { x: 10.0, y: 5 };
    let p3: GenericPoint<i32, f64> = GenericPoint { x: 5, y: 10.0 };
    let p4: GenericPoint<i32, i32> = GenericPoint { x: 5, y: 10 };

    println!("{p1:?}");
    println!("{p2:?}");
    println!("{p3:?}");
    println!("{p4:?}");
}
