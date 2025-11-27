# 🦀 Rust Project Ideas After Generics

**Topic:** Project ideas for applying Rust knowledge after completing generics  
**Skill Level:** Beginner to Intermediate

---

## Overview

Congratulations on completing generics! You now have a solid foundation in Rust, including:

- **Primitive types** and strings
- **Functions** and conditionals
- **Tuples** and structs
- **Impl blocks** and traits
- **Enums** with pattern matching
- **Generics** for reusable code

This document provides project ideas organized by difficulty level to help you practice and solidify your skills.

---

## Beginner Projects

### 1. Simple Calculator

**Skills Used:** Functions, match expressions, enums, error handling with `Result`

```rust
// Create an enum for operations
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// Implement a calculate function that returns Result<f64, String>
fn calculate(a: f64, b: f64, op: Operation) -> Result<f64, String> {
    match op {
        Operation::Add => Ok(a + b),
        Operation::Subtract => Ok(a - b),
        Operation::Multiply => Ok(a * b),
        Operation::Divide => {
            if b == 0.0 {
                Err(String::from("Division by zero"))
            } else {
                Ok(a / b)
            }
        }
    }
}
```

**Challenge:** Add more operations like power, modulo, and square root.

---

### 2. Temperature Converter

**Skills Used:** Structs, impl blocks, enums, traits

```rust
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
}

impl Temperature {
    fn to_celsius(&self) -> f64 {
        match self {
            Temperature::Celsius(c) => *c,
            Temperature::Fahrenheit(f) => (f - 32.0) * 5.0 / 9.0,
            Temperature::Kelvin(k) => k - 273.15,
        }
    }
    
    fn to_fahrenheit(&self) -> f64 {
        match self {
            Temperature::Celsius(c) => c * 9.0 / 5.0 + 32.0,
            Temperature::Fahrenheit(f) => *f,
            Temperature::Kelvin(k) => (k - 273.15) * 9.0 / 5.0 + 32.0,
        }
    }
}
```

**Challenge:** Implement the `Display` trait for formatted output.

---

### 3. Todo List (In-Memory)

**Skills Used:** Structs, Vec, enums, Option, impl blocks

```rust
#[derive(Debug)]
enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    status: TaskStatus,
}

struct TodoList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TodoList {
    fn new() -> Self {
        TodoList { tasks: Vec::new(), next_id: 1 }
    }
    
    fn add(&mut self, title: String) -> &Task {
        let task = Task {
            id: self.next_id,
            title,
            status: TaskStatus::Pending,
        };
        self.tasks.push(task);
        self.next_id += 1;
        self.tasks.last().unwrap()
    }
    
    fn find(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == id)
    }
}
```

**Challenge:** Add methods for updating status, removing tasks, and filtering by status.

---

### 4. Unit Converter

**Skills Used:** Generics, traits, structs

```rust
trait Convertible {
    fn to_base_unit(&self) -> f64;
    fn from_base_unit(value: f64) -> Self;
}

struct Meters(f64);
struct Feet(f64);
struct Inches(f64);

impl Convertible for Meters {
    fn to_base_unit(&self) -> f64 { self.0 }
    fn from_base_unit(value: f64) -> Self { Meters(value) }
}

impl Convertible for Feet {
    fn to_base_unit(&self) -> f64 { self.0 * 0.3048 }
    fn from_base_unit(value: f64) -> Self { Feet(value / 0.3048) }
}

fn convert<From: Convertible, To: Convertible>(from: From) -> To {
    To::from_base_unit(from.to_base_unit())
}
```

**Challenge:** Add more units (kilometers, miles, centimeters) and other measurement types (weight, volume).

---

## Intermediate Projects

### 5. Generic Stack Data Structure

**Skills Used:** Generics, Option, structs, impl blocks

```rust
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    fn size(&self) -> usize {
        self.items.len()
    }
}
```

**Challenge:** Implement a Queue, Deque, or PriorityQueue using similar patterns.

---

### 6. Simple Game: Guessing Number

**Skills Used:** Loops, pattern matching, enums, Result, std::io

```rust
use std::io;
use std::cmp::Ordering;

enum GuessResult {
    TooLow,
    TooHigh,
    Correct,
    Invalid,
}

struct Game {
    secret_number: u32,
    attempts: u32,
    max_attempts: u32,
}

impl Game {
    fn new(secret: u32, max_attempts: u32) -> Self {
        Game {
            secret_number: secret,
            attempts: 0,
            max_attempts,
        }
    }
    
    fn guess(&mut self, number: u32) -> GuessResult {
        self.attempts += 1;
        match number.cmp(&self.secret_number) {
            Ordering::Less => GuessResult::TooLow,
            Ordering::Greater => GuessResult::TooHigh,
            Ordering::Equal => GuessResult::Correct,
        }
    }
    
    fn attempts_remaining(&self) -> u32 {
        self.max_attempts - self.attempts
    }
}
```

**Challenge:** Add difficulty levels, score tracking, and hint system.

---

### 7. Library Management System

**Skills Used:** Structs, enums, traits, generics, collections

```rust
#[derive(Debug, Clone)]
enum BookStatus {
    Available,
    CheckedOut { borrower: String, due_date: String },
    Reserved { for_user: String },
}

#[derive(Debug, Clone)]
struct Book {
    isbn: String,
    title: String,
    author: String,
    status: BookStatus,
}

trait Borrowable {
    fn checkout(&mut self, borrower: &str) -> Result<(), String>;
    fn return_item(&mut self) -> Result<(), String>;
    fn is_available(&self) -> bool;
}

impl Borrowable for Book {
    fn checkout(&mut self, borrower: &str) -> Result<(), String> {
        match &self.status {
            BookStatus::Available => {
                self.status = BookStatus::CheckedOut {
                    borrower: borrower.to_string(),
                    due_date: String::from("2024-01-15"),
                };
                Ok(())
            }
            _ => Err(String::from("Book is not available")),
        }
    }
    
    fn return_item(&mut self) -> Result<(), String> {
        match &self.status {
            BookStatus::CheckedOut { .. } => {
                self.status = BookStatus::Available;
                Ok(())
            }
            _ => Err(String::from("Book was not checked out")),
        }
    }
    
    fn is_available(&self) -> bool {
        matches!(self.status, BookStatus::Available)
    }
}
```

**Challenge:** Add a Library struct to manage multiple books, user accounts, and search functionality.

---

### 8. Expression Evaluator

**Skills Used:** Enums (recursive), Box, pattern matching, generics

```rust
enum Expr {
    Number(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn evaluate(&self) -> Result<f64, String> {
        match self {
            Expr::Number(n) => Ok(*n),
            Expr::Add(left, right) => {
                Ok(left.evaluate()? + right.evaluate()?)
            }
            Expr::Sub(left, right) => {
                Ok(left.evaluate()? - right.evaluate()?)
            }
            Expr::Mul(left, right) => {
                Ok(left.evaluate()? * right.evaluate()?)
            }
            Expr::Div(left, right) => {
                let r = right.evaluate()?;
                if r == 0.0 {
                    Err(String::from("Division by zero"))
                } else {
                    Ok(left.evaluate()? / r)
                }
            }
        }
    }
}

// Example: (3 + 5) * 2
fn main() {
    let expr = Expr::Mul(
        Box::new(Expr::Add(
            Box::new(Expr::Number(3.0)),
            Box::new(Expr::Number(5.0)),
        )),
        Box::new(Expr::Number(2.0)),
    );
    
    println!("Result: {:?}", expr.evaluate()); // Ok(16.0)
}
```

**Challenge:** Add variables, functions, and a simple parser.

---

### 9. Simple Inventory System

**Skills Used:** Generics, traits, HashMap, enums

```rust
use std::collections::HashMap;

trait Storable {
    fn get_id(&self) -> String;
    fn get_name(&self) -> String;
}

struct Inventory<T: Storable> {
    items: HashMap<String, (T, u32)>, // (item, quantity)
}

impl<T: Storable> Inventory<T> {
    fn new() -> Self {
        Inventory { items: HashMap::new() }
    }
    
    fn add(&mut self, item: T, quantity: u32) {
        let id = item.get_id();
        self.items
            .entry(id)
            .and_modify(|(_, q)| *q += quantity)
            .or_insert((item, quantity));
    }
    
    fn remove(&mut self, id: &str, quantity: u32) -> Result<(), String> {
        if let Some((_, q)) = self.items.get_mut(id) {
            if *q >= quantity {
                *q -= quantity;
                if *q == 0 {
                    self.items.remove(id);
                }
                Ok(())
            } else {
                Err(String::from("Not enough items"))
            }
        } else {
            Err(String::from("Item not found"))
        }
    }
    
    fn get_quantity(&self, id: &str) -> u32 {
        self.items.get(id).map(|(_, q)| *q).unwrap_or(0)
    }
}
```

**Challenge:** Add categories, search functionality, and low-stock alerts.

---

### 10. State Machine

**Skills Used:** Enums, generics, traits, pattern matching

```rust
// Traffic Light State Machine
#[derive(Debug, Clone, Copy)]
enum LightState {
    Red,
    Yellow,
    Green,
}

struct TrafficLight {
    state: LightState,
    time_in_state: u32,
}

impl TrafficLight {
    fn new() -> Self {
        TrafficLight {
            state: LightState::Red,
            time_in_state: 0,
        }
    }
    
    fn tick(&mut self) {
        self.time_in_state += 1;
        
        let transition_time = match self.state {
            LightState::Red => 30,
            LightState::Green => 25,
            LightState::Yellow => 5,
        };
        
        if self.time_in_state >= transition_time {
            self.state = match self.state {
                LightState::Red => LightState::Green,
                LightState::Green => LightState::Yellow,
                LightState::Yellow => LightState::Red,
            };
            self.time_in_state = 0;
        }
    }
    
    fn current_state(&self) -> LightState {
        self.state
    }
}
```

**Challenge:** Create a generic state machine that works with any set of states and transitions.

---

## Advanced Projects (For Future Learning)

These projects will require learning additional Rust concepts:

### 11. File-Based Todo App
**Requires:** File I/O, serde for serialization
- Persist tasks to disk
- Load tasks on startup

### 12. Command-Line Application
**Requires:** clap or structopt crate
- Build a full CLI with subcommands
- Parse command-line arguments

### 13. Simple HTTP Server
**Requires:** std::net, threads/async
- Handle basic HTTP requests
- Serve static files

### 14. Database-Backed App
**Requires:** rusqlite or diesel crate
- CRUD operations
- Data persistence

### 15. REST API Client
**Requires:** reqwest, tokio (async)
- Make HTTP requests
- Parse JSON responses

---

## Project Selection Guide

| If you want to practice... | Try these projects |
|---------------------------|-------------------|
| Generics | Unit Converter, Stack, Inventory System |
| Enums & Pattern Matching | Calculator, Temperature Converter, Expression Evaluator |
| Traits | Unit Converter, Library System, Inventory |
| Structs & Impl | Todo List, Game, Library System |
| Error Handling | Calculator, Expression Evaluator, Inventory |
| Data Structures | Stack, Library System, State Machine |
| Problem Solving | Game, Expression Evaluator, State Machine |

---

## Tips for Learning

1. **Start small** — Pick the simplest project first
2. **Add features incrementally** — Start with basic functionality, then enhance
3. **Use `cargo doc`** — Document your code as you go
4. **Write tests** — Use `#[test]` to verify your code works
5. **Refactor often** — Clean up code as you learn better patterns
6. **Read error messages** — Rust's compiler is very helpful
7. **Don't be afraid to restart** — Sometimes starting fresh with new knowledge helps

---

## Next Steps in Learning

After completing some projects, consider learning:

1. **Lifetimes** — Memory safety without garbage collection
2. **Error handling** — `?` operator, custom error types
3. **Iterators** — `.map()`, `.filter()`, `.fold()`, etc.
4. **Collections** — HashMap, HashSet, BTreeMap
5. **Modules & Crates** — Organizing larger projects
6. **File I/O** — Reading and writing files
7. **Concurrency** — Threads, async/await
8. **Popular Crates** — serde, tokio, clap, etc.

---

**Confidence Level:** High ✅  
**Recommended Practice:** Start with 2-3 beginner projects, then move to intermediate ones as you feel comfortable.
