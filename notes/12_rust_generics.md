
# 🦀 Rust Generics

**Topic:** Parametric polymorphism with generic types, traits, and lifetimes  
**Skill level:** Beginner → Intermediate

---

## Overview

Generics let you write code that works for many types while staying type-safe. Instead of duplicating functions, structs, or enums for each concrete type, you use *type parameters* to create abstractions.

Why use generics?
- Reuse: one implementation for many types
- Safety: compile-time type checks
- Expressiveness: combine with traits for constrained polymorphism
- Performance: monomorphization produces specialized code with no runtime overhead

This note covers:
- Generic functions and methods
- Generic structs, enums, and impl blocks
- Trait bounds and where clauses
- Associated types vs generic parameters
- Lifetimes + generics
- Monomorphization and performance
- Common pitfalls and best practices

---

## Basic Generic Function

```rust
// A function that returns the largest item in a slice (generic over T)
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
	let mut largest = list[0];
	for &item in list.iter() {
		if item > largest {
			largest = item;
		}
	}
	largest
}

fn main() {
	let nums = vec![34, 50, 25, 100, 65];
	let result = largest(&nums);
	println!("largest = {}", result);

	let chars = vec!['y', 'm', 'a', 'q'];
	println!("largest = {}", largest(&chars));
}
```

Notes:
- `T` is a type parameter.
- `PartialOrd` is a trait bound required for `>`.
- `Copy` is used here to avoid moving out of the slice; you could use references instead.

---

## Generic Structs and Enums

```rust
// Generic struct
struct Point<T> {
	x: T,
	y: T,
}

impl<T> Point<T> {
	fn x(&self) -> &T { &self.x }
}

// You can also mix types
struct Pair<T, U> {
	first: T,
	second: U,
}

// Generic enum: Option<T> and Result<T, E> are standard examples
enum MyOption<T> {
	Some(T),
	None,
}
```

Specialization by implementing for concrete types:

```rust
impl Point<f32> {
	fn magnitude(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}
```

---

## Methods with Generic Type Parameters

```rust
impl<T, U> Pair<T, U> {
	fn swap(self) -> Pair<U, T> {
		Pair { first: self.second, second: self.first }
	}
}
```

You can also mix type parameters on `impl` and `fn` separately.

---

## Trait Bounds

Trait bounds restrict generic parameters to types that implement particular traits.

```rust
// Accept any T that implements Display and PartialOrd
use std::fmt::Display;

fn compare_and_display<T: Display + PartialOrd>(a: T, b: T) {
	if a > b {
		println!("{} is greater", a);
	} else {
		println!("{} is not greater", a);
	}
}
```

Prefer `where` clauses for readability when bounds grow:

```rust
fn long_function_name<T, U>(t: T, u: U)
where
	T: Display + Clone,
	U: Clone + Into<T>,
{
	// body
}
```

---

## Associated Types vs Generic Parameters

Associated types let you name a type inside a trait. Use them when a trait logically includes a type.

```rust
trait Iterator {
	type Item;
	fn next(&mut self) -> Option<Self::Item>;
}

// Using associated type: Iterator<Item = i32>
```

Advantages:
- Cleaner trait bounds when multiple associated types are related
- Avoids specifying the same type parameter repeatedly

When to prefer generics: when a trait should work with multiple independent types.

---

## Generic Traits and Trait Objects

Generic traits cannot be turned into trait objects unless their type parameters are fixed or they use associated types.

```rust
// This won't work: dyn SomeTrait<T>
// Use associated types or monomorphize instead.
```

If you need runtime polymorphism, design the trait without generic parameters or use an associated type.

---

## Lifetimes and Generics Together

Generics and lifetimes are orthogonal but often used together for references:

```rust
fn longest<'a, T>(x: &'a str, y: &'a str) -> &'a str
where
	T: Into<String>, // example mixing other generics (not used directly here)
{
	if x.len() > y.len() { x } else { y }
}
```

Key points:
- Lifetime parameters (`'a`) are different from type parameters (`T`).
- Order doesn't matter: `fn foo<'a, T>(...)` or `fn foo<T, 'a>(...)` is allowed.

---

## Monomorphization and Performance

Rust implements generics via *monomorphization*: the compiler generates specialized versions of functions/structs for each concrete type used. This gives zero-cost abstraction (no dynamic dispatch unless you use trait objects).

Pros:
- Fast: specialized code optimized for each type

Cons:
- Code bloat if many different concrete types are used

Example: `fn print_all<T: Display>(items: &[T])` will be compiled separately for `T = i32`, `T = &str`, etc.

---

## Generic Constraints: Blanket Implementations and Orphan Rule

Blanket implementations allow `impl<T> Trait for Type<T> where ...`.

```rust
impl<T: Display> ToString for T {
	// This is actually in the standard library as a blanket impl
}
```

The orphan rule prevents implementing a foreign trait for a foreign type in your crate. You can only implement a trait for a type if at least one of the trait or the type is local to your crate.

---

## Default Type Parameters

Traits and types can provide default type parameters:

```rust
trait MyTrait<T = u8> {
	fn value(&self) -> T;
}
```

This allows callers to omit common type arguments.

---

## Common Patterns

- Container types: `Vec<T>`, `Option<T>`, `Result<T, E>`
- Generic algorithms: sort, search, map, fold
- Newtype pattern: wrap a generic type to add behavior
- PhantomData for zero-sized type parameters (marker usage)

Example with PhantomData:

```rust
use std::marker::PhantomData;

struct MyWrapper<T> {
	data: *const (), // pretend we store something opaque
	_marker: PhantomData<T>,
}

impl<T> MyWrapper<T> {
	fn new() -> Self { MyWrapper { data: std::ptr::null(), _marker: PhantomData } }
}
```

---

## Common Pitfalls

- Over-constraining type parameters (require fewer traits when possible).
- Using `T: Copy` unnecessarily — prefer references or Clone where appropriate.
- Assuming trait-object-compatibility for generic traits.
- Excessive monomorphization causing binary size growth.

---

## Examples: Handy Utilities

Swap example (generic function):

```rust
fn swap_values<T>(a: &mut T, b: &mut T) {
	std::mem::swap(a, b);
}
```

Generic parser example with Result:

```rust
fn parse_and_double<T>(s: &str) -> Result<T, T::Err>
where
	T: std::str::FromStr,
{
	let v: T = s.parse()?;
	Ok(v)
}
```

---

## Best Practices

- Use descriptive type parameter names when helpful (T, U, V are fine for short examples; `Item`, `Key`, `Value` are clearer in real code).
- Prefer trait bounds that express intent (use `AsRef<str>` or `Into<String>` appropriately).
- Prefer `where` clauses for readability when there are many bounds.
- Minimize public API surface area with generics; sometimes concrete wrappers simplify the public API.
- Document assumptions about type parameters (e.g., ordering, clonability).

---

## Quick Reference

- Generic function: `fn foo<T>(x: T) {}`
- Generic struct: `struct S<T> { field: T }`
- Trait bound inline: `fn f<T: Debug + Clone>(t: T)`
- Trait bound where: `fn f<T, U>(t: T, u: U) where T: Debug, U: Clone {}`
- Associated type: `trait Iterator { type Item; }`

---

## Summary

Generics are a core tool in Rust for writing reusable, type-safe, and efficient code. Learn how to combine generics with traits and lifetimes to express powerful abstractions while keeping performance predictable through monomorphization.

---

## Next Steps

- Practice: implement generic data structures (Stack, Queue, LinkedList)
- Explore: associated types and how they simplify trait bounds
- Read: Rustonomicon section on monomorphization and code size

**Confidence:** High ✅

