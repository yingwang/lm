# Task: Result Chain

Write functions to chain Result-returning operations, implementing `map_result` and `and_then` patterns.

## Signatures

```
fn map_result(r: Result<Int, String>, f_ok: Int) -> Result<Int, String>
fn and_then_divide(r: Result<Int, String>, divisor: Int) -> Result<Int, String>
```

## Behavior

`map_result` adds a value to the Ok case:
- `map_result(Ok(5), 10)` returns `Ok(15)`
- `map_result(Err("fail"), 10)` returns `Err("fail")`

`and_then_divide` divides the Ok value by divisor, returning Err on division by zero:
- `and_then_divide(Ok(10), 2)` returns `Ok(5)`
- `and_then_divide(Ok(10), 0)` returns `Err("division by zero")`
- `and_then_divide(Err("fail"), 2)` returns `Err("fail")`

## Notes

- Use pattern matching on Result.
- These demonstrate monadic chaining patterns.
