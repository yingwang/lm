# Task: Safe Division

Write a function `safe_div` that performs division returning a `Result`, and demonstrate chaining multiple divisions.

## Signatures

```
fn safe_div(a: Int, b: Int) -> Result<Int, String>
fn chain_div(a: Int, b: Int, c: Int) -> Result<Int, String>
```

## Behavior

`safe_div`:
- `safe_div(10, 2)` returns `Ok(5)`
- `safe_div(10, 0)` returns `Err("division by zero")`
- `safe_div(7, 2)` returns `Ok(3)` (integer division)

`chain_div` divides a by b, then divides the result by c:
- `chain_div(100, 5, 4)` returns `Ok(5)` (100/5=20, 20/4=5)
- `chain_div(100, 0, 4)` returns `Err("division by zero")`
- `chain_div(100, 5, 0)` returns `Err("division by zero")`

## Notes

- Use pattern matching to chain Results.
- This demonstrates error propagation without exceptions.
