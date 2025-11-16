fn main() {
    // Experiment 1: See ownership in action
    let a = String::from("Hello");
    let b = String::from(" World");
    let combined = a + &b;

    // println!("{}", a); // Uncomment this - see the error!
    println!("{}", b); // This works
    println!("{}", combined); // This works

    // Experiment 2: Multiple concatenations
    let s1 = String::from("a");
    let s2 = String::from("b");
    let s3 = String::from("c");

    let result = s1 + &s2 + &s3; // Chain them!
    println!("{}", result); // "abc"

    // Experiment 3: format! macro (better for multiple strings)
    let x = String::from("x");
    let y = String::from("y");
    let z = String::from("z");

    let formatted = format!("{}{}{}", x, y, z);
    println!("{}", formatted); // "xyz"
    // Bonus: x, y, z are still usable! format! borrows, doesn't move
    println!("{}", x); // Still works!

    // Experiment 4: Playing with slices
    let text = "Rust is awesome!";
    println!("{}", &text[0..4]); // "Rust"
    println!("{}", &text[5..7]); // "is"
    println!("{}", &text[8..]); // "awesome!" (from 8 to end)
    println!("{}", &text[..4]); // "Rust" (from start to 4)
    println!("{}", &text[..]); // "Rust is awesome!" (whole thing)

    let _subtext_text: &Option<char> = &text.chars().nth(100);

    // Approach-1
    // if let Some(c) = subtext_text {
    //     println!("{}", c);
    // } else {
    //     println!("Index out of bound");
    // }

    // Approach-2
    // match subtext_text {
    //     Some(c) => println!("Found: {}", c),
    //     None => println!("Not Found"),
    // }

    // Approach-3
    // let text2 = text.chars().nth(100).unwrap_or('?'); // Custom default char
    // Or for a string slice default:
    let text2 = text.get(100..101).unwrap_or("10");
    println!("{}", text2)
}
