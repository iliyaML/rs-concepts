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

#[derive(Debug)]
struct Container<T> {
    value: T,
}

// implement method(s) for generic container
impl<T> Container<T> {
    pub fn new(value: T) -> Container<T> {
        Container { value }
    }
}

fn main() {
    {
        let p1: Point = Point { x: 10, y: 5 };

        let p2: GenericPoint<f64, i32> = GenericPoint { x: 10.0, y: 5 };
        let p3: GenericPoint<i32, f64> = GenericPoint { x: 5, y: 10.0 };
        let p4: GenericPoint<i32, i32> = GenericPoint { x: 5, y: 10 };

        println!("{p1:?}");
        println!("{p2:?}");
        println!("{p3:?}");
        println!("{p4:?}");
    }

    {
        assert_eq!(Container::new(42).value, 42);
        assert_eq!(Container::new(3.14).value, 3.14);
        assert_eq!(Container::new("Foo").value, "Foo");
        assert_eq!(
            Container::new(String::from("Bar")).value,
            String::from("Bar")
        );
        assert_eq!(Container::new(true).value, true);
        assert_eq!(Container::new(-12).value, -12);
        assert_eq!(Container::new(Some("text")).value, Some("text"));
    }
}
