# Task: Pure Computation

Demonstrate LM's effect system by writing a pure function that the type checker verifies cannot perform IO.

## Requirements

1. Write a pure function `compute` that performs arithmetic.
2. Write an `io fn` wrapper that prints the result.
3. Show that adding `print` to a pure function is caught by the type checker.

## Signatures

```
fn compute(x: Int, y: Int) -> Int    // pure
io fn show_result(x: Int) -> Unit    // io
```

## Behavior

- `compute(3, 4)` returns `25` (computes `(x + y) * (x + y)`, i.e., the square of the sum)
- `show_result(25)` prints `"Result: 25"`

## Notes

- The effect system ensures `compute` cannot call `print` or any other IO function.
- If you add `print(...)` inside `compute`, `lmc check` will report an error.
