fn test_match(point: (i32, i32)) {
    match point {
        (0, 0) => println!("Origin"),
        (0, y) => println!("On Y-axis at {}", y),
        (x, 0) => println!("On X-axis at {}", x),
        (x, y) => println!("Point at ({}, {})", x, y),
    }
}

fn main() {
    let some_tuples = (1, 2, 3, 4);

    // {:?} is a debug formatter in Rust
    println!("{:?}", some_tuples);

    // Pretty Debug: {:#?}
    println!("{:#?}", some_tuples);

    // print a single value
    println!("{}", some_tuples.3);

    let nested = ((1, 2), (3, 4), (5, 6));

    // Access nested elements
    let first_tuple = nested.0; // (1, 2)
    let first_element = nested.0.0; // 1

    println!("First tuple: {:?}", first_tuple);
    println!("First element: {}", first_element);

    test_match((0, some_tuples.2));
}
