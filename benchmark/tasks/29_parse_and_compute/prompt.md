# Task: Parse and Compute

Simulate a parse-then-compute pipeline using Result types for error handling.

## Signatures

```
fn parse_int(s: String) -> Result<Int, String>
fn compute(a: Result<Int, String>, b: Result<Int, String>) -> Result<Int, String>
fn describe_result(r: Result<Int, String>) -> String
```

## Behavior

`parse_int` simulates parsing by matching known string values:
- `parse_int("42")` returns `Ok(42)`
- `parse_int("abc")` returns `Err("not a number: abc")`

`compute` adds two parsed Results, propagating errors:
- `compute(Ok(3), Ok(4))` returns `Ok(7)`
- `compute(Err("bad"), Ok(4))` returns `Err("bad")`
- `compute(Ok(3), Err("bad"))` returns `Err("bad")`

`describe_result` formats the result for display:
- `describe_result(Ok(7))` returns `"Success: 7"`
- `describe_result(Err("bad"))` returns `"Error: bad"`

## Notes

- Since `string_to_int` is not available in the type checker, use manual pattern matching for parse simulation.
- This demonstrates chaining fallible operations.
