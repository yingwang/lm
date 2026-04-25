# Task: Factorial with Error Handling

Write a function `factorial` that computes the factorial of a non-negative integer, returning a `Result<Int, String>` to handle negative inputs.

## Signature

```
fn factorial(n: Int) -> Result<Int, String>
```

## Behavior

- `factorial(0)` returns `Ok(1)`
- `factorial(5)` returns `Ok(120)`
- `factorial(-1)` returns `Err("negative input")`

## Notes

- Return `Err("negative input")` for negative numbers.
- 0! = 1 by definition.
- Use recursion.
