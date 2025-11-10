# 🦀 Rust Basics: "Hello World" Program

**Speaker:** Doug Milford  
**Channel:** Lambda Valley  
**Goal:** Create and run a simple Hello World Rust project manually (without using `cargo new`)

---

## Project Setup

1. **Open Visual Studio Code**
   - Close initial windows for a clean workspace.

2. **Create a Project Folder**
   - Location doesn't matter (Desktop, "Projects" folder, etc.).
   - Example: `C:\temp\hello_world`
   - Right-click → New Folder → "hello_world"
   - Open that folder in VS Code.

---

## Project Structure

You'll manually create:

```
hello_world/
│
├── Cargo.toml
└── src/
    └── main.rs
```

---

## Step 1: Create Cargo.toml

Right-click in VS Code sidebar → New File → `Cargo.toml`

> ⚠️ **Case matters:** "C" uppercase, rest lowercase.

### Example Content

```toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Doug Milford <doug@example.com>"]
edition = "2018"
```

### Notes

- **name:** Your project's name.
- **version:** Start with `0.1.0`.
- **authors:** Optional, but good for credit.
- **edition:** Choose which compiler edition (`2018` = latest stable in video).
  - If omitted, defaults to `2015`.
  - Writing `2019` won't work — it doesn't exist.

---

## Step 2: Create Source Folder

- Right-click → New Folder → `src`

This folder is where all Rust source files live — the compiler expects it.

---

## Step 3: Create main.rs

Inside `src/`, right-click → New File → `main.rs`

> If this is your first Rust setup, VS Code might prompt:  
> "RLS (Rust Language Server) not installed."  
> ✅ Click **Install** — it will finish in a few seconds.

---

## Step 4: Write the Code

Type the following minimal Rust program:

```rust
fn main() {
    println!("Hello, world!");
}
```

### Explanation

- **`fn`** — defines a function.
- **`main`** — special function where every Rust program starts.
- **`println!`** — macro to print text to the terminal.
- **`;`** — ends the statement.

Save the file (Ctrl+S).

---

## Step 5: Build and Run

Open a terminal in VS Code:

Top menu → Terminal → New Terminal

Then run:

```bash
cargo build
```

- Compiles the project.
- Shows green messages for success.

Then execute:

```bash
cargo run
```

### Output

```
Hello, world!
```

---

## Generated Files Explained

After building, two new items appear:

### 1. Cargo.lock

- Auto-generated.
- Tracks exact dependency versions.
- Don't edit manually.

### 2. target/ folder

- Contains compiled binaries and intermediate build files.
- Leave it alone.

---

## Handling Errors

- Compiler errors show red squiggly lines in VS Code.
- Hover for details.
- `cargo build` also gives detailed error messages.

### Example Error

```
error[E0425]: cannot find function `fake_function` in this scope
```

Rust's compiler is very helpful and descriptive — it often tells you how to fix issues.

---

## Key Takeaways

| Concept | Summary |
|---|---|
| Manual Setup | Helps understand project structure before automation. |
| Cargo.toml | Defines metadata and build info for your project. |
| main.rs | Entry point containing the `main()` function. |
| Cargo Commands | `cargo build` compiles; `cargo run` builds + executes. |
| Compiler Help | Strong diagnostics; acts like a coding guide. |
| Hello World Purpose | Not useful alone — just verifies setup works. |

---

## Next Steps

Doug notes that future videos will cover:

- 3D graphics in a browser
- More advanced Rust applications
- How Rust really shines beyond basic console programs

---

## Summary

This first exercise introduces Rust's toolchain (cargo), project layout, compiler editions, and helpful diagnostics — laying a foundation for deeper development.

**Confidence Level:** High ✅  
**Source:** Direct transcript of Doug Milford, Lambda Valley – Rust Hello World Tutorial