# 🦀 Rust Conditional Statements

**Topic:** Control flow with `if`, `else if`, `else`, and pattern matching  
**Skill Level:** Beginner to Intermediate

---

## Overview

Conditional statements control the flow of your program based on boolean conditions. Rust's conditionals are similar to other languages but with some unique features:

- **No parentheses required** around conditions
- **Can return values** (expressions, not just statements)
- **Type safety enforced** at compile time
- **Pattern matching** with `match` is preferred for complex conditions

---

## The `if` Statement

### Basic Syntax

```rust
fn main() {
    let number = 5;
    
    if number < 10 {
        println!("Number is less than 10");
    }
}
```

**Key Points:**
- ✅ **No parentheses** around the condition (unlike C, Java, JavaScript)
- ✅ **Curly braces are required** (even for single statements)
- ✅ Condition must be a **boolean** (`true` or `false`)

### With Parentheses (Optional but Not Idiomatic)

```rust
if (number < 10) {  // Valid but not Rust style
    println!("Less than 10");
}
```

**Best Practice:** Omit unnecessary parentheses for cleaner Rust code.

---

## `if-else` Statement

### Basic Example

```rust
fn main() {
    let number = 15;
    
    if number < 10 {
        println!("Number is less than 10");
    } else {
        println!("Number is 10 or greater");
    }
}
```

---

## `if-else if-else` Chain

### Multiple Conditions

```rust
fn main() {
    let number = 15;
    
    if number < 10 {
        println!("Less than 10");
    } else if number < 20 {
        println!("Between 10 and 19");
    } else if number < 30 {
        println!("Between 20 and 29");
    } else {
        println!("30 or greater");
    }
}
```

**Note:** Rust evaluates conditions **top to bottom** and stops at the first `true` condition.

---

## Conditions Must Be Boolean

### ❌ This Won't Work

```rust
fn main() {
    let number = 5;
    
    if number {  // ❌ Compile error: expected bool, found integer
        println!("Number exists");
    }
}
```

**Error:** Rust doesn't automatically convert integers to booleans (unlike C or JavaScript).

### ✅ Correct Approach

```rust
fn main() {
    let number = 5;
    
    if number != 0 {  // ✅ Explicit boolean condition
        println!("Number is non-zero");
    }
}
```

---

## `if` as an Expression (Returns a Value)

One of Rust's most powerful features: `if` statements can **return values**.

### Basic Example

```rust
fn main() {
    let number = 5;
    
    let result = if number < 10 {
        "less than 10"
    } else {
        "10 or greater"
    };
    
    println!("Result: {}", result);
}
```

**Key Points:**
- No semicolon after the value you want to return
- Both branches must return the **same type**
- The entire `if-else` becomes an expression

### Using in Functions

```rust
fn categorize_number(num: i32) -> &'static str {
    if num < 0 {
        "negative"
    } else if num == 0 {
        "zero"
    } else {
        "positive"
    }
}

fn main() {
    println!("{}", categorize_number(-5));  // Output: negative
    println!("{}", categorize_number(0));   // Output: zero
    println!("{}", categorize_number(10));  // Output: positive
}
```

---

## Type Consistency in `if` Expressions

### ❌ Mismatched Types

```rust
fn main() {
    let number = 5;
    
    let result = if number < 10 {
        "less than 10"  // &str
    } else {
        42  // ❌ i32 - compile error: types don't match
    };
}
```

**Error:** Both branches must return the same type.

### ✅ Consistent Types

```rust
fn main() {
    let number = 5;
    
    let result = if number < 10 {
        "less than 10"
    } else {
        "10 or greater"
    };
    
    println!("{}", result);
}
```

---

## Comparison Operators

| Operator | Meaning | Example |
|---|---|---|
| `==` | Equal to | `a == b` |
| `!=` | Not equal to | `a != b` |
| `<` | Less than | `a < b` |
| `>` | Greater than | `a > b` |
| `<=` | Less than or equal | `a <= b` |
| `>=` | Greater than or equal | `a >= b` |

### Example

```rust
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
}
```

---

## Logical Operators

### AND (`&&`)

```rust
fn main() {
    let age = 25;
    let has_license = true;
    
    if age >= 18 && has_license {
        println!("Can drive");
    }
}
```

### OR (`||`)

```rust
fn main() {
    let is_weekend = true;
    let is_holiday = false;
    
    if is_weekend || is_holiday {
        println!("Time to relax!");
    }
}
```

### NOT (`!`)

```rust
fn main() {
    let is_raining = false;
    
    if !is_raining {
        println!("Go outside!");
    }
}
```

### Combining Operators

```rust
fn main() {
    let age = 25;
    let has_license = true;
    let has_car = false;
    
    if (age >= 18 && has_license) || has_car {
        println!("Can drive or owns a car");
    }
}
```

**Note:** Parentheses help with readability when combining multiple conditions.

---

## Short-Circuit Evaluation

Rust uses **short-circuit evaluation** for logical operators:

### AND (`&&`)

```rust
fn main() {
    let x = 5;
    
    // If first condition is false, second is never evaluated
    if x < 0 && expensive_function() {
        println!("Both true");
    }
}

fn expensive_function() -> bool {
    println!("This won't print if x >= 0");
    true
}
```

### OR (`||`)

```rust
fn main() {
    let x = 5;
    
    // If first condition is true, second is never evaluated
    if x > 0 || expensive_function() {
        println!("At least one true");
    }
}

fn expensive_function() -> bool {
    println!("This won't print if x > 0");
    true
}
```

---

## Nested `if` Statements

```rust
fn main() {
    let number = 15;
    let is_positive = true;
    
    if is_positive {
        if number < 10 {
            println!("Small positive number");
        } else if number < 100 {
            println!("Medium positive number");
        } else {
            println!("Large positive number");
        }
    } else {
        println!("Not positive");
    }
}
```

**Tip:** Consider using `match` for complex nested conditions (see below).

---

## `if let` (Pattern Matching Light)

Use `if let` when you want to match a pattern and execute code only if it matches.

### Working with `Option`

```rust
fn main() {
    let some_value: Option<i32> = Some(42);
    
    if let Some(x) = some_value {
        println!("Value is: {}", x);
    } else {
        println!("No value");
    }
}
```

### Compared to `match`

```rust
// Using if let
if let Some(x) = some_value {
    println!("{}", x);
}

// Equivalent match
match some_value {
    Some(x) => println!("{}", x),
    None => {}
}
```

**When to use `if let`:**
- You only care about one pattern
- You want cleaner syntax than `match`

---

## `match` Statement (Powerful Pattern Matching)

For complex conditions, `match` is often better than `if-else` chains.

### Basic `match`

```rust
fn main() {
    let number = 3;
    
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }
}
```

### `match` with Ranges

```rust
fn main() {
    let number = 15;
    
    match number {
        0..=10 => println!("Between 0 and 10"),
        11..=20 => println!("Between 11 and 20"),
        21..=30 => println!("Between 21 and 30"),
        _ => println!("Greater than 30"),
    }
}
```

### `match` as an Expression

```rust
fn main() {
    let number = 3;
    
    let description = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "many",
    };
    
    println!("Number is: {}", description);
}
```

### `match` Must Be Exhaustive

```rust
fn main() {
    let number = 5;
    
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        // ❌ Compile error: pattern `_` not covered
    }
}
```

**Solution:** Always include a catch-all pattern (`_`) or cover all possible values.

```rust
match number {
    1 => println!("One"),
    2 => println!("Two"),
    _ => println!("Other"),  // ✅ Exhaustive
}
```

---

## `if` vs. `match`: When to Use Which

### Use `if` when:
- Simple boolean conditions
- 1-3 branches
- Conditions involve ranges or complex logic

```rust
if age >= 18 && has_license {
    println!("Can drive");
}
```

### Use `match` when:
- Matching specific values
- Multiple branches (4+)
- Pattern matching (enums, tuples, etc.)
- Need exhaustive checking

```rust
match status_code {
    200 => println!("OK"),
    404 => println!("Not Found"),
    500 => println!("Server Error"),
    _ => println!("Other"),
}
```

---

## Common Patterns and Idioms

### Ternary-Like Expression

Rust doesn't have a ternary operator (`? :`), but `if-else` expressions work similarly:

```rust
// Other languages: int result = (x > 0) ? 1 : -1;
let result = if x > 0 { 1 } else { -1 };
```

### Guard Clauses (Early Returns)

```rust
fn process_value(value: Option<i32>) {
    if value.is_none() {
        println!("No value provided");
        return;  // Early return
    }
    
    // Continue processing...
    let val = value.unwrap();
    println!("Processing: {}", val);
}
```

### Boolean Assignment

```rust
fn main() {
    let number = 15;
    let is_even = number % 2 == 0;
    
    if is_even {
        println!("Even");
    } else {
        println!("Odd");
    }
}
```

---

## Performance Considerations

### Branch Prediction

Modern CPUs predict which branch will be taken. Rust compiles to efficient machine code:

```rust
// This is efficient
if likely_condition {
    // Most common path
} else {
    // Less common path
}
```

### `match` Optimization

The Rust compiler optimizes `match` statements into jump tables or if-else chains depending on the pattern.

---

## Common Mistakes

### Mistake 1: Using Assignment Instead of Comparison

```rust
let x = 5;

if x = 10 {  // ❌ Compile error: assignment in condition
    println!("x is 10");
}
```

**Fix:** Use `==` for comparison:

```rust
if x == 10 {  // ✅ Comparison
    println!("x is 10");
}
```

### Mistake 2: Forgetting Braces

```rust
// ❌ Compile error: missing braces
if x > 0
    println!("Positive");
```

**Fix:**

```rust
// ✅ Always use braces
if x > 0 {
    println!("Positive");
}
```

### Mistake 3: Type Mismatch in Expression

```rust
let result = if x > 0 {
    "positive"
} else {
    0  // ❌ Different type
};
```

**Fix:** Both branches must return the same type.

---

## Summary

| Feature | Details |
|---|---|
| **Basic Syntax** | `if condition { }` (no parentheses needed) |
| **Braces** | Always required, even for single statements |
| **Type Safety** | Condition must be `bool` |
| **Expression** | `if` can return a value |
| **Type Consistency** | All branches must return same type |
| **Operators** | `==`, `!=`, `<`, `>`, `<=`, `>=`, `&&`, `||`, `!` |
| **Pattern Matching** | Use `if let` for simple cases, `match` for complex |
| **Exhaustiveness** | `match` requires all cases covered |

---

## Key Takeaways

1. **No parentheses required** around conditions (but braces are mandatory)

2. **Conditions must be boolean** — Rust won't coerce types

3. **`if` returns values** — powerful for assignments and function returns

4. **Type consistency enforced** — all branches must return same type

5. **Short-circuit evaluation** — `&&` and `||` stop early when possible

6. **`if let` for simple patterns** — cleaner than full `match`

7. **`match` for complex logic** — exhaustive, optimized, expressive

8. **Use early returns** for guard clauses and error handling

9. **Compiler optimizes well** — write clear code, trust the compiler

10. **Prefer `match` over long `if-else` chains** for readability

---

## Best Practices

✅ **Do:**
- Omit unnecessary parentheses around conditions
- Use `if` expressions to reduce code duplication
- Prefer `match` for multiple specific values
- Use `if let` for simple pattern matching
- Keep conditions simple and readable

❌ **Don't:**
- Try to use non-boolean values as conditions
- Create deeply nested `if` statements (use `match` instead)
- Forget to handle all branches when returning values
- Use long `if-else` chains when `match` is clearer

---

## Next Steps

- Study **pattern matching** in depth with `match`
- Learn about **`Option` and `Result` types** for error handling
- Explore **loops** (`loop`, `while`, `for`)
- Understand **ownership and borrowing** for more complex conditions

---

**Confidence Level:** High ✅  
**Recommended Practice:** Write small programs using different conditional patterns to build muscle memory.
