# RCompiler

## Purpose
This the recreation of my old compiler i made in C++, i am recreating it in rust, mainly because i will need some sort of scripting language for my [shell](www.github.com/gianluur/RShell).

## Features
Should be a strongly typed language, without classes or oop, but with some sort of namespaces to properly order your code and to improve usability.

## Syntax
### 1. Data Types

| Category | Type Keywords |
| :--- | :--- |
| **Signed Integers** | `i8`, `i16`, `i32`, `i64` |
| **Unsigned Integers** | `u8`, `u16`, `u32`, `u64` |
| **Boolean** | `bool` |
| **Character** | `char` |
| **String** | `str` |

#### Arrays
Arrays are declared by specifying the base type followed by the size in square brackets:
> `<type>[<size>]`

---

### 2. Variables and Assignment
All variables must be declared with a type or preceding it a const specifier. Statements must end with a semicolon `;`.

```rust
// Declaration
const i32 my_const;
i32 my_number;
u16[5] my_array;

// Assignment
my_number = 100;
bool is_running;
is_running = true;
```

### 3. If Statements
```rust
if x > 0 {
    y = x;
}
```

### 4. While Loops
```rust
while i < 10 {
    i = i + 1;
}
```

### 5. Functions
```rust

// With return value
fn func1(i32 x, i32 y) i32 {
    return x * y;
}

// Without return value
fn func2(i32 x, i32 y) {
    return;
}

```

**This is a very limited syntax right now i will add more later on as i need it** 