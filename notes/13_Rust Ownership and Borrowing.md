# 🦀 Rust Ownership and Borrowing

**Topic:** Understanding Rust's memory management system  
**Skill Level:** Beginner to Intermediate  
**Goal:** Learn how Rust manages memory safely without a garbage collector

---

## Overview

**Ownership** is Rust's most unique feature. It's how Rust manages memory safely and efficiently without needing a garbage collector.

### Why Ownership Matters

- **Memory Safety:** No null pointer errors, no dangling pointers
- **No Garbage Collector:** Predictable performance
- **Thread Safety:** Prevents data races at compile time
- **Zero Cost:** No runtime overhead

**Key Insight:** The compiler checks ownership rules at compile time, so you catch bugs before your code runs! 🎯

---

## The Three Rules of Ownership

### Rule 1: Each Value Has an Owner

```rust
fn main() {
    let s = String::from("hello");  // s is the owner of this String
}
```

- Every value in Rust has a single owner
- The owner is the variable that holds the value

### Rule 2: There Can Only Be One Owner at a Time

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // Ownership moved from s1 to s2
    
    // println!("{}", s1);  // ❌ Error! s1 no longer owns the data
    println!("{}", s2);     // ✅ Works! s2 is now the owner
}
```

### Rule 3: When the Owner Goes Out of Scope, the Value is Dropped

```rust
fn main() {
    {
        let s = String::from("hello");  // s is valid from here
        println!("{}", s);
    }  // s goes out of scope here and memory is freed
    
    // println!("{}", s);  // ❌ Error! s is no longer valid
}
```

**Memory is automatically cleaned up when the owner goes out of scope!**

---

## The Stack vs. The Heap

### Stack: Simple and Fast

```rust
fn main() {
    let x = 5;        // Stored on stack
    let y = x;        // Value copied on stack
    
    println!("{} {}", x, y);  // ✅ Both work! Simple types are copied
}
```

**Stack types** (fixed size):
- All integers (`i32`, `u64`, etc.)
- Floats (`f32`, `f64`)
- Booleans (`bool`)
- Characters (`char`)
- Tuples (if all elements are stack types)

### Heap: Flexible but Needs Management

```rust
fn main() {
    let s1 = String::from("hello");  // Stored on heap
    let s2 = s1;                      // Ownership moved, not copied
    
    // println!("{}", s1);  // ❌ Error! s1 no longer owns the data
    println!("{}", s2);     // ✅ Only s2 owns the data now
}
```

**Heap types** (dynamic size):
- `String`
- `Vec<T>`
- `Box<T>`
- Any type with dynamic size

---

## Move Semantics

### What is a Move?

When you assign a heap-allocated value to another variable, ownership **moves**:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 moved to s2
    
    // s1 is no longer valid here
    println!("{}", s2);  // Only s2 can be used
}
```

**Visual representation:**
```
Before move:
s1 → [ptr] → "hello" on heap

After move:
s1 → [invalid]
s2 → [ptr] → "hello" on heap
```

### Moving into Functions

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);  // s moved into function
    
    // println!("{}", s);  // ❌ Error! s is no longer valid
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}  // some_string goes out of scope and is dropped
```

### Moving Out of Functions

```rust
fn main() {
    let s = gives_ownership();  // Function gives ownership to s
    println!("{}", s);           // ✅ s is valid here
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string  // Ownership moved to caller
}
```

---

## Clone: Making Deep Copies

If you want to actually **copy** heap data (not just move ownership), use `.clone()`:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();  // Deep copy created
    
    println!("{} {}", s1, s2);  // ✅ Both work! We have two separate copies
}
```

**Warning:** `.clone()` can be expensive! Only use it when necessary.

---

## Copy Trait: Automatic Copying

Some types implement the `Copy` trait and are automatically copied instead of moved:

```rust
fn main() {
    let x = 5;
    let y = x;  // x is copied, not moved
    
    println!("{} {}", x, y);  // ✅ Both work!
}
```

**Types with `Copy` trait:**
- All integer types
- Boolean (`bool`)
- Floating point types
- Character (`char`)
- Tuples (if all elements implement `Copy`)

**Types without `Copy` trait:**
- `String`
- `Vec<T>`
- Any type that manages heap memory

---

## Borrowing: References Without Ownership

Instead of transferring ownership, you can **borrow** a value using references:

### Immutable Borrowing (`&T`)

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // Borrow s1 (no ownership transfer)
    
    println!("'{}' has length {}", s1, len);  // ✅ s1 still valid!
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope, but doesn't drop data (it doesn't own it)
```

**Think of `&` as "let me look at it, but I won't take it"**

### Mutable Borrowing (`&mut T`)

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);  // Borrow s mutably
    
    println!("{}", s);  // ✅ s is now "hello, world"
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

**Think of `&mut` as "let me modify it temporarily"**

---

## The Rules of Borrowing

### Rule 1: You Can Have Many Immutable References OR One Mutable Reference

```rust
fn main() {
    let mut s = String::from("hello");
    
    // ✅ Multiple immutable borrows are fine
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    
    // ✅ One mutable borrow is fine (after immutable borrows are done)
    let r3 = &mut s;
    r3.push_str(" world");
}
```

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;        // ✅ Immutable borrow
    let r2 = &mut s;    // ❌ Error! Can't have mutable borrow while immutable exists
    
    println!("{}", r1);
}
```

### Rule 2: References Must Always Be Valid

```rust
fn main() {
    let reference_to_nothing = dangle();  // ❌ Error!
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s  // ❌ Error! s is dropped, but we're returning a reference to it
}
```

**Fix:** Return the owned value instead:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // ✅ Ownership moved to caller
}
```

---

## Practical Examples

### Example 1: Reading Without Transferring Ownership

```rust
fn main() {
    let book = String::from("The Rust Book");
    
    // I want to read it, not take it
    print_book(&book);
    
    // I can still use it!
    println!("I still have: {}", book);
}

fn print_book(title: &String) {
    println!("Reading: {}", title);
}
```

### Example 2: Modifying Without Taking Ownership

```rust
fn main() {
    let mut score = 0;
    
    // Let the function modify it
    increase_score(&mut score);
    
    println!("Final score: {}", score);  // 10
}

fn increase_score(score: &mut i32) {
    *score += 10;  // Dereference to modify
}
```

### Example 3: Multiple Readers, No Writers

```rust
fn main() {
    let data = String::from("shared data");
    
    // Many readers at once is fine
    let reader1 = &data;
    let reader2 = &data;
    let reader3 = &data;
    
    println!("{} {} {}", reader1, reader2, reader3);
}
```

### Example 4: One Writer, No Readers

```rust
fn main() {
    let mut data = String::from("changeable");
    
    {
        let writer = &mut data;
        writer.push_str(" data");
        // No readers allowed here!
    }  // writer goes out of scope
    
    // Now we can read again
    println!("{}", data);
}
```

---

## Common Patterns

### Pattern 1: Temporary Borrow

```rust
fn main() {
    let mut list = vec![1, 2, 3];
    
    // Borrow temporarily
    print_list(&list);
    
    // Modify after borrowing
    list.push(4);
}

fn print_list(list: &Vec<i32>) {
    for item in list {
        println!("{}", item);
    }
}
```

### Pattern 2: Method Chaining with `&mut self`

```rust
struct Counter {
    count: i32,
}

impl Counter {
    fn increment(&mut self) -> &mut Self {
        self.count += 1;
        self  // Return mutable reference for chaining
    }
    
    fn reset(&mut self) -> &mut Self {
        self.count = 0;
        self
    }
}

fn main() {
    let mut counter = Counter { count: 0 };
    counter.increment().increment().reset();
}
```

### Pattern 3: Returning References from Methods

```rust
struct Person {
    name: String,
}

impl Person {
    fn name(&self) -> &str {
        &self.name  // Return reference to internal data
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
    };
    
    println!("Name: {}", person.name());
}
```

---

## Common Mistakes and Fixes

### Mistake 1: Using After Move

```rust
// ❌ Wrong
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    println!("{}", s);  // Error! s was moved
}

fn takes_ownership(s: String) {
    println!("{}", s);
}
```

```rust
// ✅ Fix: Use a reference
fn main() {
    let s = String::from("hello");
    uses_reference(&s);
    println!("{}", s);  // Works! s is still valid
}

fn uses_reference(s: &String) {
    println!("{}", s);
}
```

### Mistake 2: Multiple Mutable Borrows

```rust
// ❌ Wrong
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;  // Error! Can't have two mutable borrows
    println!("{} {}", r1, r2);
}
```

```rust
// ✅ Fix: Use scopes or use one at a time
fn main() {
    let mut s = String::from("hello");
    
    {
        let r1 = &mut s;
        r1.push_str(" world");
    }  // r1 goes out of scope
    
    let r2 = &mut s;  // Now this works!
    r2.push_str("!");
}
```

### Mistake 3: Mixing Mutable and Immutable Borrows

```rust
// ❌ Wrong
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;      // Immutable borrow
    let r2 = &mut s;  // Error! Can't borrow mutably while immutable borrow exists
    println!("{} {}", r1, r2);
}
```

```rust
// ✅ Fix: Finish with immutable borrows first
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    println!("{}", r1);  // Last use of r1
    
    let r2 = &mut s;  // Now this works!
    r2.push_str(" world");
}
```

---

## Mental Models

### Think of Ownership Like a Book

- **Ownership:** You own the book. You can read it, modify it, give it away.
- **Immutable borrow (`&`):** Someone borrows your book to read. You can't modify or give it away while they're reading.
- **Mutable borrow (`&mut`):** Someone borrows your book to write notes. No one else can read or write while they have it.

### Think of References Like Pointers (But Safer)

```rust
fn main() {
    let x = 5;
    let r = &x;  // r points to x
    
    println!("x = {}", x);   // Read through x
    println!("r = {}", *r);  // Read through reference (dereference with *)
}
```

---

## Summary Table

| Concept | Symbol | Meaning | Example |
|---------|--------|---------|---------|
| **Ownership** | — | One owner per value | `let s = String::from("hi");` |
| **Move** | — | Transfer ownership | `let s2 = s1;` |
| **Clone** | `.clone()` | Deep copy | `let s2 = s1.clone();` |
| **Copy** | — | Automatic copy (stack types) | `let y = x;` (for `i32`) |
| **Immutable borrow** | `&T` | Read-only reference | `fn read(s: &String)` |
| **Mutable borrow** | `&mut T` | Read-write reference | `fn write(s: &mut String)` |
| **Dereference** | `*` | Access value through reference | `*r = 10;` |

---

## Key Takeaways

✅ **Every value has exactly one owner**  
✅ **Ownership can be moved or borrowed**  
✅ **Borrowing doesn't transfer ownership**  
✅ **You can have many readers OR one writer, not both**  
✅ **References must always be valid**  
✅ **The compiler enforces these rules at compile time**  

**The Big Win:** No null pointers, no dangling pointers, no data races, no memory leaks — all checked at compile time! 🚀

---

## What's Next?

- **Lifetimes:** Understanding how long references are valid
- **Smart Pointers:** `Box<T>`, `Rc<T>`, `RefCell<T>`
- **Concurrency:** Safe multithreading with ownership
- **Interior Mutability:** `Cell<T>` and `RefCell<T>` patterns

---

## Additional Resources

- [The Rust Book - Chapter 4: Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust By Example - Ownership](https://doc.rust-lang.org/rust-by-example/scope/move.html)
- [Visualizing Memory Layout](https://www.youtube.com/watch?v=VFIOSWy93H0)

---

**Remember:** Ownership feels weird at first, but it's what makes Rust powerful. Stick with it! 🦀