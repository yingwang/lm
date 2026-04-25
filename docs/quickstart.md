# LM in 5 Minutes

You know how to code. Here's how LM is different.

## Setup

```sh
cargo build --release
cp target/release/lmc /usr/local/bin/
lmc run yourfile.lm
```

## The Basics

```lm
let name = "Alice";
let age = 30;
let pi = 3.14;
let active = true;
```

Everything is immutable. No `let mut`, no reassignment. Need a new value? Make a new binding:

```lm
let x = 10;
let x = x + 1;   // shadows, doesn't mutate
```

## Functions

```lm
fn add(a: Int, b: Int) -> Int {
    a + b
}
```

No `return`. Last expression is the return value. Types are inferred but annotations are encouraged.

## If You're Coming from Python

| Python | LM | Why |
|--------|-----|-----|
| `x = x + 1` | `let x = x + 1;` | No mutation, only shadowing |
| `"hi" + "there"` | `"hi" ++ "there"` | `+` is numbers only |
| `str(42)` | `int_to_string(42)` | No implicit conversion |
| `None` | `None` (with `Option<T>`) | Must handle explicitly via `match` |
| `raise Error` | `Err("msg")` (with `Result<T,E>`) | No exceptions |
| `for i in range(10)` | recursion | No loops |
| `print("hi")` | `print("hi")` | But function must be marked `io` |
| `10 % 3` | `10 % 3` | Same |

## If You're Coming from JavaScript/TypeScript

| JS/TS | LM | Why |
|-------|-----|-----|
| `let x = 1; x = 2;` | Not possible | Fully immutable |
| `"hi" + 42` | Compile error | No implicit conversion |
| `null`, `undefined` | `None` with `Option<T>` | Compiler forces you to handle it |
| `try/catch` | `Result<T, E>` + `match` | Errors in the type signature |
| `class Animal extends ...` | `type Animal = \| Dog \| Cat` | ADTs + pattern matching, no classes |
| `for/while` | recursion | No loops |
| `console.log` | `print` | Must be in an `io fn` |

## IO is Explicit

```lm
// This is pure — no side effects allowed
fn double(x: Int) -> Int { x * 2 }

// This talks to the outside world — must be marked io
io fn say(msg: String) -> Unit { print(msg) }

// COMPILE ERROR: pure function calling io function
fn bad() -> Unit { print("hi") }
```

If you call an `io` function, your function must be `io` too. The compiler enforces this.

## Custom Types (ADTs)

No classes. Define variants:

```lm
type Shape =
    | Circle(Float)
    | Rect(Float, Float)
```

Use pattern matching to work with them:

```lm
fn area(s: Shape) -> Float {
    match s {
        Circle(r) -> 3.14159 * r * r,
        Rect(w, h) -> w * h,
    }
}
```

Miss a case? Compile error.

## No Null, No Exceptions

```lm
// Instead of returning null:
fn find(id: Int) -> Option<String> {
    if id == 1 { Some("Alice") }
    else { None }
}

// Instead of throwing:
fn divide(a: Int, b: Int) -> Result<Int, String> {
    if b == 0 { Err("division by zero") }
    else { Ok(a / b) }
}

// Callers MUST handle both cases
fn show(id: Int) -> String {
    match find(id) {
        Some(name) -> "found: " ++ name,
        None -> "not found",
    }
}
```

## Recursion Instead of Loops

```lm
fn factorial(n: Int) -> Int {
    if n <= 1 { 1 }
    else { n * factorial(n - 1) }
}

io fn count(i: Int, max: Int) -> Unit {
    if i > max { print("done") }
    else {
        let _ = print(int_to_string(i));
        count(i + 1, max)
    }
}
```

## Operators

```lm
1 + 2           // arithmetic: + - * / %
"a" ++ "b"      // string concat (NOT +)
3 == 3          // comparison: == != < <= > >=
true && false   // logic: && || !
```

`+` on strings is a compile error. `Int + Float` is a compile error. No surprises.

## Built-in Functions

| Function | Does |
|----------|------|
| `print(s)` | Print string (io) |
| `int_to_string(n)` | Int to String |
| `float_to_string(f)` | Float to String |
| `to_string(x)` | Anything to String |
| `string_to_int(s)` | Parse int (returns `Result`) |
| `str_len(s)` | String length |
| `char_at(s, i)` | Character at index |
| `len(list)` | List length |
| `list_get(list, i)` | Get element (returns `Option`) |
| `list_push(list, x)` | Append (returns new list) |
| `list_map(list, f)` | Map function over list |

## Running a Program

```lm
// Define functions
fn greet(name: String) -> String {
    "Hello, " ++ name ++ "!"
}

// Top-level expressions execute immediately
let msg = greet("world");

// IO at top level needs let _ =
io fn main() -> Unit {
    print(msg)
}
let _ = main();
```

No `main()` convention. Top-level code just runs. Use `let _ =` to call io functions at the top level.

For the full language reference, see the [Tutorial](tutorial.md).
