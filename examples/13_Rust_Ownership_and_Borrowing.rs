// 🦀 Rust Ownership and Borrowing Exercises
// Type out each exercise to build muscle memory!

fn main() {
    println!("=== Exercise 1: Basic Ownership ===");
    // TODO: Create a String called 's' with value "Hello, Rust!"
    // let s = String::from("Hello, Rust!");
    let s = String::from("Hello, Rust!");

    // TODO: Print the string
    // println!("{}", s);
    println!("{}", s);

    println!("\n=== Exercise 2: Move Semantics ===");
    // TODO: Create a String 's1' with value "ownership"
    // let s1 = String::from("ownership");
    let s1 = String::from("ownership");

    // TODO: Move s1 to s2
    // let s2 = s1;
    let s2 = s1;

    // TODO: Try to print s2 (this works)
    // println!("s2 = {}", s2);
    println!("s2 = {}", s2);

    // TODO: Try to print s1 (uncomment to see error)
    // println!("s1 = {}", s1); // Error: value borrowed after move

    println!("\n=== Exercise 3: Clone Instead of Move ===");
    // TODO: Create a String 's1' with value "clone me"
    // let s1 = String::from("clone me");
    let s1 = String::from("clone me");

    // TODO: Clone s1 to s2
    // let s2 = s1.clone();
    let s2 = s1.clone();

    // TODO: Print both s1 and s2 (both work!)
    println!("s1 = {}, s2 = {}", s1, s2);

    println!("\n=== Exercise 4: Copy Trait (Stack Types) ===");
    // TODO: Create an i32 variable 'x' with value 5
    // let x = 5;
    let x = 5;

    // TODO: Assign x to y (copies automatically)
    // let y = x;
    let y = x;

    // TODO: Print both x and y (both work because i32 implements Copy)
    // println!("x = {}, y = {}", x, y);

    println!("x : {}, y : {}", x, y);

    println!("\n=== Exercise 5: Immutable Borrowing ===");
    // TODO: Create a String 's' with value "borrow me"
    // let s = String::from("borrow me");
    let s = String::from("borrow me");

    // TODO: Create an immutable reference 'r' to s
    // let r = &s;
    let r = &s;

    // TODO: Print using explicit dereference
    // println!("Explicit deref: {}", *r);
    println!("{}", *r);

    // TODO: Print using automatic dereference
    // println!("Auto deref: {}", r);
    println!("{}", r);

    // TODO: Original s is still valid!
    // println!("Original: {}", s);
    println!("Original : {}", s);

    println!("\n=== Exercise 6: Multiple Immutable References ===");
    // TODO: Create a String 's' with value "shared"
    // let s = String::from("shared");
    let s = String::from("valid");

    // TODO: Create three immutable references: r1, r2, r3
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &s;
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;

    // TODO: Print all three references (all work!)
    // println!("{}, {}, {}", r1, r2, r3);

    println!("{} {} {}", r1, r2, r3);
    println!("\n=== Exercise 7: Mutable Borrowing ===");
    // TODO: Create a MUTABLE String 's' with value "change me"
    // let mut s = String::from("change me");
    let mut s = String::from("change me");
    // TODO: Create a mutable reference 'r'
    // let r = &mut s;
    let r = &mut s;

    // TODO: Modify through the reference using push_str
    // r.push_str("!");
    r.push_str("!");

    // TODO: Print the modified string through reference
    // println!("Modified: {}", r);
    println!("Modified: {}", r);

    // TODO: Print original s (works after r is done being used)
    // println!("Original: {}", s);
    println!("Original: {}", s);

    println!("\n=== Exercise 8: One Mutable OR Many Immutable ===");
    // TODO: Create a mutable String 's' with value "rules"
    // let mut s = String::from("rules");
    let mut s = String::from("rules");

    // TODO: Create immutable reference r1
    // let r1 = &s;
    let r1 = &s;

    // TODO: Create another immutable reference r2
    // let r2 = &s;
    let r2 = &s;

    // TODO: Print both immutable references
    // println!("{} and {}", r1, r2);
    // Note: r1 and r2 are no longer used after this

    println!("{} {}", r1, r2);

    // TODO: Now create a mutable reference r3 (works because r1, r2 are done)
    // let r3 = &mut s;
    let r3 = &mut s;

    // TODO: Modify through r3
    // r3.push_str(" of borrowing");
    r3.push_str(" of borrowing");

    // TODO: Print r3
    // println!("{}", r3);
    println!("{}", r3);

    println!("\n=== Exercise 9: Function with Ownership Transfer ===");
    // TODO: Create a String 's' with value "take me"
    // let s = String::from("take me");
    let s = String::from("take me");

    // TODO: Call takes_ownership with s
    // takes_ownership(s);

    // TODO: Try to use s here (uncomment to see error)
    // println!("{}", s);  // Error: value used after move

    println!("\n=== Exercise 10: Function with Borrowing ===");
    // TODO: Create a String 's' with value "just look"
    // let s = String::from("just look");

    // TODO: Call borrows_string with a reference to s
    // borrows_string(&s);

    // TODO: s is still valid here!
    // println!("Still valid: {}", s);

    println!("\n=== Exercise 11: Function with Mutable Borrowing ===");
    // TODO: Create a mutable String 's' with value "modify"
    // let mut s = String::from("modify");

    // TODO: Call modifies_string with a mutable reference
    // modifies_string(&mut s);

    // TODO: Print modified s
    // println!("After modification: {}", s);

    println!("\n=== Exercise 12: Returning Ownership ===");
    // TODO: Call gives_ownership and store result in 's'
    // let s = gives_ownership();

    // TODO: Print the received string
    // println!("Received: {}", s);

    println!("\n=== Exercise 13: Taking and Returning Ownership ===");
    // TODO: Create a String 's1' with value "round trip"
    // let s1 = String::from("round trip");

    // TODO: Call takes_and_gives_back with s1, store in s2
    // let s2 = takes_and_gives_back(s1);

    // TODO: Print s2 (works)
    // println!("{}", s2);

    // TODO: Try to print s1 (uncomment to see error)
    // println!("{}", s1);  // Error: s1 was moved

    println!("\n✅ All exercises complete!");
}

// Helper function: Takes ownership
// TODO: Uncomment and type this function
// fn takes_ownership(some_string: String) {
//     println!("I now own: {}", some_string);
// } // some_string goes out of scope and is dropped

// Helper function: Borrows immutably
// TODO: Uncomment and type this function
// fn borrows_string(some_string: &String) {
//     println!("Just looking at: {}", some_string);
// }

// Helper function: Borrows mutably
// TODO: Uncomment and type this function
// fn modifies_string(some_string: &mut String) {
//     some_string.push_str(" - modified!");
// }

// Helper function: Gives ownership
// TODO: Uncomment and type this function
// fn gives_ownership() -> String {
//     let some_string = String::from("yours");
//     some_string  // Return value moves ownership to caller
// }

// Helper function: Takes and returns ownership
// TODO: Uncomment and type this function
// fn takes_and_gives_back(a_string: String) -> String {
//     a_string  // Return value moves ownership back
// }
