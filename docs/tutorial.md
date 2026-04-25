# Learning LM

LM is a small language. You can learn all of it here.

## Setup

```sh
# Build the compiler
cargo build --release

# Run a program
cargo run --bin lmc -- run myfile.lm

# Type check without running
cargo run --bin lmc -- check myfile.lm
```

## 1. Hello World

```lm
io fn main() -> Unit {
    print("Hello, world!")
}

let _ = main();
```

`print` is an IO function — it talks to the outside world. Any function that calls `print` must be marked `io`. The `let _ = main();` at the top level actually runs it.

## 2. Variables

```lm
let name = "Alice";
let age = 30;
let height = 1.75;
let active = true;
```

All variables are immutable. There is no way to change a variable after binding it. If you need a "new version", make a new variable:

```lm
let x = 10;
let x = x + 1;   // shadows the previous x, doesn't mutate it
```

## 3. Types

Six built-in types:

| Type | Example | Notes |
|------|---------|-------|
| `Int` | `42`, `-7` | 64-bit integer |
| `Float` | `3.14`, `-0.5` | 64-bit float |
| `Bool` | `true`, `false` | |
| `String` | `"hello"` | Supports `\n`, `\t`, `\\`, `\"` |
| `Unit` | (no literal) | Like `void`, returned by `print` |
| `List<T>` | (via builtins) | Homogeneous list |

Type annotations are optional — the compiler infers them:

```lm
let x = 42;           // inferred as Int
let y: Int = 42;      // explicit annotation, same thing
```

## 4. Functions

```lm
fn add(a: Int, b: Int) -> Int {
    a + b
}

let result = add(3, 4);   // 7
```

The last expression in a function body is the return value. No `return` keyword.

Parameter types and return type can be annotated. The compiler infers what you leave out:

```lm
fn double(x: Int) -> Int { x * 2 }    // fully annotated
fn double(x: Int) { x * 2 }           // return type inferred
```

### IO Functions

Functions that interact with the world must be marked `io`:

```lm
io fn greet(name: String) -> Unit {
    print("Hello, " ++ name ++ "!")
}
```

The compiler enforces this. A `pure` function (the default) **cannot** call an `io` function:

```lm
fn bad() -> Unit {
    print("hi")    // COMPILE ERROR: pure function calls io function
}
```

This is infectious — if you call anything `io`, your function must be `io` too.

## 5. Operators

### Arithmetic (numbers only)
```lm
1 + 2       // 3
10 - 3      // 7
4 * 5       // 20
10 / 3      // 3 (integer division)
```

### String concatenation
```lm
"hello" ++ " " ++ "world"    // "hello world"
```

`+` does NOT work on strings. This is intentional — no ambiguity about what `+` means.

### String helpers
```lm
str_len("hello")             // 5
char_at("hello", 1)          // "e"
char_code("A")               // 65
from_char_code(65)           // "A"
```

`char_at` indexes by Unicode character, not byte offset. `char_code` expects a single-character `String`.

### Comparison
```lm
3 == 3      // true
3 != 4      // true
1 < 2       // true
5 >= 5      // true
```

### Logic
```lm
true && false    // false
true || false    // true
!true            // false
```

### No implicit conversion

```lm
1 + 1.0     // COMPILE ERROR: Int vs Float
```

You must convert explicitly:

```lm
let x: Int = 42;
let s = int_to_string(x);    // "42"
```

## 6. If / Else

Everything is an expression — `if/else` returns a value:

```lm
let max = if a > b { a } else { b };
```

Both branches are **required** (since it's an expression, it must produce a value either way):

```lm
if x > 0 { "positive" }    // COMPILE ERROR: missing else branch
```

Both branches must return the same type:

```lm
if true { 42 } else { "no" }    // COMPILE ERROR: Int vs String
```

## 7. Custom Types (ADTs)

Define your own types with variants:

```lm
type Color =
    | Red
    | Green
    | Blue

type Shape =
    | Circle(Float)
    | Rect(Float, Float)
    | Triangle(Float, Float, Float)
```

Construct them by name:

```lm
let c = Circle(5.0);
let r = Rect(3.0, 4.0);
let color = Red;
```

## 8. Pattern Matching

The most important control structure in LM:

```lm
fn area(s: Shape) -> Float {
    match s {
        Circle(r) -> 3.14159 * r * r,
        Rect(w, h) -> w * h,
        Triangle(a, b, c) -> {
            let s = (a + b + c) / 2.0;
            // Heron's formula (simplified)
            s * (s - a)
        },
    }
}
```

### Rules

**Exhaustive:** You must cover every variant. Miss one and it's a compile error:

```lm
match color {
    Red -> "red",
    Green -> "green",
    // COMPILE ERROR: non-exhaustive, missing Blue
}
```

**Wildcard `_`** matches anything:

```lm
match color {
    Red -> "red",
    _ -> "not red",    // catches Green and Blue
}
```

**Literal patterns:**

```lm
match n {
    0 -> "zero",
    1 -> "one",
    _ -> "many",
}
```

## 9. Option — No More Null

There is no `null` in LM. Use `Option<T>` instead:

```lm
// Some(value) or None
let found = Some(42);
let missing = None;
```

You must handle both cases:

```lm
fn describe(opt: Option<Int>) -> String {
    match opt {
        Some(n) -> "found: " ++ int_to_string(n),
        None -> "nothing",
    }
}
```

This means you can never get a null pointer error. The compiler forces you to handle the missing case.

## 10. Result — No More Exceptions

There are no exceptions or try/catch in LM. Use `Result<T, E>`:

```lm
fn divide(a: Int, b: Int) -> Result<Int, String> {
    match b {
        0 -> Err("division by zero"),
        _ -> Ok(a / b),
    }
}
```

Callers must handle the error:

```lm
fn show_division(a: Int, b: Int) -> String {
    match divide(a, b) {
        Ok(n) -> "result: " ++ int_to_string(n),
        Err(msg) -> "error: " ++ msg,
    }
}
```

No surprises — errors are visible in the type signature and enforced by the compiler.

## 11. Recursion (No Loops)

LM has no `for` or `while`. Use recursion:

```lm
fn factorial(n: Int) -> Int {
    if n <= 1 { 1 }
    else { n * factorial(n - 1) }
}

fn countdown(n: Int) -> String {
    if n <= 0 { "go!" }
    else { int_to_string(n) ++ " " ++ countdown(n - 1) }
}
```

### FizzBuzz

```lm
fn fizzbuzz(n: Int) -> String {
    let by3 = n - (n / 3) * 3;
    let by5 = n - (n / 5) * 5;
    if by3 == 0 {
        if by5 == 0 { "FizzBuzz" }
        else { "Fizz" }
    } else {
        if by5 == 0 { "Buzz" }
        else { int_to_string(n) }
    }
}

io fn run_fizzbuzz(i: Int, max: Int) -> Unit {
    if i > max { print("") }
    else {
        let _ = print(fizzbuzz(i));
        run_fizzbuzz(i + 1, max)
    }
}

let _ = run_fizzbuzz(1, 20);
```

## 12. Blocks

A block `{ ... }` is a sequence of expressions. The last one is the value:

```lm
let result = {
    let a = 10;
    let b = 20;
    a + b           // this is the block's value: 30
};
```

## 13. Built-in Functions

| Function | Type | Effect | Description |
|----------|------|--------|-------------|
| `print(x)` | `(String) -> Unit` | io | Print to stdout |
| `read_line()` | `() -> String` | io | Read line from stdin |
| `int_to_string(n)` | `(Int) -> String` | pure | Convert int to string |
| `float_to_string(f)` | `(Float) -> String` | pure | Convert float to string |
| `string_to_int(s)` | `(String) -> Result<Int, String>` | pure | Parse string as int |
| `to_string(x)` | `(a) -> String` | pure | Convert anything to string |
| `len(list)` | `(List<a>) -> Int` | pure | List length |
| `str_len(s)` | `(String) -> Int` | pure | String length |
| `list_get(list, i)` | `(List<a>, Int) -> Option<a>` | pure | Get element by index |
| `list_push(list, x)` | `(List<a>, a) -> List<a>` | pure | Append (returns new list) |
| `list_map(list, f)` | `(List<a>, (a) -> b) -> List<b>` | io | Map function over list |

## 14. Comments

```lm
// This is a line comment.
// There are no block comments.
```

## 15. What LM Does NOT Have

| Feature | Why not | What to use instead |
|---------|---------|-------------------|
| Mutation / `mut` | Eliminates aliasing bugs | Return new values |
| `null` / `nil` | Eliminates null pointer errors | `Option<T>` |
| Exceptions / `try`/`catch` | Makes errors invisible | `Result<T, E>` |
| Loops (`for`, `while`) | Implies mutation | Recursion |
| Classes / inheritance | Complexity, ambiguity | ADTs + functions |
| Operator overloading | `+` means different things | `++` for strings |
| Implicit type conversion | Silent bugs | Explicit conversion functions |
| Macros / reflection | Code does surprising things | Write plain functions |
| Traits / interfaces | Method dispatch ambiguity | Pattern matching |

## Cheat Sheet

```lm
// Bind a value
let x = 42;

// Define a function
fn f(a: Int, b: Int) -> Int { a + b }

// IO function
io fn say(msg: String) -> Unit { print(msg) }

// Define a type
type T = | A(Int) | B(String) | C

// Match
match value {
    A(n) -> n + 1,
    B(s) -> 0,
    C -> -1,
}

// If/else
if cond { then_expr } else { else_expr }

// Block
{ let a = 1; let b = 2; a + b }

// String concat
"hello" ++ " " ++ "world"

// Option
Some(42)    None

// Result
Ok(42)      Err("oops")
```

That's the whole language.
