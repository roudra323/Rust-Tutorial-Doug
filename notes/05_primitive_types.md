# 🦀 Rust Primitives: Data Types & Variable Management

**Speaker:** Doug Milford  
**Channel:** Lambda Valley  
**Topic:** Primitive types in Rust and how variables are managed

---

## Overview

Rust has primitive types similar to other languages, but **variable management is fundamentally different**. Understanding Rust's approach is critical before diving deeper.

---

## Key Principle: Immutability by Default

### Mutable vs. Immutable

- **Immutable (default):** Variables cannot be changed once assigned.
- **Mutable:** Must explicitly use the `mut` keyword.

```rust
let x = 5;           // immutable — cannot change
let mut y = 5;       // mutable — can be modified
```

**Why immutability?** It's a safety feature that prevents accidental modifications and becomes more critical as complexity grows.

---

## 1. Boolean (`bool`)

The simplest primitive type.

```rust
let is_active: bool = true;
let is_ready = false;  // type inferred
```

- Holds only `true` or `false`.
- Type is auto-inferred if you don't specify it.
- Convention: Use **snake_case** for variable names (all lowercase, words separated by `_`).

---

## 2. Integers (`i` and `u` types)

Represent whole numbers, positive and negative.

### Signed Integers (`i` = signed integer)

Split the available bits into positive and negative ranges.

| Type | Bits | Range | Use Case |
|---|---|---|---|
| `i8` | 8 | -128 to 127 | Constrained data |
| `i16` | 16 | -32,768 to 32,767 | Medium ranges |
| `i32` | 32 | ±2 billion (approx.) | **Default if not specified** |
| `i64` | 64 | ±9 quintillion (approx.) | Large values, timestamps |
| `i128` | 128 | Extremely large (12 commas!) | Very large numbers |
| `isize` | Arch-dependent | Depends on CPU (32 or 64-bit) | Pointer-related |

**Example:**

```rust
let age: i8 = 25;
let population: i64 = 8_000_000_000;
let default_int = 42;  // assumes i32
```

**Access min/max constants:**

```rust
i8::MIN   // -128
i8::MAX   // 127
```

### Unsigned Integers (`u` = unsigned integer)

Use all bits for positive values only (0 to max).

| Type | Bits | Range | Use Case |
|---|---|---|---|
| `u8` | 8 | 0 to 255 | **Color values (RGB)** |
| `u16` | 16 | 0 to 65,535 | Medium ranges |
| `u32` | 32 | 0 to ~4 billion | Larger ranges |
| `u64` | 64 | 0 to ~18 quintillion | Very large positive values |
| `u128` | 128 | Extremely large | Massive positive numbers |
| `usize` | Arch-dependent | Depends on CPU | Array indexing |

**Example (color RGB):**

```rust
let red: u8 = 255;
let green: u8 = 128;
let blue: u8 = 0;
// 256^3 = 16.7 million colors possible with just 3 u8 values
```

---

## ⚠️ Integer Overflow: A Critical Issue

### Problem: Numbers Outside the Range

```rust
let x: i8 = 127;
let result = x + 1;  // Overflow!
```

### Debug Mode vs. Release Mode

| Mode | Behavior |
|---|---|
| **Debug** | Panics (crashes) when overflow detected. Safer but slower. |
| **Release** | Wraps around silently — produces incorrect data. Faster but dangerous. |

**Example:**

```rust
let mut x: i8 = 127;
x = x + 1;  // Debug: panics; Release: wraps to -128 (wrong data!)
```

### Best Practice

**Always ensure your variable type is large enough to hold all possible values.**

```rust
let bank_balance: u64 = 1_000_000_000_000;  // Use large enough type
```

---

## 3. Floating-Point Numbers (`f32` and `f64`)

Represent decimal numbers with floating-point precision.

### Types

| Type | Bits | Precision | Default |
|---|---|---|---|
| `f32` | 32 | Single precision | No |
| `f64` | 64 | Double precision | **Yes** |

**Important:** Floats cannot hold infinite decimal places and are not perfect representations of all numbers.

Example: π (pi) cannot be accurately stored as a float.

### Declaration

```rust
let pi: f32 = 3.14;
let e = 2.71828;      // assumes f64 (default)
let x: f64 = 3.0;     // decimal point required, even if .0
```

**Critical:** A decimal point is required. Without it, Rust treats the value as an integer, not a float.

```rust
let x: f32 = 5;    // ❌ Compile error: can't assign i32 to f32
let x: f32 = 5.0;  // ✅ OK
```

---

## 4. Character (`char`)

Represents a single Unicode character.

### Key Points

- **4 bytes** (seems large for one character, but supports Unicode).
- Supports many languages: ASCII, emojis, Chinese, etc.
- Surrounded by **single quotes** (`'`), not double quotes.
- Don't confuse with `String` (which is covered in the next video).

### Declaration

```rust
let letter: char = 'A';
let emoji: char = '😀';
let chinese: char = '中';
```

### Pronunciation

Some say "char" (like the type), others say "car" (short for character). Both are accepted.

---

## 5. Non-Primitive Types (Not Covered Here)

Rust doesn't have primitive types for these; they're handled via external crates:

### Strings (`String` and `&str`)

- **Much more complex** than `char`.
- Dedicated video coming next.
- Difference between `char` (single quote) and strings (double quotes) is subtle but critical.

### Date/Time

- No built-in primitive.
- Use crates like `chrono` or `time`.
- You have flexibility to choose what fits your needs.

### Decimals (Precise Decimal Arithmetic)

- Not a primitive type.
- Use crates like `rust_decimal` for financial calculations.
- More flexible than forcing a built-in standard.

---

## Type Inference

Rust's compiler is smart — it often figures out your data type automatically.

```rust
let x = 42;           // inferred as i32
let y = 3.14;         // inferred as f64
let is_done = true;   // inferred as bool
```

**Hover your mouse over variables in VS Code to see the inferred type.**

---

## Convention: Variable Naming

Use **snake_case** (all lowercase, words separated by underscores):

```rust
let my_variable = 5;      // ✅ Good
let myVariable = 5;       // ⚠️ Warning
let MY_VARIABLE = 5;      // ⚠️ Warning (reserved for constants)
```

Rust will warn you if you don't follow conventions, but you can suppress warnings with `#[allow(...)]` if needed. **Don't do this in production** — conventions help other developers read your code.

---

## Warning Overrides (For Learning Only)

In videos, Doug suppresses warnings to reduce clutter:

```rust
#![allow(unused_variables)]
#![allow(non_snake_case)]
```

**Don't use these in production code.** Warnings are there to guide you.

---

## Summary Table

| Type | Size | Range/Description | Default | Use Case |
|---|---|---|---|---|
| `bool` | N/A | true / false | — | Conditional logic |
| `i8` | 8 bits | -128 to 127 | — | Small signed integers |
| `i32` | 32 bits | ±2 billion | **Yes** | General-purpose integers |
| `i64` | 64 bits | ±9 quintillion | — | Large numbers, timestamps |
| `i128` | 128 bits | Extremely large | — | Huge calculations |
| `isize` | CPU arch | Varies | — | Pointer/indexing |
| `u8` | 8 bits | 0 to 255 | — | **Colors (RGB)** |
| `u32` | 32 bits | 0 to ~4B | — | Large positive |
| `u64` | 64 bits | 0 to ~18Q | — | Very large positive |
| `usize` | CPU arch | Varies | — | Array indices |
| `f32` | 32 bits | Single precision | — | Memory-constrained |
| `f64` | 64 bits | Double precision | **Yes** | General-purpose floats |
| `char` | 32 bits | Unicode chars | — | Single characters, emojis |

---

## Key Takeaways

1. **Immutability is default** — use `mut` if you need to change variables.

2. **Choose the right size** — overflow is a real issue. Be explicit about your data type.

3. **Signed vs. Unsigned** — signed (`i`) covers negatives; unsigned (`u`) covers only positives.

4. **Debug vs. Release** — overflow handling differs (crash vs. silent wrap).

5. **Floats aren't perfect** — they can't represent all decimal values accurately.

6. **Char is not String** — single quotes vs. double quotes; very different.

7. **Type inference helps** — Rust guesses correctly in most cases, but be aware of defaults.

8. **Follow conventions** — snake_case for variables, helps code readability.

9. **Non-primitives are flexible** — Strings, dates, decimals use external crates, giving you choice.

10. **Data overflow is critical** — "One of the large issues in programming" — always ensure your type is large enough.

---

## Next Steps

- Deep dive into **Strings** and **String slices** (more complex than primitives).
- Explore how Rust manages memory differently from other languages.

---

**Confidence Level:** High ✅  
**Source:** Direct transcript of Doug Milford, Lambda Valley – Rust Primitives Tutorial
****