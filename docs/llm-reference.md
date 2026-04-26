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
3. **No nested functions.** All `fn` definitions must be at the top level. You cannot define a function inside another function. Use separate top-level helper functions instead.
4. **No null.** Use `Option<T>` with `Some(x)` / `None`. Must `match` to extract.
5. **No exceptions.** Use `Result<T, E>` with `Ok(x)` / `Err(msg)`. Must `match` to extract.
6. **No implicit conversion.** `Int + Float` is a compile error. Use `to_float(x)` or `to_int(x)`.
7. **`+` is for numbers only.** Use `++` for string concatenation.
8. **IO is infectious.** Functions calling `print` or `read_line` must be marked `io fn`. Pure functions cannot call io functions.
9. **Pattern matching must be exhaustive.** Cover every variant or use `_` wildcard.
10. **No classes, no inheritance, no traits.** Use ADTs + functions + pattern matching.
11. **No macros, no reflection.** What you write is what runs.

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
to_float(n: Int) -> Float               // Int to Float conversion
to_int(f: Float) -> Int                  // Float to Int (truncate toward zero)

// String (pure)
str_len(s: String) -> Int
char_at(s: String, i: Int) -> String
char_code(s: String) -> Int
from_char_code(n: Int) -> String

// List
len(list: List<a>) -> Int          // pure — only works on List, NOT String
list_get(list: List<a>, i: Int) -> Option<a>  // pure
list_push(list: List<a>, x: a) -> List<a>     // pure — appends, returns new list
list_map(list: List<a>, f: (a) -> b) -> List<b>  // ⚠️ IO — must be called from io fn
```

**Warning:** `list_map` is marked as IO. You cannot call it from a pure function. If you need to transform a list in a pure function, use index-based recursion with `list_get` and `list_push` instead.

**Warning:** `len()` only works on `List`, not `String`. Use `str_len()` for strings.

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

### Pure list transformation (since list_map is IO)

```lm
fn map_helper(lst: List<Int>, f: (Int) -> Int, i: Int, acc: List<Int>) -> List<Int> {
    if i >= len(lst) { acc }
    else {
        match list_get(lst, i) {
            Some(v) -> map_helper(lst, f, i + 1, list_push(acc, f(v))),
            None -> acc,
        }
    }
}

fn my_map(lst: List<Int>, f: (Int) -> Int) -> List<Int> {
    map_helper(lst, f, 0, [])
}
```

### Entry point

No special `main` convention. Top-level expressions run in order. Use `let _ =` for io calls directly at the top level:

```lm
let _ = print("Hello!");
let _ = print(to_string(my_function(42)));
```

**Do NOT wrap calls in `io fn main()`.** Just call functions directly with `let _ =` at the top level. Define all helper functions first (before the function that uses them), then put test calls at the bottom.

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
| `fn f() { fn helper() {...} }` | Define `helper` at top level | No nested function definitions |
| `fn f() { list_map(lst, g) }` | `io fn f() { list_map(lst, g) }` | `list_map` is IO, use recursion in pure fns |
| `len("hello")` | `str_len("hello")` | `len()` is for Lists only, use `str_len` for strings |
| `10 / 0` | Check before dividing | Division by zero is a runtime error |
