// Define a struct named User to store user information
// Structs are custom data types that group related data together
struct User {
    username: String, // String type - owned, heap-allocated string
    email: String,    // String type - owned, heap-allocated string
    age: i32,         // i32 - 32-bit signed integer
    active: bool,     // bool - boolean type (true/false)
}

fn main() {
    // Create an instance of the User struct
    // Each field must be initialized with a value of the correct type
    let user1: User = User {
        username: String::from("Alice"), // Create a String from a string literal
        email: String::from("alice@gmail.com"), // Create a String from a string literal
        age: 30,                         // Integer literal
        active: true,                    // Boolean literal
    };

    // Access struct fields using dot notation
    // Print each field value to the console
    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Age: {}", user1.age);
    println!("Active {}", user1.active);
}
