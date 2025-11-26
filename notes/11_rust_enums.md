# 🦀 Rust Enums (Enumerations)

**Topic:** Algebraic data types and pattern matching with enums  
**Skill Level:** Beginner to Advanced

---

## Overview

**Enums** (enumerations) define a type by listing its possible variants. Rust enums are extremely powerful:

- **Algebraic data types** — can hold data of different types and amounts
- **Type safety** — exhaustive checking at compile time
- **Pattern matching** — elegant way to handle all cases
- **Zero-cost abstraction** — compiled to efficient machine code

Enums are one of Rust's most distinctive features and enable robust, expressive code.

---

## Basic Enums

### Simple Enum (No Data)

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let dir = Direction::North;
}
```

**Convention:** Enum names and variants use **PascalCase**.

### Using Enum Values

```rust
fn move_player(direction: Direction) {
    match direction {
        Direction::North => println!("Moving north"),
        Direction::South => println!("Moving south"),
        Direction::East => println!("Moving east"),
        Direction::West => println!("Moving west"),
    }
}

fn main() {
    move_player(Direction::North);
    move_player(Direction::East);
}
```

---

## Enums with Data

### Variants with Different Data Types

```rust
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },   // Struct-like
    Write(String),              // Tuple-like with one field
    ChangeColor(u8, u8, u8),   // Tuple-like with three fields
}

fn main() {
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello"));
    let msg4 = Message::ChangeColor(255, 0, 0);
}
```

**Each variant can hold different types and amounts of data!**

### Accessing Enum Data

Use pattern matching to extract data:

```rust
fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Quit message");
        }
        Message::Move { x, y } => {
            println!("Move to ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("Write: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to RGB({}, {}, {})", r, g, b);
        }
    }
}
```

---

## The `match` Expression

### Exhaustive Matching

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

**Compiler enforces exhaustiveness** — you must handle all variants.

### Match with Data Extraction

```rust
enum UsState {
    Alabama,
    Alaska,
    California,
    // ... other states
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

### Match Guards

```rust
fn check_number(num: Option<i32>) {
    match num {
        Some(x) if x < 0 => println!("Negative: {}", x),
        Some(x) if x == 0 => println!("Zero"),
        Some(x) if x > 0 => println!("Positive: {}", x),
        None => println!("No value"),
        _ => println!("Shouldn't reach here"),
    }
}
```

### Catch-All Pattern with `_`

```rust
fn describe_value(value: u8) {
    match value {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),  // Catch all others
    }
}
```

---

## The `Option<T>` Enum

### Definition

```rust
enum Option<T> {
    Some(T),
    None,
}
```

**Rust doesn't have null** — uses `Option` instead!

### Basic Usage

```rust
fn main() {
    let some_number = Some(5);
    let some_string = Some("hello");
    let absent_number: Option<i32> = None;
}
```

### Pattern Matching with Option

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

### Unwrapping Options (Unsafe)

```rust
fn main() {
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;
    
    println!("{}", x.unwrap());  // ✅ Returns 5
    // println!("{}", y.unwrap());  // ❌ Panics at runtime!
}
```

**Avoid `unwrap()`** — use pattern matching or safer methods.

### Safe Option Handling

```rust
fn main() {
    let x: Option<i32> = Some(5);
    
    // if let
    if let Some(value) = x {
        println!("Value: {}", value);
    }
    
    // unwrap_or
    let value = x.unwrap_or(0);
    
    // unwrap_or_else
    let value = x.unwrap_or_else(|| 0);
    
    // map
    let doubled = x.map(|v| v * 2);
    
    // and_then
    let result = x.and_then(|v| Some(v * 2));
}
```

---

## The `Result<T, E>` Enum

### Definition

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

**Used for error handling** — functions that can fail return `Result`.

### Basic Usage

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    
    match f {
        Ok(file) => println!("File opened successfully"),
        Err(error) => println!("Error opening file: {:?}", error),
    }
}
```

### Propagating Errors with `?`

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;  // ? propagates error
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

**`?` operator** — returns error early if `Result` is `Err`, or unwraps if `Ok`.

### Shorthand Error Handling

```rust
fn main() {
    let f = File::open("hello.txt").unwrap();  // Panics if error
    
    let f = File::open("hello.txt")
        .expect("Failed to open file");  // Panics with custom message
}
```

---

## `if let` (Concise Pattern Matching)

### Basic `if let`

```rust
fn main() {
    let some_value = Some(3);
    
    // Using match
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }
    
    // Using if let (cleaner)
    if let Some(3) = some_value {
        println!("three");
    }
}
```

### `if let` with `else`

```rust
fn main() {
    let config_max = Some(3u8);
    
    if let Some(max) = config_max {
        println!("The maximum is {}", max);
    } else {
        println!("No maximum configured");
    }
}
```

### When to Use `if let`

- You only care about **one pattern**
- Don't need exhaustive checking
- Cleaner than a `match` with mostly `_ => ()`

---

## Methods on Enums

### Implementing Methods

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quitting"),
            Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
            Message::Write(text) => println!("Writing: {}", text),
        }
    }
    
    fn is_quit(&self) -> bool {
        matches!(self, Message::Quit)
    }
}

fn main() {
    let msg = Message::Write(String::from("Hello"));
    msg.call();
}
```

### Associated Functions

```rust
impl Message {
    fn new_move(x: i32, y: i32) -> Self {
        Message::Move { x, y }
    }
    
    fn new_write(text: &str) -> Self {
        Message::Write(String::from(text))
    }
}

fn main() {
    let msg = Message::new_move(10, 20);
}
```

---

## Deriving Traits for Enums

### Common Derived Traits

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let dir1 = Direction::North;
    let dir2 = Direction::North;
    
    println!("{:?}", dir1);  // Debug
    
    if dir1 == dir2 {  // PartialEq
        println!("Same direction");
    }
    
    let dir3 = dir1;  // Copy
}
```

### Debug Trait

```rust
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };
    println!("{:?}", msg);  // Message::Move { x: 10, y: 20 }
}
```

---

## C-Style Enums (Discriminants)

### Basic Discriminants

```rust
enum Number {
    Zero = 0,
    One = 1,
    Two = 2,
}

fn main() {
    let zero = Number::Zero as i32;
    println!("Zero: {}", zero);  // 0
}
```

### Auto-Incrementing

```rust
enum Color {
    Red = 1,
    Green,    // 2
    Blue,     // 3
}
```

### Use Cases

```rust
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500,
}

fn get_status_code(status: HttpStatus) -> u16 {
    status as u16
}
```

---

## Generic Enums

### Simple Generic Enum

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Option<T> {
    Some(T),
    None,
}
```

### Custom Generic Enum

```rust
enum Either<L, R> {
    Left(L),
    Right(R),
}

fn main() {
    let left: Either<i32, String> = Either::Left(42);
    let right: Either<i32, String> = Either::Right(String::from("hello"));
}
```

---

## Recursive Enums

### List Type (Cons List)

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

**`Box`** is needed for recursive types — provides indirection.

### Binary Tree

```rust
enum BinaryTree {
    Node {
        value: i32,
        left: Box<BinaryTree>,
        right: Box<BinaryTree>,
    },
    Empty,
}
```

---

## Pattern Matching Patterns

### Destructuring Enums

```rust
enum Message {
    Move { x: i32, y: i32 },
}

fn process(msg: Message) {
    match msg {
        Message::Move { x, y } => {
            println!("x: {}, y: {}", x, y);
        }
    }
}
```

### Ignoring Values

```rust
enum Message {
    Move { x: i32, y: i32 },
    Write(String),
}

fn process(msg: Message) {
    match msg {
        Message::Move { x, .. } => {  // Ignore y
            println!("x: {}", x);
        }
        Message::Write(_) => {  // Ignore string content
            println!("Write message");
        }
    }
}
```

### Multiple Patterns

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn is_large_coin(coin: Coin) -> bool {
    match coin {
        Coin::Dime | Coin::Quarter => true,
        Coin::Penny | Coin::Nickel => false,
    }
}
```

### Range Patterns

```rust
enum Value {
    Number(i32),
}

fn classify(val: Value) {
    match val {
        Value::Number(1..=10) => println!("Small"),
        Value::Number(11..=100) => println!("Medium"),
        Value::Number(_) => println!("Large"),
    }
}
```

---

## Common Patterns

### State Machine

```rust
enum State {
    Waiting,
    Processing,
    Complete,
    Error(String),
}

struct Task {
    state: State,
}

impl Task {
    fn new() -> Self {
        Task { state: State::Waiting }
    }
    
    fn process(&mut self) {
        self.state = match self.state {
            State::Waiting => State::Processing,
            State::Processing => State::Complete,
            State::Complete => State::Complete,
            State::Error(_) => State::Error(String::from("Still errored")),
        };
    }
}
```

### Command Pattern

```rust
enum Command {
    Save,
    Load,
    Quit,
}

fn execute(command: Command) {
    match command {
        Command::Save => println!("Saving..."),
        Command::Load => println!("Loading..."),
        Command::Quit => println!("Quitting..."),
    }
}
```

### Result Type Pattern

```rust
enum ParseError {
    InvalidFormat,
    TooLarge,
    TooSmall,
}

fn parse_number(s: &str) -> Result<i32, ParseError> {
    match s.parse::<i32>() {
        Ok(n) if n > 100 => Err(ParseError::TooLarge),
        Ok(n) if n < 0 => Err(ParseError::TooSmall),
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidFormat),
    }
}
```

---

## `matches!` Macro

### Quick Boolean Checks

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
}

fn main() {
    let msg = Message::Quit;
    
    // Using match
    let is_quit = match msg {
        Message::Quit => true,
        _ => false,
    };
    
    // Using matches! macro (cleaner)
    let is_quit = matches!(msg, Message::Quit);
    
    println!("Is quit: {}", is_quit);
}
```

### With Patterns

```rust
fn main() {
    let value = Some(5);
    
    if matches!(value, Some(x) if x > 3) {
        println!("Value is greater than 3");
    }
}
```

---

## Memory Layout and Size

### Size of Enums

```rust
use std::mem::size_of;

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn main() {
    println!("Size of Message: {}", size_of::<Message>());
    // Size is the largest variant + discriminant tag
}
```

### Optimization with `#[repr]`

```rust
#[repr(C)]
enum Color {
    Red,
    Green,
    Blue,
}
```

**`#[repr(C)]`** — use C-compatible representation (for FFI).

---

## Common Mistakes

### Mistake 1: Not Handling All Variants

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        // ❌ Compile error: missing Coin::Dime
    }
}
```

**Fix:** Handle all variants or use `_` for catch-all.

### Mistake 2: Moving Out of Enum

```rust
enum Message {
    Write(String),
}

fn main() {
    let msg = Message::Write(String::from("hello"));
    
    match msg {
        Message::Write(text) => {
            println!("{}", text);
        }
    }
    
    // ❌ Can't use msg again (moved in match)
    // println!("{:?}", msg);
}
```

**Fix:** Use references in match:
```rust
match &msg {
    Message::Write(text) => {
        println!("{}", text);
    }
}
```

### Mistake 3: Comparing Enums Without PartialEq

```rust
enum Direction {
    North,
    South,
}

fn main() {
    let dir1 = Direction::North;
    let dir2 = Direction::North;
    
    // ❌ Compile error without #[derive(PartialEq)]
    // if dir1 == dir2 { }
}
```

**Fix:**
```rust
#[derive(PartialEq)]
enum Direction {
    North,
    South,
}
```

---

## Advanced Features

### Enum Variants as Types (Nightly)

```rust
// Nightly feature
#![feature(arbitrary_enum_discriminant)]

enum Color {
    Red = 0xFF0000,
    Green = 0x00FF00,
    Blue = 0x0000FF,
}
```

### Associated Constants

```rust
enum Direction {
    North,
    South,
}

impl Direction {
    const COUNT: usize = 2;
}
```

---

## Enums vs. Structs

| Feature | Enums | Structs |
|---|---|---|
| **Purpose** | One of several variants | Bundle of named fields |
| **Data** | Different per variant | Same fields always |
| **Pattern Matching** | Essential | Optional |
| **Exhaustiveness** | Compiler enforced | N/A |
| **Use Case** | States, alternatives | Data structures |

### When to Use Enums

- Representing a **choice** between alternatives
- State machines
- Error types
- Command/message types
- Optional values (`Option`)
- Result types (`Result`)

### When to Use Structs

- Representing a **single** data structure
- Always have all fields
- No alternatives or variants

---

## Performance Considerations

### Size Optimization

```rust
// Larger enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),  // String is 24 bytes
}

// Smaller with Box
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(Box<String>),  // Box is 8 bytes (pointer)
}
```

### Pattern Matching is Zero-Cost

```rust
// Compiles to efficient jump table or if-else chain
match value {
    Variant1 => { /* ... */ }
    Variant2 => { /* ... */ }
    Variant3 => { /* ... */ }
}
```

---

## Summary Table

| Feature | Syntax | Use Case |
|---|---|---|
| **Simple Enum** | `enum Name { A, B }` | Fixed set of values |
| **Enum with Data** | `enum Name { A(i32) }` | Variants hold data |
| **Match** | `match value { A => {} }` | Pattern matching |
| **if let** | `if let A = value {}` | Single pattern |
| **Option** | `Option<T>` | Optional values |
| **Result** | `Result<T, E>` | Error handling |
| **Methods** | `impl Name { fn() }` | Add behavior |
| **Discriminant** | `Name::A as i32` | Get numeric value |
| **Generic** | `enum Name<T> { A(T) }` | Type parameters |

---

## Key Takeaways

1. **Enums represent alternatives** — one of several possible variants

2. **Algebraic data types** — variants can hold different types and amounts of data

3. **Pattern matching is essential** — use `match` to handle all variants

4. **Compiler enforces exhaustiveness** — must handle all cases

5. **`Option` replaces null** — safe handling of optional values

6. **`Result` for errors** — explicit error handling

7. **`if let` for convenience** — when you only care about one pattern

8. **Methods on enums** — add behavior with `impl` blocks

9. **Zero-cost abstraction** — compiled to efficient code

10. **Deriving traits** — automatically implement common functionality

---

## Best Practices

✅ **Do:**
- Use `match` for exhaustive handling
- Derive `Debug` for all enums
- Use `Option` instead of null values
- Use `Result` for fallible operations
- Use `if let` when matching one pattern
- Add methods to enums when it makes sense
- Use meaningful variant names

❌ **Don't:**
- Use `unwrap()` without good reason
- Ignore `Result` errors (use `?` or match)
- Create enums with too many variants (split them up)
- Forget to handle all variants in `match`
- Use `_` catch-all when you should be explicit

---

## Next Steps

- Study **pattern matching** in depth
- Learn **error handling** with `Result` and `?`
- Explore **Option combinators** (`map`, `and_then`, etc.)
- Understand **trait objects** vs. enums for polymorphism
- Learn **advanced pattern matching** (guards, ranges, etc.)

---

**Confidence Level:** High ✅  
**Recommended Practice:** Create several enums, use pattern matching extensively, work with `Option` and `Result`, and implement methods on enums.
