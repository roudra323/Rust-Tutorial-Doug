# 🦀 Rust Structs

**Topic:** Creating and using structs — custom data types with named fields  
**Skill Level:** Beginner to Intermediate

---

## Overview

**Structs** (short for "structures") are custom data types that let you package together and name multiple related values. They are:

- **Named fields** — each piece of data has a descriptive name
- **Custom types** — you define your own types
- **Flexible** — can contain any combination of types
- **Foundation for OOP-like patterns** — combined with `impl` blocks for methods

Structs are one of the most important features in Rust for organizing code and data.

---

## Three Types of Structs

### 1. Classic Structs (Named Fields)

```rust
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}
```

Most common type — fields have names.

### 2. Tuple Structs (Unnamed Fields)

```rust
struct Color(u8, u8, u8);
struct Point(i32, i32);
```

Like tuples but with a custom type name.

### 3. Unit-Like Structs (No Fields)

```rust
struct Marker;
```

No data, just a type marker. Useful for traits and generics.

---

## Classic Structs (Named Fields)

### Defining a Struct

```rust
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}
```

**Convention:** Struct names use **PascalCase** (UpperCamelCase).

### Creating an Instance

```rust
fn main() {
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        age: 30,
        active: true,
    };
}
```

**Key Points:**
- Use curly braces `{ }` to instantiate
- Specify all fields (no defaults)
- Order doesn't matter
- Trailing comma is optional but recommended

### Accessing Fields

```rust
fn main() {
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        age: 30,
        active: true,
    };
    
    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Age: {}", user1.age);
}
```

Use **dot notation** to access fields.

### Modifying Fields (Mutable Structs)

```rust
fn main() {
    let mut user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        age: 30,
        active: true,
    };
    
    user1.email = String::from("newalice@example.com");
    user1.age = 31;
    
    println!("New email: {}", user1.email);
}
```

**Important:** The **entire struct** must be mutable — you can't make individual fields mutable.

---

## Field Init Shorthand

When variable names match field names, use shorthand syntax:

### Without Shorthand

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        age: 0,
        active: true,
    }
}
```

### With Shorthand

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,      // Shorthand: same as email: email
        username,   // Shorthand: same as username: username
        age: 0,
        active: true,
    }
}
```

**Much cleaner!** This is the idiomatic Rust way.

---

## Struct Update Syntax

Create a new struct instance using most of an existing instance's values:

```rust
fn main() {
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        age: 30,
        active: true,
    };
    
    let user2 = User {
        email: String::from("bob@example.com"),
        username: String::from("bob"),
        ..user1  // Copy remaining fields from user1
    };
    
    println!("User2 email: {}", user2.email);
    println!("User2 age: {}", user2.age);    // Copied from user1
    println!("User2 active: {}", user2.active); // Copied from user1
}
```

**Important:** `..user1` must come **last** and copies remaining fields.

### ⚠️ Ownership Moves with Update Syntax

```rust
fn main() {
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        age: 30,
        active: true,
    };
    
    let user2 = User {
        email: String::from("bob@example.com"),
        ..user1  // Moves username from user1
    };
    
    // ❌ Can't use user1.username anymore (moved to user2)
    // println!("{}", user1.username);  // Compile error
    
    // ✅ Can still use user1.age and user1.active (Copy types)
    println!("{}", user1.age);
}
```

**Why?** `String` doesn't implement `Copy`, so it moves. Primitive types like `u32` and `bool` implement `Copy`, so they're copied.

---

## Tuple Structs

Structs that look like tuples but have a custom type name.

### Defining Tuple Structs

```rust
struct Color(u8, u8, u8);
struct Point(i32, i32, i32);
```

### Creating and Using Tuple Structs

```rust
fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    // Access like tuples with dot notation
    println!("Red: {}", black.0);
    println!("X coordinate: {}", origin.0);
}
```

### Destructuring Tuple Structs

```rust
fn main() {
    let color = Color(255, 128, 0);
    
    let Color(red, green, blue) = color;
    println!("RGB: ({}, {}, {})", red, green, blue);
}
```

### Why Use Tuple Structs?

```rust
struct Color(u8, u8, u8);
struct Point(i32, i32, i32);

fn process_color(c: Color) {
    // ...
}

fn main() {
    let color = Color(255, 0, 0);
    let point = Point(10, 20, 30);
    
    process_color(color);         // ✅ Works
    // process_color(point);      // ❌ Compile error: wrong type!
}
```

**Type Safety:** Even though both have three numbers, they're **different types**.

---

## Unit-Like Structs

Structs with no fields — just a type marker.

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

### Use Cases

- **Marker traits** — indicate a type has certain properties
- **Phantom types** — type-level programming
- **State machines** — zero-sized type markers

```rust
struct NoData;

impl SomeTrait for NoData {
    // Implementation
}
```

---

## Printing Structs

### ❌ Default Display Doesn't Work

```rust
struct User {
    username: String,
    age: u32,
}

fn main() {
    let user = User {
        username: String::from("alice"),
        age: 30,
    };
    
    println!("{}", user);  // ❌ Compile error: User doesn't implement Display
}
```

### ✅ Solution 1: Debug Trait

Add `#[derive(Debug)]` to enable debug printing:

```rust
#[derive(Debug)]
struct User {
    username: String,
    age: u32,
}

fn main() {
    let user = User {
        username: String::from("alice"),
        age: 30,
    };
    
    println!("{:?}", user);  // Debug format
    // Output: User { username: "alice", age: 30 }
    
    println!("{:#?}", user); // Pretty-print debug format
    // Output:
    // User {
    //     username: "alice",
    //     age: 30,
    // }
}
```

### ✅ Solution 2: Implement Display Trait

```rust
use std::fmt;

struct User {
    username: String,
    age: u32,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (age: {})", self.username, self.age)
    }
}

fn main() {
    let user = User {
        username: String::from("alice"),
        age: 30,
    };
    
    println!("{}", user);  // Output: alice (age: 30)
}
```

---

## Methods on Structs (`impl` Blocks)

Add functionality to structs using `impl` (implementation) blocks.

### Defining Methods

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method (takes &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Method that modifies the struct
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
    
    // Method that takes ownership
    fn consume(self) -> u32 {
        self.width + self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("Area: {}", rect.area());
}
```

**Method Receiver Types:**
- `&self` — borrow (most common)
- `&mut self` — mutable borrow
- `self` — take ownership (consumes the value)

### Associated Functions (No `self`)

Functions defined in `impl` blocks that don't take `self` — like "static methods" in other languages.

```rust
impl Rectangle {
    // Associated function (constructor)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    // Associated function (named constructor)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle::new(30, 50);
    let sq = Rectangle::square(25);
    
    println!("Rectangle: {:?}", rect);
    println!("Square: {:?}", sq);
}
```

**Call with `::`** notation: `Rectangle::new(30, 50)`

### Multiple `impl` Blocks

You can have multiple `impl` blocks for the same struct:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}
```

**This is valid but not common** — usually all methods are in one `impl` block.

---

## Method Syntax vs. Function Syntax

### Method Syntax (With Receiver)

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle::new(30, 50);
    let area = rect.area();  // Method call
}
```

### Function Syntax (No Receiver)

```rust
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn main() {
    let rect = Rectangle::new(30, 50);
    let area = area(&rect);  // Function call
}
```

**Methods are more readable and idiomatic in Rust.**

---

## Common Derived Traits

### `#[derive(Debug)]`

Enables debug printing with `{:?}`:

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
```

### `#[derive(Clone)]`

Allows explicit cloning:

```rust
#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1.clone();  // Explicit clone
}
```

### `#[derive(Copy)]`

Makes the struct copy on assignment (requires all fields to be `Copy`):

```rust
#[derive(Copy, Clone)]  // Copy requires Clone
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1;  // Copy (not move)
    println!("{}, {}", p1.x, p2.x);  // Both still valid
}
```

### `#[derive(PartialEq)]`

Enables equality comparison with `==`:

```rust
#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 10, y: 20 };
    
    if p1 == p2 {
        println!("Equal!");
    }
}
```

### Multiple Traits

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
```

---

## Struct Ownership and Lifetimes

### Owned Data (Recommended for Beginners)

```rust
struct User {
    username: String,  // Owned String
    email: String,     // Owned String
    age: u32,
}
```

The struct owns its data — simple and safe.

### References in Structs (Requires Lifetimes)

```rust
struct User<'a> {
    username: &'a str,  // Borrowed data
    email: &'a str,     // Borrowed data
    age: u32,
}

fn main() {
    let name = String::from("alice");
    let email = String::from("alice@example.com");
    
    let user = User {
        username: &name,
        email: &email,
        age: 30,
    };
}
```

**Lifetimes** ensure the referenced data lives long enough. This is an advanced topic.

---

## Privacy and Modules

### Public Structs

```rust
pub struct User {
    pub username: String,  // Public field
    email: String,         // Private field (default)
    pub age: u32,
}
```

- `pub struct` makes the struct visible outside the module
- Fields are **private by default**
- Mark fields `pub` to make them accessible

### Accessing Private Fields

```rust
pub struct User {
    username: String,  // Private
    pub age: u32,      // Public
}

impl User {
    // Constructor for private fields
    pub fn new(username: String, age: u32) -> User {
        User { username, age }
    }
    
    // Getter for private field
    pub fn username(&self) -> &str {
        &self.username
    }
}
```

**Pattern:** Use constructors and getters for private fields.

---

## Common Patterns

### Builder Pattern

```rust
struct User {
    username: String,
    email: String,
    age: u32,
}

impl User {
    fn new() -> UserBuilder {
        UserBuilder::default()
    }
}

struct UserBuilder {
    username: Option<String>,
    email: Option<String>,
    age: Option<u32>,
}

impl UserBuilder {
    fn default() -> Self {
        UserBuilder {
            username: None,
            email: None,
            age: None,
        }
    }
    
    fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }
    
    fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
    
    fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }
    
    fn build(self) -> User {
        User {
            username: self.username.unwrap(),
            email: self.email.unwrap(),
            age: self.age.unwrap(),
        }
    }
}

fn main() {
    let user = User::new()
        .username(String::from("alice"))
        .email(String::from("alice@example.com"))
        .age(30)
        .build();
}
```

### Newtype Pattern

Wrap a single value in a struct for type safety:

```rust
struct UserId(u32);
struct ProductId(u32);

fn get_user(id: UserId) {
    // ...
}

fn main() {
    let user_id = UserId(42);
    let product_id = ProductId(99);
    
    get_user(user_id);            // ✅ Works
    // get_user(product_id);      // ❌ Compile error: wrong type!
}
```

### State Pattern

```rust
struct Draft;
struct Published;

struct BlogPost<State> {
    content: String,
    state: std::marker::PhantomData<State>,
}

impl BlogPost<Draft> {
    fn new(content: String) -> Self {
        BlogPost {
            content,
            state: std::marker::PhantomData,
        }
    }
    
    fn publish(self) -> BlogPost<Published> {
        BlogPost {
            content: self.content,
            state: std::marker::PhantomData,
        }
    }
}

impl BlogPost<Published> {
    fn content(&self) -> &str {
        &self.content
    }
}
```

---

## Common Mistakes

### Mistake 1: Forgetting to Make Struct Mutable

```rust
let user = User { /* ... */ };
user.age = 31;  // ❌ Compile error: user is immutable
```

**Fix:**
```rust
let mut user = User { /* ... */ };
user.age = 31;  // ✅ Works
```

### Mistake 2: Partial Mutability

```rust
struct User {
    username: String,
    age: u32,
}

let mut user = User { /* ... */ };
// ❌ Can't make only username mutable
```

**Rust doesn't support per-field mutability** — the entire struct is mutable or not.

### Mistake 3: Moving Out of Struct

```rust
struct User {
    username: String,
}

fn main() {
    let user = User {
        username: String::from("alice"),
    };
    
    let name = user.username;  // Moves username out
    // println!("{}", user.username);  // ❌ Can't use after move
}
```

**Fix:** Use references or `.clone()`:
```rust
let name = &user.username;   // Borrow
// or
let name = user.username.clone();  // Clone
```

### Mistake 4: Missing Fields During Construction

```rust
let user = User {
    username: String::from("alice"),
    // ❌ Missing email and age fields
};
```

**Rust requires all fields to be specified** (no defaults unless you implement them).

---

## Structs vs. Tuples

| Feature | Structs | Tuples |
|---|---|---|
| **Field Names** | Named fields | Positional access |
| **Readability** | Very clear | Can be confusing |
| **Type Safety** | Strong (custom types) | Weaker |
| **Methods** | Can add methods | No methods |
| **Use Case** | Complex data structures | Simple, temporary groupings |

### When to Use Structs

- Data has more than 2-3 fields
- Field names improve clarity
- You need methods on the data
- The structure is part of your API

### When to Use Tuples

- Temporary groupings (2-3 values)
- Relationship is obvious from context
- Returning multiple values from functions

---

## Performance Considerations

### Stack vs. Heap

```rust
struct Point {
    x: i32,
    y: i32,
}  // Stack-allocated (small, fixed size)

struct User {
    username: String,  // String itself is on heap
    age: u32,
}  // Struct is on stack, but String points to heap
```

### Zero-Cost Abstraction

```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}
```

**Methods have no runtime overhead** — they compile to the same code as direct field access.

### Memory Layout

Structs have predictable memory layout (can be optimized by compiler):

```rust
#[repr(C)]  // Use C-compatible layout
struct Point {
    x: i32,
    y: i32,
}
```

---

## Summary Table

| Feature | Syntax | Notes |
|---|---|---|
| **Classic Struct** | `struct Name { field: Type }` | Named fields |
| **Tuple Struct** | `struct Name(Type, Type)` | Positional fields |
| **Unit Struct** | `struct Name;` | No fields |
| **Instance** | `Name { field: value }` | All fields required |
| **Access** | `instance.field` | Dot notation |
| **Methods** | `impl Name { fn method(&self) }` | In `impl` block |
| **Associated Fn** | `impl Name { fn new() }` | No `self` parameter |
| **Mutability** | `let mut instance` | Entire struct is mut |
| **Printing** | `#[derive(Debug)]` | Use `{:?}` |

---

## Key Takeaways

1. **Structs group related data** — foundation for organizing code

2. **Named fields improve clarity** — better than positional access

3. **Three types exist** — classic, tuple, and unit-like structs

4. **Methods add behavior** — use `impl` blocks

5. **Field init shorthand** — `User { email, username, ... }`

6. **Struct update syntax** — `..other_instance` copies fields

7. **Entire struct is mutable** — not per-field

8. **Derive traits for functionality** — `Debug`, `Clone`, `Copy`, `PartialEq`

9. **Ownership matters** — `String` moves, primitives copy

10. **Zero-cost abstraction** — methods have no runtime overhead

---

## Best Practices

✅ **Do:**
- Use PascalCase for struct names
- Use field init shorthand when possible
- Derive common traits (`Debug`, `Clone`, etc.)
- Make entire struct `mut`, not individual fields
- Use constructors (associated functions) for initialization
- Keep structs focused and single-purpose

❌ **Don't:**
- Create huge structs with dozens of fields (split them up)
- Make all fields public unnecessarily
- Forget to derive `Debug` for structs you'll print
- Mix owned and borrowed data without understanding lifetimes
- Use tuples when a struct would be clearer

---

## Next Steps

- Learn about **enums** (algebraic data types)
- Study **traits** (interfaces) and trait bounds
- Explore **lifetimes** for borrowed data in structs
- Understand **generics** for reusable struct types
- Learn **pattern matching** with structs

---

**Confidence Level:** High ✅  
**Recommended Practice:** Create several structs with methods, practice deriving traits, and build small programs using struct-based designs.
