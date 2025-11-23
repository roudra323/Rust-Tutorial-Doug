# 🦀 Rust Functions and Procedures

**Speaker:** Doug Milford  
**Channel:** Lambda Valley  
**Topic:** Functions vs. procedures, parameters, return types, and passing strings

---

## Overview

Functions and procedures are very similar in Rust — the distinction is subtle:

| Aspect | Functions | Procedures |
|---|---|---|
| **Parameters** | Accept parameters | Accept parameters |
| **Call others** | Can call functions/procedures | Can call functions/procedures |
| **Return value** | Returns a value | Does **not** return a value |
| **Keyword** | `fn` | `fn` (same keyword!) |

**Key Point:** The only real difference is whether or not they return a value.

---

## Creating a Function

### Basic Syntax

```rust
fn some_function(param1: f32, param2: i128) -> f32 {
    // Function body
    10.1  // No semicolon = return value
}
```

### Components Breakdown

1. **`fn` keyword** — declares a function
2. **Function name** — must be **snake_case** (all lowercase, underscores between words)
3. **Parameters** — defined in parentheses `(name: type, ...)`
4. **Return type** — indicated with arrow `-> Type`
5. **Body** — enclosed in curly braces `{ }`
6. **Return value** — last expression **without semicolon**

---

## Function Parameters

### Defining Parameters

```rust
fn example(a: f32, b: i128) -> f32 {
    // ...
}
```

- You can have **as many parameters as needed**
- Each parameter **must have an explicit type**
- Parameter names should be **snake_case**
- Even with no parameters, **parentheses are required**: `fn no_params() -> i32 { }`

### Unused Parameter Warning

```rust
fn example(a: f32, b: i128) -> f32 {
    a * 2.0  // 'b' is unused, will give warning
}
```

**Solution 1:** Prefix with underscore to tell compiler to ignore:

```rust
fn example(a: f32, _b: i128) -> f32 {
    a * 2.0  // No warning
}
```

**Solution 2:** Actually use the parameter:

```rust
fn example(a: f32, b: i128) -> f32 {
    a * (b as f32)  // Both used, no warning
}
```

---

## Return Values

### No Semicolon = Return

The compiler knows you want to return data when there's **no semicolon**:

```rust
fn get_value() -> f32 {
    10.1  // Returns 10.1 (no semicolon!)
}
```

### Hard-Coded Values

```rust
fn get_ten() -> f32 {
    10.1  // Compiler infers this is f32 based on return type
}
```

### Equations

```rust
fn calculate(a: f32, b: f32) -> f32 {
    a * b + 5.0  // No semicolon
}
```

### Local Variables

```rust
fn compute(a: f32, b: f32) -> f32 {
    let result = a * b + 5.0;
    result  // No semicolon
}
```

---

## Type Coercion and Casting

### Compiler Inference

```rust
fn example() -> f32 {
    10.1  // Compiler knows this is f32 from return type
}
```

### Returning Integers as Floats

```rust
fn example() -> f32 {
    10    // ❌ Compile error: can't return i32 as f32
}
```

**Solution 1: Using `as` keyword**

```rust
fn example() -> f32 {
    10 as f32  // ✅ Explicit cast
}
```

**Solution 2: Type suffix notation**

```rust
fn example() -> f32 {
    10f32      // ✅ Type suffix
    // or
    10_f32     // ✅ With underscore for readability
}
```

**Solution 3: Decimal point**

```rust
fn example() -> f32 {
    10.        // ✅ Dot indicates float
    // or
    10.0       // ✅ With trailing zero
}
```

---

## Mixing Types (Casting Required)

### Problem: Type Mismatch

```rust
fn calculate(a: f32, b: i128) -> f32 {
    a * b  // ❌ Compile error: can't multiply f32 and i128
}
```

### Solution: Explicit Casting

```rust
fn calculate(a: f32, b: i128) -> f32 {
    a * (b as f32)  // ✅ Cast i128 to f32
}
```

### ⚠️ Warning: Data Loss

Casting between types can cause **data loss**:

```rust
fn calculate(a: f32, b: i128) -> f32 {
    a * (b as f32)  // ⚠️ i128 can hold MUCH larger values than f32
}
```

**Doug's Warning:**
> "If I saw this casting in real code I'd be suspicious. Someone specifically picked an i128 to represent their data, meaning they expected it to hold extremely large numbers that an f32 was never meant to handle."

- `i128` can hold enormous numbers (12 commas!)
- `f32` has limited precision and range
- Casting from `i128` to `f32` loses precision for large values

**Best Practice:** Only cast when you **understand your data** and know the conversion is safe.

---

## Compile-Time Safety: Return Type Checking

### Problem: Missing Return in Some Branches

```rust
fn example(value: f32) -> f32 {
    if value < 100.0 {
        value * 2.0  // Returns if value < 100
    }
    // ❌ Compile error: what if value >= 100?
}
```

**Error:** "if statement may be missing an else clause"

### Solution: Handle All Cases

```rust
fn example(value: f32) -> f32 {
    if value < 100.0 {
        value * 2.0
    } else {
        -1.0  // Default return for other cases
    }
}
```

**Remember:** Add `.` or `.0` to indicate float, not integer!

```rust
fn example(value: f32) -> f32 {
    if value < 100.0 {
        value * 2.0
    } else {
        -1  // ❌ Compile error: expected f32, found integer
    }
}
```

**The compiler ensures all code paths return the correct type.**

---

## Creating a Procedure

### Syntax (No Return Type)

```rust
fn some_procedure(param: i32) {
    println!("Value: {}", param);
    // No return type, no return statement
}
```

### Key Differences from Functions

```rust
fn some_procedure() {
    println!("I don't return anything!");
}
```

- **No arrow (`->`)** after parameters
- **No return type**
- Compiler **doesn't expect a return value**

### Attempting to Return from Procedure

```rust
fn some_procedure() {
    10  // ❌ Compile error: procedure shouldn't return anything
}
```

---

## Function vs. Procedure: Really Just Semantics

A procedure is just a **function without a return type**.

### The `main` Function

```rust
fn main() {
    println!("Hello, world!");
}
```

Technically, `main` is a **procedure** (doesn't return anything), but people commonly call it the "main function."

**Don't argue about this.** You have bigger fish to fry! 🐟

---

## Dead Code Warnings

### Problem: Unused Functions/Procedures

```rust
fn unused_function() -> i32 {
    42
}
// ⚠️ Warning: function is never used (dead code)
```

### Solution 1: Use the Function

```rust
fn main() {
    let result = unused_function();  // ✅ No warning
}
```

### Solution 2: Allow Dead Code (Temporary)

```rust
#[allow(dead_code)]
fn future_function() -> i32 {
    42  // ✅ No warning, useful for functions you'll use later
}
```

**Recommendation:** Treat warnings as seriously as errors. Only use `#[allow(dead_code)]` temporarily.

---

## Naming Convention: snake_case

### Good

```rust
fn calculate_total() -> f32 { /* ... */ }
fn get_user_name() -> String { /* ... */ }
```

### Bad (Compiler Warning)

```rust
fn CalculateTotal() -> f32 { /* ... */ }  // ⚠️ Warning
fn getUserName() -> String { /* ... */ }   // ⚠️ Warning
```

### Override Warning (Not Recommended)

```rust
#[allow(non_snake_case)]
fn CalculateTotal() -> f32 { /* ... */ }  // ✅ No warning, but breaks convention
```

**Best Practice:** Stick with Rust conventions unless absolutely necessary.

---

## Passing Strings: The Special Topic ⚠️

### Procedure Accepting `&str`

```rust
fn print_message(message: &str) {
    println!("{}", message);
}
```

### Calling with String Literal

```rust
fn main() {
    print_message("Hello!");  // ✅ Works (string literal is &str)
}
```

### Calling with `&str` Variable

```rust
fn main() {
    let message: &str = "Hello!";
    print_message(message);  // ✅ Works
}
```

### Calling with `String` (Requires `&`)

```rust
fn main() {
    let message: String = String::from("Hello!");
    print_message(message);  // ❌ Compile error: expected &str, found String
}
```

**Solution:** Add ampersand (`&`):

```rust
fn main() {
    let message: String = String::from("Hello!");
    print_message(&message);  // ✅ Works (coercion to &str)
}
```

---

## Coercion: String to &str

### What's Happening?

```rust
let message: String = String::from("Hello!");
print_message(&message);  // & coerces String to &str
```

The ampersand (`&`) triggers **automatic coercion**:
- Rust converts the `String` reference into a `&str`
- This works **as long as Rust guarantees the data won't change** during the function call
- Related to **ownership** and **borrowing** (advanced topic)

---

## Accepting `String` Instead of `&str`

### Function Accepting `String`

```rust
fn use_string(message: String) {
    println!("{}", message);
}
```

### Calling with `String`

```rust
fn main() {
    let message: String = String::from("Hello!");
    use_string(message);  // ✅ Works
    
    // ❌ Compile error: message was moved!
    println!("{}", message);  // Can't use message again
}
```

### The Catch: Ownership Transfer

When you pass a `String` (without `&`), **ownership is transferred** to the function:

```rust
fn main() {
    let message: String = String::from("Hello!");
    use_string(message);      // Ownership moved to function
    println!("{}", message);  // ❌ Error: value used after move
}
```

### Why `&str` Doesn't Have This Problem

```rust
fn main() {
    let message: &str = "Hello!";
    print_message(message);   // No ownership transfer
    println!("{}", message);  // ✅ Works fine
}
```

String slices (`&str`) are **references**, so no ownership issues.

---

## Solution: Pass `String` by Reference

### Using `&String`

```rust
fn use_string(message: &String) {
    println!("{}", message);
}

fn main() {
    let message: String = String::from("Hello!");
    use_string(&message);     // Pass by reference
    println!("{}", message);  // ✅ Works now
}
```

### Limitation: Can't Use String Literals

```rust
fn use_string(message: &String) {
    println!("{}", message);
}

fn main() {
    use_string("Hello!");  // ❌ Error: expected &String, found &str
}
```

---

## Best Practice: Prefer `&str` Parameters

### Why `&str` Is Better

```rust
fn print_message(message: &str) {
    println!("{}", message);
}
```

**Advantages:**
1. ✅ Accepts string literals (`"Hello!"`)
2. ✅ Accepts `&str` variables
3. ✅ Accepts `String` with `&` prefix
4. ✅ More flexible than `&String`
5. ✅ No ownership issues

### When to Use `String` Parameters

```rust
fn modify_string(mut message: String) -> String {
    message.push_str(" - modified");
    message  // Return modified string
}
```

**Use `String` parameters when:**
- You need to **mutate** the string
- You need to **take ownership** of the string
- The function **consumes** the string

---

## Summary Table

| Concept | Syntax | Notes |
|---|---|---|
| **Function** | `fn name() -> Type { }` | Returns a value |
| **Procedure** | `fn name() { }` | No return value |
| **Parameters** | `fn name(a: Type, b: Type)` | Explicit types required |
| **Return** | Last expression without `;` | No `return` keyword needed |
| **Unused param** | Prefix with `_` | `_param: Type` |
| **Type cast** | `value as Type` | May lose data |
| **String slice param** | `message: &str` | Most flexible |
| **String param** | `message: String` | Transfers ownership |
| **Reference** | `&variable` | Borrow, don't transfer |
| **Coercion** | `&String` → `&str` | Automatic with `&` |

---

## Key Takeaways

1. **Functions return values, procedures don't** — both use `fn` keyword.

2. **No semicolon = return** — Rust's way of returning values.

3. **All code paths must return** — Compiler enforces this in if/else branches.

4. **Type casting is explicit** — Use `as Type` or type suffixes.

5. **Prefer `&str` parameters** — More flexible than `String` or `&String`.

6. **Ampersand (`&`) enables coercion** — Converts `String` to `&str` automatically.

7. **Ownership matters** — Passing `String` transfers ownership; can't reuse variable.

8. **Use `&` to pass by reference** — Avoids ownership transfer.

9. **Follow conventions** — snake_case for function names and parameters.

10. **Treat warnings seriously** — They identify real issues.

---

## Topics for Deeper Understanding

### Ownership and Borrowing (Critical!)

This is **the essence of understanding Rust**:
- Why `String` parameters cause "value moved" errors
- Why `&str` parameters don't have this problem
- When to use `&` vs. passing by value
- How Rust prevents memory bugs at compile time

**This video is just scratching the surface.** The ownership/borrowing video will explain everything.

---

## Confusion Is Expected

Doug acknowledges:
> "This is where I usually get squinched faces and 'huh?' confused groans..."

**Don't worry if you're confused about:**
- Why `String` can't be reused after passing
- Why `&str` is "better" than `String`
- What the ampersand (`&`) really does

**These topics are advanced.** The ownership and borrowing video will clarify everything.

---

## Final Encouragement

> "It's expected that you may have some confusion at this point. I encourage you to stick with it and grok what you can. More explanation is coming, so stick with it."

**Key Message:** Functions and procedures are straightforward, but passing strings reveals Rust's unique memory model. Understanding this unlocks Rust's power.

---

**Confidence Level:** High ✅  
**Source:** Direct transcript of Doug Milford, Lambda Valley – Rust Functions and Procedures Tutorial

**Next Steps:** Study ownership and borrowing to fully understand string parameter behavior and why Rust is different.
