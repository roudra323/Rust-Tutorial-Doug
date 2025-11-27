// Project Ideas - Sample Implementation: Generic Stack
// This is a simple example of what you can build after learning generics

/// A generic stack data structure
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Create a new empty stack
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    /// Push an item onto the stack
    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// Pop an item from the stack
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Peek at the top item without removing it
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    /// Check if the stack is empty
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get the size of the stack
    fn size(&self) -> usize {
        self.items.len()
    }
}

// Additional trait implementations for enhanced functionality
impl<T: std::fmt::Debug> Stack<T> {
    /// Print all items in the stack
    fn print_all(&self) {
        println!("Stack (bottom to top): {:?}", self.items);
    }
}

fn main() {
    println!("=== Generic Stack Demo ===\n");

    // Stack of integers
    let mut int_stack: Stack<i32> = Stack::new();

    println!("Pushing 1, 2, 3 onto integer stack...");
    int_stack.push(1);
    int_stack.push(2);
    int_stack.push(3);

    int_stack.print_all();
    println!("Size: {}", int_stack.size());
    println!("Peek: {:?}", int_stack.peek());

    println!("\nPopping items...");
    while let Some(item) = int_stack.pop() {
        println!("Popped: {}", item);
    }

    println!("Is empty? {}\n", int_stack.is_empty());

    // Stack of strings - same code works for different types!
    println!("=== String Stack Demo ===\n");

    let mut string_stack: Stack<String> = Stack::new();

    string_stack.push(String::from("Hello"));
    string_stack.push(String::from("Rust"));
    string_stack.push(String::from("World"));

    string_stack.print_all();

    if let Some(top) = string_stack.peek() {
        println!("Top of stack: {}", top);
    }

    // Stack of custom types
    println!("\n=== Custom Type Stack Demo ===\n");

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Point {
        x: i32,
        y: i32,
    }

    let mut point_stack: Stack<Point> = Stack::new();

    point_stack.push(Point { x: 0, y: 0 });
    point_stack.push(Point { x: 10, y: 20 });
    point_stack.push(Point { x: 5, y: 15 });

    point_stack.print_all();

    println!("\nThis demonstrates the power of generics:");
    println!("- One Stack implementation");
    println!("- Works with any type: i32, String, custom structs");
    println!("- Type safety guaranteed at compile time");
}
