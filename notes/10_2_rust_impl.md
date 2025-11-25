# 🦀 Rust `impl` Blocks (Implementation Blocks)

**Topic:** Adding methods and associated functions to types using `impl`  
**Skill Level:** Beginner to Intermediate

---

## Overview

**`impl` blocks** (short for "implementation") allow you to define methods and associated functions for structs, enums, and other types. They are:

- **Methods** — functions that operate on instances of a type
- **Associated functions** — functions tied to the type itself (like static methods)
- **Organized** — group related functionality together
- **Foundation for OOP patterns** — encapsulation and behavior

`impl` blocks are how Rust adds behavior to data structures.

---

## Basic Syntax

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Methods and associated functions go here
}
```

**Structure:**
- `impl` keyword
- Type name
- Curly braces `{ }` containing functions

---

## Methods vs. Associated Functions

### Methods (With `self`)

Functions that take `self` as the first parameter — operate on an instance.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("Area: {}", rect.area());  // Method call with dot notation
}
```

### Associated Functions (Without `self`)

Functions that don't take `self` — called on the type itself.

```rust
impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let rect = Rectangle::new(30, 50);  // Called with :: notation
}
```

**Think of associated functions as "static methods" from other languages.**

---

## The Three Forms of `self`

### 1. `&self` — Immutable Borrow (Most Common)

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height  // Read-only access
    }
    
    fn width(&self) -> u32 {
        self.width
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("Area: {}", rect.area());
    println!("Width: {}", rect.width());  // Can call multiple times
}
```

**Use when:** You need to read data but not modify it.

### 2. `&mut self` — Mutable Borrow

```rust
impl Rectangle {
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
    
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {
    let mut rect = Rectangle { width: 30, height: 50 };
    rect.scale(2);  // Modifies rect
    println!("New width: {}", rect.width);  // 60
}
```

**Use when:** You need to modify the instance.

### 3. `self` — Takes Ownership (Consumes)

```rust
impl Rectangle {
    fn destroy(self) -> String {
        format!("Rectangle {}x{} destroyed", self.width, self.height)
    }
    
    fn into_square(self) -> Rectangle {
        let size = self.width.max(self.height);
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    let message = rect.destroy();  // rect is consumed here
    println!("{}", message);
    // println!("{}", rect.width);  // ❌ Error: rect was moved
}
```

**Use when:** The method consumes the value (transforms it or cleans up).

---

## Method Call Syntax

Rust automatically references/dereferences when calling methods:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    
    // All of these are equivalent:
    rect.area();           // Rust automatically borrows
    (&rect).area();        // Explicit borrow
    Rectangle::area(&rect); // Function call syntax
}
```

**Automatic referencing** is one of Rust's ergonomic features — you rarely need to write `&` explicitly.

---

## Associated Functions (Constructors)

### Standard Constructor Pattern

```rust
impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let rect = Rectangle::new(30, 50);
}
```

### Named Constructors

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    fn default() -> Rectangle {
        Rectangle {
            width: 1,
            height: 1,
        }
    }
}

fn main() {
    let sq = Rectangle::square(25);
    let default_rect = Rectangle::default();
}
```

### Builder Pattern Constructor

```rust
impl Rectangle {
    fn new() -> Self {  // Self is an alias for Rectangle
        Self { width: 0, height: 0 }
    }
    
    fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }
    
    fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
}

fn main() {
    let rect = Rectangle::new()
        .with_width(30)
        .with_height(50);
}
```

---

## Using `Self` Type Alias

`Self` is an alias for the type being implemented:

```rust
impl Rectangle {
    // These are equivalent:
    fn new(width: u32, height: u32) -> Rectangle { /* ... */ }
    fn new(width: u32, height: u32) -> Self { /* ... */ }
    
    // Self is especially useful in generics
    fn clone_and_scale(&self, factor: u32) -> Self {
        Self {
            width: self.width * factor,
            height: self.height * factor,
        }
    }
}
```

**Advantage:** If you rename the type, you don't have to update all return types.

---

## Multiple `impl` Blocks

You can have multiple `impl` blocks for the same type:

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

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
```

**When to use:**
- Organizing related methods (e.g., constructors vs. calculations)
- Implementing traits (each trait gets its own `impl` block)
- Conditional compilation (`#[cfg(...)]`)

**Usually:** Keep all methods in one `impl` block for simplicity.

---

## Methods Can Take Additional Parameters

```rust
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    fn expand_by(&mut self, width_delta: u32, height_delta: u32) {
        self.width += width_delta;
        self.height += height_delta;
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 20, height: 40 };
    
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    
    let mut rect3 = Rectangle { width: 10, height: 10 };
    rect3.expand_by(5, 5);
}
```

---

## Implementing Methods for Tuple Structs

```rust
struct Point(i32, i32);

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point(x, y)
    }
    
    fn x(&self) -> i32 {
        self.0
    }
    
    fn y(&self) -> i32 {
        self.1
    }
    
    fn distance_from_origin(&self) -> f64 {
        ((self.0.pow(2) + self.1.pow(2)) as f64).sqrt()
    }
}

fn main() {
    let point = Point::new(3, 4);
    println!("Distance: {}", point.distance_from_origin());
}
```

---

## Implementing Methods for Enums

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write: {}", text),
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

---

## Getters and Setters

### Getter Pattern

```rust
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn age(&self) -> u32 {
        self.age
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    println!("Name: {}", person.name());
}
```

### Setter Pattern

```rust
impl Person {
    fn set_age(&mut self, age: u32) {
        self.age = age;
    }
    
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

fn main() {
    let mut person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    person.set_age(31);
}
```

**Note:** Rust doesn't require getters/setters for public fields — only use them when you need validation or encapsulation.

---

## Method Chaining

Methods that return `Self` or `&mut Self` can be chained:

```rust
impl Rectangle {
    fn scale(&mut self, factor: u32) -> &mut Self {
        self.width *= factor;
        self.height *= factor;
        self
    }
    
    fn shift(&mut self, dx: u32, dy: u32) -> &mut Self {
        self.width += dx;
        self.height += dy;
        self
    }
}

fn main() {
    let mut rect = Rectangle { width: 10, height: 20 };
    
    rect.scale(2)
        .shift(5, 5)
        .scale(3);
    
    println!("Width: {}", rect.width);  // (10 * 2 + 5) * 3 = 75
}
```

---

## Private vs. Public Methods

```rust
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Public method
    pub fn area(&self) -> u32 {
        self.calculate_area()  // Calls private helper
    }
    
    // Private method (no pub)
    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }
    
    // Public constructor
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
```

**Private methods** can only be called from within the module. Use them for internal helpers.

---

## The `self` Parameter Explained

### Full Syntax (Rarely Needed)

```rust
impl Rectangle {
    // These are equivalent:
    fn area(&self) -> u32 { /* ... */ }
    fn area(self: &Self) -> u32 { /* ... */ }
    fn area(self: &Rectangle) -> u32 { /* ... */ }
}
```

**Rust shorthand:** `&self` is sugar for `self: &Self`

### Why Three Forms?

| Form | Full Syntax | Ownership | Use Case |
|---|---|---|---|
| `&self` | `self: &Self` | Borrow | Read-only access |
| `&mut self` | `self: &mut Self` | Mutable borrow | Modify instance |
| `self` | `self: Self` | Take ownership | Consume/transform |

---

## Common Patterns

### Constructor Validation

```rust
impl Rectangle {
    fn new(width: u32, height: u32) -> Result<Self, String> {
        if width == 0 || height == 0 {
            Err(String::from("Dimensions must be greater than 0"))
        } else {
            Ok(Self { width, height })
        }
    }
}

fn main() {
    match Rectangle::new(10, 20) {
        Ok(rect) => println!("Created: {:?}", rect),
        Err(e) => println!("Error: {}", e),
    }
}
```

### Conversion Methods

```rust
impl Rectangle {
    fn into_square(self) -> Square {
        let size = self.width.max(self.height);
        Square { size }
    }
    
    fn as_tuple(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
```

### Comparison Methods

```rust
impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
    
    fn is_larger_than(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }
}
```

---

## Generic `impl` Blocks

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
}

// Implement additional methods for specific types
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let int_point = Point::new(5, 10);
    let float_point = Point::new(3.0, 4.0);
    
    println!("Distance: {}", float_point.distance_from_origin());
}
```

---

## Implementing Traits

```rust
use std::fmt;

struct Rectangle {
    width: u32,
    height: u32,
}

// Implement the Display trait
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rectangle({}x{})", self.width, self.height)
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("{}", rect);  // Uses Display implementation
}
```

**Trait implementations** use the syntax: `impl TraitName for TypeName`

---

## Associated Constants

```rust
impl Rectangle {
    const MAX_WIDTH: u32 = 1000;
    const MAX_HEIGHT: u32 = 1000;
    
    fn new(width: u32, height: u32) -> Result<Self, String> {
        if width > Self::MAX_WIDTH || height > Self::MAX_HEIGHT {
            Err(String::from("Dimensions exceed maximum"))
        } else {
            Ok(Self { width, height })
        }
    }
}

fn main() {
    println!("Max width: {}", Rectangle::MAX_WIDTH);
}
```

---

## Documentation Comments in `impl` Blocks

```rust
impl Rectangle {
    /// Creates a new Rectangle with the given dimensions.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let rect = Rectangle::new(30, 50);
    /// ```
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    
    /// Calculates the area of the rectangle.
    /// 
    /// # Returns
    /// 
    /// The area as a `u32`.
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

**Use `///` for documentation comments** — they appear in generated docs.

---

## Common Mistakes

### Mistake 1: Forgetting `&` in Method Definition

```rust
impl Rectangle {
    fn area(self) -> u32 {  // ❌ Takes ownership
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    let area = rect.area();  // rect is moved here
    // println!("{}", rect.width);  // ❌ Error: rect was moved
}
```

**Fix:** Use `&self` for read-only methods:
```rust
fn area(&self) -> u32 { /* ... */ }
```

### Mistake 2: Using Method Syntax for Associated Functions

```rust
impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn main() {
    let rect = Rectangle { width: 0, height: 0 };
    let new_rect = rect.new(30, 50);  // ❌ Error: new doesn't take self
}
```

**Fix:** Use `::` notation:
```rust
let rect = Rectangle::new(30, 50);  // ✅
```

### Mistake 3: Multiple Mutable Borrows

```rust
impl Rectangle {
    fn compare_and_update(&mut self, other: &mut Rectangle) {
        if self.width > other.width {
            other.width = self.width;
        }
    }
}

fn main() {
    let mut rect = Rectangle { width: 30, height: 50 };
    rect.compare_and_update(&mut rect);  // ❌ Error: can't borrow as mutable twice
}
```

**This is a borrow checker limitation** — can't borrow `rect` mutably twice at once.

### Mistake 4: Returning Reference to Local Data

```rust
impl Rectangle {
    fn get_dimensions(&self) -> &(u32, u32) {
        let dims = (self.width, self.height);
        &dims  // ❌ Error: returns reference to local variable
    }
}
```

**Fix:** Return owned data:
```rust
fn get_dimensions(&self) -> (u32, u32) {
    (self.width, self.height)  // ✅
}
```

---

## Performance Considerations

### Zero-Cost Abstraction

```rust
impl Rectangle {
    #[inline]
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

Methods are **zero-cost** — the compiler can inline them, making them as fast as direct field access.

### Method Call Overhead

```rust
// These compile to the same machine code:
let area = rect.area();
let area = rect.width * rect.height;
```

**No runtime overhead** for method calls in release builds.

---

## Summary Table

| Feature | Syntax | Use Case |
|---|---|---|
| **Method** | `fn name(&self)` | Operate on instance |
| **Mutable Method** | `fn name(&mut self)` | Modify instance |
| **Consuming Method** | `fn name(self)` | Take ownership |
| **Associated Function** | `fn name()` | No instance needed |
| **Constructor** | `fn new() -> Self` | Create instance |
| **Method Chaining** | `return &mut self` | Fluent API |
| **Generic impl** | `impl<T> Name<T>` | Work with generics |
| **Trait impl** | `impl Trait for Type` | Implement traits |

---

## Key Takeaways

1. **`impl` blocks add behavior** — methods and functions for types

2. **Three forms of `self`** — `&self` (borrow), `&mut self` (mutable), `self` (ownership)

3. **Associated functions** — called with `::`, don't take `self`

4. **Constructors use `new`** — convention, not enforced

5. **`Self` is type alias** — useful for return types and generics

6. **Multiple `impl` blocks allowed** — but usually use one

7. **Methods auto-reference** — Rust adds `&` automatically

8. **Zero-cost abstraction** — no runtime overhead for methods

9. **Private by default** — use `pub` for public methods

10. **Works with structs and enums** — add behavior to any type

---

## Best Practices

✅ **Do:**
- Use `&self` for read-only methods (most common)
- Use `&mut self` when modifying the instance
- Use `self` when consuming/transforming the value
- Group related methods in one `impl` block
- Use `Self` instead of repeating the type name
- Follow naming conventions (`new`, `from`, `into`, etc.)
- Document public methods with `///`

❌ **Don't:**
- Take ownership (`self`) unless you need to consume the value
- Create unnecessary getters/setters for public fields
- Forget `pub` keyword for public methods in public structs
- Return references to local data
- Use method syntax for associated functions

---

## Next Steps

- Study **traits** — define shared behavior across types
- Learn **trait implementations** — `impl Trait for Type`
- Explore **generics in `impl` blocks** — reusable implementations
- Understand **lifetimes in methods** — when returning references
- Learn **operator overloading** — implementing operator traits

---

**Confidence Level:** High ✅  
**Recommended Practice:** Create structs with various methods, practice all three forms of `self`, and implement constructors and utility functions.
