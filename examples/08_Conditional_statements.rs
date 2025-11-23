fn main() {
    let a = 5;
    let b = 10;

    if a == b {
        println!("Equal");
    } else if a < b {
        println!("a is less than b");
    } else {
        println!("a is greater than b");
    }

    let result: &str = if a == b {
        "Equal"
    } else if a < b {
        "a less than b"
    } else {
        "a greater than b"
    };

    println!("{}", result);

    let result2 = match a {
        1 => "One",
        2 => "Two",
        3 => "Three",
        5 => "Five",
        _ => "Others",
    };

    println!("{}", result2);
}
