# 🦀 Rust Tuples

**Topic:** Working with tuples — fixed-size collections of heterogeneous values  
**Skill Level:** Beginner

---

## Overview

**Tuples** are compound data types that group together multiple values of different types into a single value. They are:

- **Fixed-size** — cannot grow or shrink after creation
- **Heterogeneous** — can contain values of different types
- **Ordered** — elements have a specific position
- **Stack-allocated** — typically stored on the stack (fast access)

---

## Creating Tuples

### Basic Syntax

```rust
fn main() {
    let tuple = (1, "hello", 3.14);
    println!("{:?}", tuple);
}
```

**Components:**
- Parentheses `( )` enclose the elements
- Commas `,` separate the values
- Can mix different types

### With Type Annotations

```rust
fn main() {
    let tuple: (i32, &str, f64) = (1, "hello", 3.14);
    println!("{:?}", tuple);
}
```

**Type signature:** `(Type1, Type2, Type3, ...)`

### Empty Tuple (Unit Type)

```rust
fn main() {
    let unit: () = ();
    println!("{:?}", unit);
}
```

- The **unit type** `()` is a special tuple with zero elements
- Represents "no meaningful value"
- Used when functions don't return anything

---

## Accessing Tuple Elements

### Using Dot Notation with Index

```rust
fn main() {
    let tuple = (10, "Rust", 3.14);
    
    let first = tuple.0;   // Access first element (i32)
    let second = tuple.1;  // Access second element (&str)
    let third = tuple.2;   // Access third element (f64)
    
    println!("First: {}", first);
    println!("Second: {}", second);
    println!("Third: {}", third);
}
```

**Key Points:**
- Use `.0`, `.1`, `.2`, etc. to access elements
- Indexing is **zero-based**
- Type is known at compile time

### ❌ Cannot Use Variables for Indexing

```rust
fn main() {
    let tuple = (1, 2, 3);
    let i = 0;
    
    let value = tuple.i;  // ❌ Compile error
    let value = tuple[0]; // ❌ Compile error: tuples don't support bracket indexing
}
```

Tuples require **compile-time constant indices** — you can't use variables or runtime values.

---

## Destructuring Tuples

### Basic Destructuring (Pattern Matching)

```rust
fn main() {
    let tuple = (10, "Rust", 3.14);
    
    let (x, y, z) = tuple;  // Unpack all elements
    
    println!("x: {}", x);  // 10
    println!("y: {}", y);  // Rust
    println!("z: {}", z);  // 3.14
}
```

**This is the most common and idiomatic way to work with tuples!**

### Partial Destructuring with `_`

```rust
fn main() {
    let tuple = (10, "Rust", 3.14, true);
    
    let (x, _, z, _) = tuple;  // Ignore 2nd and 4th elements
    
    println!("x: {}", x);  // 10
    println!("z: {}", z);  // 3.14
}
```

Use `_` to ignore values you don't need.

### Destructuring in Function Parameters

```rust
fn print_coordinates(point: (i32, i32)) {
    let (x, y) = point;
    println!("x: {}, y: {}", x, y);
}

// Or destructure directly in the parameter
fn print_coords((x, y): (i32, i32)) {
    println!("x: {}, y: {}", x, y);
}

fn main() {
    let point = (10, 20);
    print_coordinates(point);
    print_coords(point);
}
```

---

## Tuple Length and Type

### Fixed Length

```rust
fn main() {
    let tuple2 = (1, 2);
    let tuple3 = (1, 2, 3);
    
    // These are DIFFERENT types:
    // tuple2 is type (i32, i32)
    // tuple3 is type (i32, i32, i32)
}
```

**Important:** Tuples of different lengths are **different types** — even if they contain the same element types.

### Maximum Size

Tuples can have up to **12 elements** for many standard library traits (like `Debug`, `Clone`, etc.). Beyond that, some functionality may be limited.

```rust
// This works fine
let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);

// More than 12 may have limited trait support
let big_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
```

---

## Tuples in Functions

### Returning Multiple Values

```rust
fn get_user_info() -> (String, u32, bool) {
    let name = String::from("Alice");
    let age = 30;
    let is_active = true;
    
    (name, age, is_active)  // Return as tuple
}

fn main() {
    let (name, age, active) = get_user_info();
    println!("{} is {} years old, active: {}", name, age, active);
}
```

**Common Use Case:** Functions that need to return multiple related values.

### Passing Tuples as Parameters

```rust
fn calculate_area(dimensions: (f64, f64)) -> f64 {
    let (width, height) = dimensions;
    width * height
}

fn main() {
    let rect = (10.0, 5.0);
    let area = calculate_area(rect);
    println!("Area: {}", area);
}
```

---

## Nested Tuples

Tuples can contain other tuples:

```rust
fn main() {
    let nested = ((1, 2), (3, 4), (5, 6));
    
    // Access nested elements
    let first_tuple = nested.0;     // (1, 2)
    let first_element = nested.0.0; // 1
    
    println!("First tuple: {:?}", first_tuple);
    println!("First element: {}", first_element);
}
```

### Destructuring Nested Tuples

```rust
fn main() {
    let nested = ((1, 2), (3, 4));
    
    let ((a, b), (c, d)) = nested;
    
    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
}
```

---

## Mutable Tuples

### Modifying Tuple Elements

```rust
fn main() {
    let mut tuple = (1, 2, 3);
    
    tuple.0 = 10;  // Modify first element
    tuple.2 = 30;  // Modify third element
    
    println!("{:?}", tuple);  // Output: (10, 2, 30)
}
```

**Note:** You can modify elements, but you **cannot change the tuple's size or types**.

### Cannot Change Type or Length

```rust
fn main() {
    let mut tuple = (1, 2, 3);
    
    tuple = (1, 2);        // ❌ Compile error: mismatched types
    tuple = (1, 2, 3, 4);  // ❌ Compile error: mismatched types
    tuple.0 = "hello";     // ❌ Compile error: expected i32, found &str
}
```

---

## Single-Element Tuples

### Requires Trailing Comma

```rust
fn main() {
    let not_a_tuple = (5);      // This is just an i32 in parentheses
    let tuple = (5,);           // This is a tuple with one element
    
    println!("{:?}", not_a_tuple);  // 5
    println!("{:?}", tuple);        // (5,)
}
```

**Important:** The trailing comma `,` distinguishes a single-element tuple from a parenthesized expression.

---

## Tuples vs. Arrays

| Feature | Tuples | Arrays |
|---|---|---|
| **Element Types** | Different types allowed | All elements must be same type |
| **Size** | Fixed at compile time | Fixed at compile time |
| **Access** | Dot notation (`.0`, `.1`) | Bracket notation (`[0]`, `[1]`) |
| **Indexing** | Compile-time constant only | Can use variables at runtime |
| **Use Case** | Group related values of different types | Collection of same-type values |

### Example Comparison

```rust
fn main() {
    // Tuple: mixed types
    let tuple: (i32, &str, f64) = (1, "hello", 3.14);
    let first = tuple.0;
    
    // Array: same type
    let array: [i32; 3] = [1, 2, 3];
    let first = array[0];
    let i = 0;
    let dynamic = array[i];  // Arrays allow runtime indexing
}
```

---

## Common Use Cases

### 1. Returning Multiple Values from Functions

```rust
fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}

fn main() {
    let (q, r) = divide_with_remainder(10, 3);
    println!("10 / 3 = {} remainder {}", q, r);
}
```

### 2. Swapping Values

```rust
fn main() {
    let mut a = 5;
    let mut b = 10;
    
    // Swap using tuple destructuring
    (a, b) = (b, a);
    
    println!("a: {}, b: {}", a, b);  // a: 10, b: 5
}
```

### 3. Coordinates and Points

```rust
fn main() {
    let point: (f64, f64) = (3.0, 4.0);
    let (x, y) = point;
    
    let distance = (x.powi(2) + y.powi(2)).sqrt();
    println!("Distance from origin: {}", distance);
}
```

### 4. RGB Colors

```rust
fn main() {
    let color: (u8, u8, u8) = (255, 128, 0);  // Orange
    let (red, green, blue) = color;
    
    println!("RGB: ({}, {}, {})", red, green, blue);
}
```

### 5. Key-Value Pairs (Simple Cases)

```rust
fn main() {
    let config: (&str, i32) = ("max_connections", 100);
    let (key, value) = config;
    
    println!("{}: {}", key, value);
}
```

---

## Printing Tuples

### Using `{:?}` Debug Format

```rust
fn main() {
    let tuple = (1, "hello", 3.14);
    println!("{:?}", tuple);  // Output: (1, "hello", 3.14)
}
```

### Pretty Printing with `{:#?}`

```rust
fn main() {
    let nested = ((1, 2), (3, 4), (5, 6));
    println!("{:#?}", nested);
}
```

**Output:**
```
(
    (1, 2),
    (3, 4),
    (5, 6),
)
```

### Cannot Use `{}` Display Format

```rust
fn main() {
    let tuple = (1, 2, 3);
    println!("{}", tuple);  // ❌ Compile error: tuples don't implement Display
}
```

Must use `{:?}` for debug formatting.

---

## Tuples with References

```rust
fn main() {
    let name = String::from("Rust");
    let age = 5;
    
    let tuple = (&name, &age);  // Tuple of references
    
    println!("Name: {}, Age: {}", tuple.0, tuple.1);
    
    // Original values still accessible
    println!("Original name: {}", name);
}
```

---

## Pattern Matching with Tuples

### Using `match`

```rust
fn main() {
    let point = (0, 5);
    
    match point {
        (0, 0) => println!("Origin"),
        (0, y) => println!("On Y-axis at {}", y),
        (x, 0) => println!("On X-axis at {}", x),
        (x, y) => println!("Point at ({}, {})", x, y),
    }
}
```

### Using `if let`

```rust
fn main() {
    let point = (0, 5);
    
    if let (0, y) = point {
        println!("On Y-axis at {}", y);
    }
}
```

### Match with Guards

```rust
fn main() {
    let pair = (2, -2);
    
    match pair {
        (x, y) if x == y => println!("Equal"),
        (x, y) if x + y == 0 => println!("Sum is zero"),
        (x, y) if x > y => println!("First is greater"),
        _ => println!("Something else"),
    }
}
```

---

## Common Patterns and Idioms

### Ignoring Values

```rust
fn main() {
    let tuple = (1, 2, 3, 4, 5);
    
    // Only care about first and last
    let (first, _, _, _, last) = tuple;
    println!("First: {}, Last: {}", first, last);
}
```

### Using `..` to Ignore Remaining Elements

```rust
fn main() {
    let tuple = (1, 2, 3, 4, 5);
    
    let (first, .., last) = tuple;
    println!("First: {}, Last: {}", first, last);
}
```

**Note:** This syntax is more commonly used with slices and arrays.

### Multiple Returns in Iterators

```rust
fn main() {
    let pairs = vec![(1, 2), (3, 4), (5, 6)];
    
    for (x, y) in pairs {
        println!("x: {}, y: {}", x, y);
    }
}
```

---

## Tuple Structs (Named Tuples)

For better readability, use **tuple structs** with named types:

```rust
struct Point(i32, i32);
struct Color(u8, u8, u8);

fn main() {
    let point = Point(10, 20);
    let color = Color(255, 0, 0);
    
    println!("Point: ({}, {})", point.0, point.1);
    println!("Color: ({}, {}, {})", color.0, color.1, color.2);
}
```

**Advantage:** `Point` and `Color` are distinct types, even though they're both tuples.

---

## Performance Considerations

### Stack Allocation

```rust
fn main() {
    let tuple = (1, 2, 3);  // Allocated on stack (fast)
}
```

Tuples are typically stack-allocated, making them very efficient.

### Zero-Cost Abstraction

```rust
fn main() {
    let tuple = (10, 20);
    let (x, y) = tuple;  // No runtime cost — compiler optimizes this away
}
```

Destructuring tuples has **no runtime overhead** — it's purely a compile-time operation.

---

## Common Mistakes

### Mistake 1: Forgetting Trailing Comma for Single-Element Tuple

```rust
let not_tuple = (5);   // ❌ This is just i32
let tuple = (5,);      // ✅ This is a tuple
```

### Mistake 2: Trying to Use Variables for Indexing

```rust
let tuple = (1, 2, 3);
let i = 0;
let value = tuple[i];  // ❌ Tuples don't support bracket indexing
```

### Mistake 3: Mismatched Destructuring

```rust
let tuple = (1, 2, 3);
let (a, b) = tuple;  // ❌ Compile error: pattern has 2 elements, tuple has 3
```

**Fix:**
```rust
let (a, b, c) = tuple;  // ✅ Match all elements
// or
let (a, b, _) = tuple;  // ✅ Ignore third element
```

### Mistake 4: Trying to Change Tuple Size

```rust
let mut tuple = (1, 2, 3);
tuple = (1, 2);  // ❌ Compile error: different types
```

---

## Summary Table

| Feature | Details |
|---|---|
| **Syntax** | `(value1, value2, ...)` |
| **Access** | `.0`, `.1`, `.2` (dot notation) |
| **Types** | Can mix different types |
| **Size** | Fixed at compile time |
| **Mutability** | Elements can be modified if tuple is `mut` |
| **Max Size** | Up to 12 elements for full trait support |
| **Indexing** | Compile-time constant only |
| **Unit Type** | `()` represents no value |
| **Single Element** | Requires trailing comma: `(5,)` |

---

## Key Takeaways

1. **Fixed-size, heterogeneous** — tuples group different types together

2. **Dot notation for access** — use `.0`, `.1`, `.2` (not brackets)

3. **Destructuring is idiomatic** — `let (x, y, z) = tuple;`

4. **Perfect for multiple returns** — return multiple values from functions

5. **Stack-allocated and fast** — zero-cost abstraction

6. **Single-element requires comma** — `(5,)` not `(5)`

7. **Cannot change size or types** — even with `mut`

8. **Use `_` to ignore values** — in destructuring patterns

9. **Consider tuple structs** — for named, type-safe tuples

10. **Max 12 elements** — for full standard library trait support

---

## Best Practices

✅ **Do:**
- Use tuples for small, related groups of values
- Destructure tuples for readability
- Use tuples to return multiple values from functions
- Consider tuple structs for better type safety
- Use `_` to ignore unneeded values

❌ **Don't:**
- Create tuples with more than 4-5 elements (use structs instead)
- Use tuples when a struct would be clearer
- Forget the trailing comma for single-element tuples
- Try to index tuples with variables
- Use tuples for large, complex data structures

---

## When to Use Tuples vs. Structs

### Use Tuples When:
- You have 2-4 related values
- The relationship is obvious from context
- It's a temporary grouping
- You're returning multiple values from a function

### Use Structs When:
- You have many fields (5+)
- Field names would improve clarity
- The data structure is part of your API
- You need methods on the type

---

## Next Steps

- Learn about **arrays** and **vectors** for collections of same-type values
- Study **structs** for named, structured data
- Explore **pattern matching** in depth with `match`
- Understand **ownership** and how it applies to compound types

---

**Confidence Level:** High ✅  
**Recommended Practice:** Create functions that return tuples and practice destructuring patterns.
