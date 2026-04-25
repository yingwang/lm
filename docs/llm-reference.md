# LM Language Reference for LLM Agents

You are writing code in LM, a purely functional language. Follow these rules exactly.

## Syntax

```
// Variable binding (immutable, always)
let x = 42;
let name = "Alice";

// Function (pure by default)
fn add(a: Int, b: Int) -> Int {
    a + b
}

// IO function (required if calling print, read_line, or any io fn)
io fn greet(name: String) -> Unit {
    print("Hello, " ++ name ++ "!")
}

// Type definition (algebraic data type)
type Shape =
    | Circle(Float)
    | Rect(Float, Float)

// Pattern matching (must be exhaustive)
fn area(s: Shape) -> Float {
    match s {
        Circle(r) -> 3.14159 * r * r,
        Rect(w, h) -> w * h,
    }
}

// If/else (is an expression, both branches required, same type)
let max = if a > b { a } else { b };

// Block (last expression is the value)
let result = { let a = 1; let b = 2; a + b };

// Top-level execution
let _ = greet("world");
```

## Types

- `Int` — 64-bit integer: `42`, `-7`
- `Float` — 64-bit float: `3.14`, `-0.5`
- `Bool` — `true`, `false`
- `String` — `"hello"`, supports `\n \t \\ \"`
- `Unit` — like void, returned by `print`
- `Option<T>` — `Some(value)` or `None`
- `Result<T, E>` — `Ok(value)` or `Err(error)`
- `List<T>` — homogeneous list

Type annotations are optional. The compiler infers types.

## Operators

```
// Arithmetic (numbers only, Int or Float, no mixing)
+  -  *  /  %

// String concatenation (NEVER use + for strings)
++

// Comparison
==  !=  <  <=  >  >=

// Logical
&&  ||  !
```

## Critical Rules

1. **No mutation.** All bindings are immutable. No `mut`. To "update", shadow with a new `let`.
2. **No loops.** No `for`, `while`. Use recursion.
3. **No null.** Use `Option<T>` with `Some(x)` / `None`. Must `match` to extract.
4. **No exceptions.** Use `Result<T, E>` with `Ok(x)` / `Err(msg)`. Must `match` to extract.
5. **No implicit conversion.** `Int + Float` is a compile error. Use `to_float(x)` or `to_int(x)`.
6. **`+` is for numbers only.** Use `++` for string concatenation.
7. **IO is infectious.** Functions calling `print` or `read_line` must be marked `io fn`. Pure functions cannot call io functions.
8. **Pattern matching must be exhaustive.** Cover every variant or use `_` wildcard.
9. **No classes, no inheritance, no traits.** Use ADTs + functions + pattern matching.
10. **No macros, no reflection.** What you write is what runs.

## Built-in Functions

```
// IO (must be called from io fn)
print(s: String) -> Unit
read_line() -> String

// Conversion (pure)
int_to_string(n: Int) -> String
float_to_string(f: Float) -> String
string_to_int(s: String) -> Result<Int, String>
to_string(x: a) -> String

// String (pure)
str_len(s: String) -> Int
char_at(s: String, i: Int) -> String
char_code(s: String) -> Int
from_char_code(n: Int) -> String

// List (pure)
len(list: List<a>) -> Int
list_get(list: List<a>, i: Int) -> Option<a>
list_push(list: List<a>, x: a) -> List<a>
list_map(list: List<a>, f: (a) -> b) -> List<b>
```

## Common Patterns

### Recursion instead of loops

```lm
// "for i in 1..n"
io fn loop(i: Int, n: Int) -> Unit {
    if i > n { print("") }
    else {
        let _ = print(int_to_string(i));
        loop(i + 1, n)
    }
}
```

### Accumulator pattern

```lm
fn sum_to(n: Int, acc: Int) -> Int {
    if n <= 0 { acc }
    else { sum_to(n - 1, acc + n) }
}
let total = sum_to(100, 0);
```

### Error handling chain

```lm
fn parse_and_double(s: String) -> Result<Int, String> {
    match string_to_int(s) {
        Ok(n) -> Ok(n * 2),
        Err(e) -> Err(e),
    }
}
```

### Building strings with recursion

```lm
fn repeat(s: String, n: Int) -> String {
    if n <= 0 { "" }
    else { s ++ repeat(s, n - 1) }
}
```

### Entry point

```lm
io fn main() -> Unit {
    print("Hello!")
}
let _ = main();
```

No special `main` convention. Top-level expressions run in order. Use `let _ =` for io calls.

## Common Mistakes to Avoid

| Wrong | Right | Why |
|-------|-------|-----|
| `"a" + "b"` | `"a" ++ "b"` | `+` is numbers only |
| `1 + 1.0` | `1 + 1` or `1.0 + 1.0` | No implicit conversion |
| `fn f() { print("hi") }` | `io fn f() { print("hi") }` | Must mark as `io` |
| `for i in ...` | Use recursion | No loops exist |
| `return x` | Just write `x` | Last expression is return value |
| `x = x + 1` | `let x = x + 1;` | No mutation, use shadowing |
| `null` | `None` | No null, use `Option<T>` |
| `throw Error(...)` | `Err("msg")` | No exceptions, use `Result<T, E>` |
