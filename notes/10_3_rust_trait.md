# 🦀 Rust Traits

**Topic:** Defining shared behavior across types using traits  
**Skill Level:** Beginner to Advanced

---

## Overview

**Traits** define shared behavior in an abstract way. They are similar to interfaces in other languages. Traits allow you to:

- **Define shared behavior** — specify methods that types must implement
- **Enable polymorphism** — write code that works with multiple types
- **Provide default implementations** — optional method implementations
- **Set trait bounds** — constrain generic types
- **Enable operator overloading** — implement operators like `+`, `==`, etc.

Traits are fundamental to Rust's type system and zero-cost abstractions.

---

## Defining a Trait

### Basic Syntax

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

**Components:**
- `trait` keyword
- Trait name (PascalCase convention)
- Method signatures (no implementation)

### Trait with Multiple Methods

```rust
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn name(&self) -> &str;
}
```

---

## Implementing a Trait

### Basic Implementation

```rust
struct Article {
    headline: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.content)
    }
}

fn main() {
    let article = Article {
        headline: String::from("Rust Traits"),
        content: String::from("Traits are awesome!"),
    };
    
    println!("{}", article.summarize());
}
```

**Syntax:** `impl TraitName for TypeName`

### Implementing for Multiple Types

```rust
struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Announcing Rust 1.70!"),
    };
    
    println!("{}", tweet.summarize());
}
```

---

## Default Implementations

Traits can provide default method implementations:

```rust
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")  // Default implementation
    }
}

struct NewsArticle {
    headline: String,
}

// Use default implementation
impl Summary for NewsArticle {}

fn main() {
    let article = NewsArticle {
        headline: String::from("Breaking News"),
    };
    
    println!("{}", article.summarize());  // Output: (Read more...)
}
```

### Overriding Default Implementations

```rust
struct BlogPost {
    title: String,
    content: String,
}

impl Summary for BlogPost {
    fn summarize(&self) -> String {
        format!("{} - {}", self.title, self.content)  // Override default
    }
}
```

### Default Methods Calling Other Methods

```rust
trait Summary {
    fn summarize_author(&self) -> String;
    
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // summarize() uses default implementation
}
```

---

## Traits as Parameters

### Simple Trait Bound

```rust
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let article = Article { /* ... */ };
    let tweet = Tweet { /* ... */ };
    
    notify(&article);  // ✅ Works
    notify(&tweet);    // ✅ Works
}
```

**`impl Trait` syntax** — accepts any type that implements `Summary`.

### Trait Bound Syntax (More Verbose)

```rust
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

**Equivalent to `impl Trait`** but more explicit.

### Multiple Trait Bounds

```rust
use std::fmt::Display;

fn notify(item: &(impl Summary + Display)) {
    println!("{}", item);
    println!("{}", item.summarize());
}

// Or with trait bound syntax:
fn notify<T: Summary + Display>(item: &T) {
    println!("{}", item);
    println!("{}", item.summarize());
}
```

Use `+` to require multiple traits.

### `where` Clauses for Complex Bounds

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // Function body
}
```

**Cleaner syntax** for complex trait bounds.

---

## Returning Types That Implement Traits

### Basic Return

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("rustlang"),
        content: String::from("New Rust release!"),
    }
}
```

**Limitation:** Can only return **one concrete type** (not different types in different branches).

### ❌ Can't Return Different Types

```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle { /* ... */ }  // ❌ Compile error
    } else {
        Tweet { /* ... */ }        // Can't return different types
    }
}
```

**Solution:** Use trait objects (covered below) or enums.

---

## Common Standard Library Traits

### `Clone` — Explicit Duplication

```rust
#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = p1.clone();  // Explicit clone
    
    println!("p1: ({}, {})", p1.x, p1.y);
    println!("p2: ({}, {})", p2.x, p2.y);
}
```

### `Copy` — Implicit Duplication

```rust
#[derive(Copy, Clone)]  // Copy requires Clone
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = p1;  // Copy (not move)
    
    println!("p1: ({}, {})", p1.x, p1.y);  // Still valid
}
```

**Requirements:** All fields must implement `Copy`.

### `Debug` — Debug Formatting

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("{:?}", rect);   // Debug format
    println!("{:#?}", rect);  // Pretty-print debug
}
```

### `Display` — User-Facing Formatting

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("{}", p);  // Output: (10, 20)
}
```

### `PartialEq` and `Eq` — Equality Comparison

```rust
#[derive(PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5, y: 10 };
    
    if p1 == p2 {
        println!("Equal!");
    }
}
```

### `PartialOrd` and `Ord` — Ordering

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 3, y: 8 };
    
    if p1 > p2 {
        println!("p1 is greater");
    }
}
```

### `Default` — Default Values

```rust
#[derive(Default)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point::default();  // x: 0, y: 0
    println!("({}, {})", p.x, p.y);
}
```

---

## Deriving Traits

### Automatic Implementation

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}
```

**Derivable traits:**
- `Debug`, `Clone`, `Copy`
- `PartialEq`, `Eq`
- `PartialOrd`, `Ord`
- `Hash`, `Default`

### Custom Implementation

```rust
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
```

---

## Trait Objects (Dynamic Dispatch)

### Box<dyn Trait>

```rust
trait Draw {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}

struct Square {
    side: f64,
}

impl Draw for Square {
    fn draw(&self) {
        println!("Drawing square with side {}", self.side);
    }
}

fn main() {
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Square { side: 10.0 }),
    ];
    
    for shape in shapes {
        shape.draw();
    }
}
```

**Dynamic dispatch** — method calls resolved at runtime.

### `&dyn Trait` (Borrowed Trait Objects)

```rust
fn draw_shape(shape: &dyn Draw) {
    shape.draw();
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let square = Square { side: 10.0 };
    
    draw_shape(&circle);
    draw_shape(&square);
}
```

---

## Associated Types

### Defining Associated Types

```rust
trait Iterator {
    type Item;  // Associated type
    
    fn next(&mut self) -> Option<Self::Item>;
}
```

### Implementing with Associated Types

```rust
struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;  // Specify associated type
    
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

### Associated Types vs. Generics

**Associated types:**
```rust
trait Container {
    type Item;
    fn get(&self) -> &Self::Item;
}
```

**Generics:**
```rust
trait Container<T> {
    fn get(&self) -> &T;
}
```

**Use associated types when:** There's only one logical implementation per type.

---

## Supertraits

A trait can require other traits:

```rust
use std::fmt::Display;

trait Summary: Display {
    fn summarize(&self) -> String {
        format!("Summary: {}", self)  // Can use Display
    }
}

impl Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.headline)
    }
}

impl Summary for Article {}  // Must also implement Display
```

---

## Trait Bounds on Generics

### Basic Trait Bound

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}
```

### Multiple Bounds

```rust
fn print_and_compare<T: Display + PartialOrd>(a: T, b: T) {
    if a > b {
        println!("{} is greater", a);
    } else {
        println!("{} is greater or equal", b);
    }
}
```

### Where Clauses

```rust
fn some_function<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // Function body
}
```

---

## Blanket Implementations

Implement a trait for all types that satisfy certain bounds:

```rust
trait MyTrait {
    fn my_method(&self);
}

// Implement for all types that implement Display
impl<T: Display> MyTrait for T {
    fn my_method(&self) {
        println!("Value: {}", self);
    }
}

fn main() {
    let x = 5;
    x.my_method();  // Works because i32 implements Display
}
```

**Standard library example:**
```rust
impl<T: Display> ToString for T {
    // Automatic ToString for all Display types
}
```

---

## Orphan Rule

You can implement a trait for a type only if:
- The trait is defined in your crate, OR
- The type is defined in your crate

### ✅ Valid Implementations

```rust
// Your trait, your type
trait MyTrait {}
struct MyType;
impl MyTrait for MyType {}

// Your trait, external type
trait MyTrait2 {}
impl MyTrait2 for Vec<i32> {}

// External trait, your type
use std::fmt::Display;
struct MyType2;
impl Display for MyType2 { /* ... */ }
```

### ❌ Invalid Implementation

```rust
// Can't implement external trait for external type
use std::fmt::Display;
impl Display for Vec<i32> {  // ❌ Compile error
    // Neither Display nor Vec is defined in this crate
}
```

**Reason:** Prevents conflicting implementations across crates.

---

## Operator Overloading

### `Add` Trait

```rust
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;  // Uses Add implementation
    
    println!("{:?}", p3);  // Point { x: 4, y: 6 }
}
```

### Other Operator Traits

| Trait | Operator | Method |
|---|---|---|
| `Add` | `+` | `add` |
| `Sub` | `-` | `sub` |
| `Mul` | `*` | `mul` |
| `Div` | `/` | `div` |
| `Rem` | `%` | `rem` |
| `Neg` | `-` (unary) | `neg` |
| `Not` | `!` | `not` |
| `Index` | `[]` | `index` |

---

## Marker Traits

Traits with no methods — used to mark types:

### `Copy` (Marker Trait)

```rust
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
```

### `Send` and `Sync` (Concurrency Markers)

```rust
// Automatically implemented by compiler
// Send: can be transferred between threads
// Sync: can be referenced from multiple threads
```

---

## Advanced Trait Features

### Associated Functions in Traits

```rust
trait Constructible {
    fn new() -> Self;
    fn default_value() -> i32 {
        0  // Default implementation
    }
}

impl Constructible for Point {
    fn new() -> Self {
        Point { x: 0, y: 0 }
    }
}
```

### Traits with Lifetimes

```rust
trait Validator<'a> {
    fn validate(&self, input: &'a str) -> bool;
}
```

### Trait with Generic Methods

```rust
trait Converter {
    fn convert<T>(&self) -> T
    where
        T: From<i32>;
}
```

---

## Common Patterns

### Strategy Pattern

```rust
trait CompressionStrategy {
    fn compress(&self, data: &[u8]) -> Vec<u8>;
}

struct ZipCompression;
impl CompressionStrategy for ZipCompression {
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        // ZIP compression logic
        data.to_vec()
    }
}

struct GzipCompression;
impl CompressionStrategy for GzipCompression {
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        // GZIP compression logic
        data.to_vec()
    }
}

fn compress_data(data: &[u8], strategy: &dyn CompressionStrategy) -> Vec<u8> {
    strategy.compress(data)
}
```

### Builder Pattern with Traits

```rust
trait Builder {
    type Output;
    fn build(self) -> Self::Output;
}
```

### Extension Trait Pattern

```rust
trait StringExt {
    fn reverse(&self) -> String;
}

impl StringExt for String {
    fn reverse(&self) -> String {
        self.chars().rev().collect()
    }
}

fn main() {
    let s = String::from("hello");
    println!("{}", s.reverse());  // olleh
}
```

---

## Common Mistakes

### Mistake 1: Not Importing Trait

```rust
use std::fmt::Display;

fn print_it(item: &dyn Display) {
    println!("{}", item);
}

// Must import trait to use its methods
use std::io::Write;
let mut buffer = Vec::new();
buffer.write_all(b"hello");  // Must have Write in scope
```

### Mistake 2: Returning Different Types

```rust
fn get_summary(flag: bool) -> impl Summary {
    if flag {
        Article { /* ... */ }  // ❌ Error
    } else {
        Tweet { /* ... */ }    // Can't return different concrete types
    }
}
```

**Fix:** Use trait objects:
```rust
fn get_summary(flag: bool) -> Box<dyn Summary> {
    if flag {
        Box::new(Article { /* ... */ })
    } else {
        Box::new(Tweet { /* ... */ })
    }
}
```

### Mistake 3: Missing Trait Bounds

```rust
fn compare<T>(a: T, b: T) -> bool {
    a > b  // ❌ Error: T might not implement PartialOrd
}
```

**Fix:**
```rust
fn compare<T: PartialOrd>(a: T, b: T) -> bool {
    a > b  // ✅ Works
}
```

---

## Performance Considerations

### Static Dispatch (Monomorphization)

```rust
fn process<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}
```

**Zero runtime cost** — compiler generates specialized code for each type.

### Dynamic Dispatch (Trait Objects)

```rust
fn process(item: &dyn Summary) {
    println!("{}", item.summarize());
}
```

**Small runtime cost** — uses vtable for method lookup.

### When to Use Each

| Use Static Dispatch When | Use Dynamic Dispatch When |
|---|---|
| Performance is critical | Need heterogeneous collections |
| Know types at compile time | Building plugin systems |
| Code size isn't a concern | Flexibility more important than speed |

---

## Summary Table

| Feature | Syntax | Use Case |
|---|---|---|
| **Define Trait** | `trait Name { }` | Specify shared behavior |
| **Implement** | `impl Trait for Type` | Add behavior to type |
| **Default Method** | Method with body in trait | Provide default implementation |
| **Trait Bound** | `T: Trait` | Constrain generics |
| **Multiple Bounds** | `T: Trait1 + Trait2` | Require multiple traits |
| **Trait Object** | `&dyn Trait` | Dynamic dispatch |
| **Associated Type** | `type Item;` | Type associated with trait |
| **Supertrait** | `trait Sub: Super` | Require another trait |
| **Derive** | `#[derive(Trait)]` | Auto-implement traits |

---

## Key Takeaways

1. **Traits define shared behavior** — like interfaces in other languages

2. **Implement with `impl Trait for Type`** — add behavior to types

3. **Default implementations** — traits can provide method bodies

4. **Trait bounds** — constrain generic types

5. **`impl Trait` syntax** — accept any type implementing a trait

6. **Trait objects (`dyn Trait`)** — enable dynamic dispatch

7. **Derive macros** — automatically implement common traits

8. **Associated types** — types associated with trait implementations

9. **Orphan rule** — can only implement trait if trait or type is local

10. **Zero-cost abstractions** — static dispatch has no runtime overhead

---

## Best Practices

✅ **Do:**
- Use trait bounds to make generic code more flexible
- Derive traits when possible (`Debug`, `Clone`, etc.)
- Prefer static dispatch (`impl Trait`) for performance
- Use trait objects (`dyn Trait`) for heterogeneous collections
- Keep traits focused and single-purpose
- Document trait requirements and guarantees

❌ **Don't:**
- Forget to import traits before using their methods
- Try to return different concrete types from `impl Trait`
- Implement external traits for external types (orphan rule)
- Overuse trait objects when static dispatch would work
- Create traits with too many required methods

---

## Next Steps

- Study **trait objects and dynamic dispatch** in depth
- Learn **advanced trait features** (GATs, associated constants)
- Explore **generic programming** with trait bounds
- Understand **lifetimes with traits**
- Learn **derive macros** and procedural macros

---

**Confidence Level:** High ✅  
**Recommended Practice:** Create several traits, implement them for different types, use trait bounds in generic functions, and experiment with trait objects.
