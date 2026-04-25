# Task: List Maximum

Find the maximum value in a collection of integers, returning `Option<Int>` to handle the empty case.

## Signature

Since list builtins aren't available in the type checker, we simulate with a variadic-style approach using nested Option:

```
fn max2(a: Int, b: Int) -> Int
fn max_opt(a: Option<Int>, b: Option<Int>) -> Option<Int>
```

## Behavior

`max2`:
- `max2(3, 7)` returns `7`
- `max2(10, 2)` returns `10`
- `max2(5, 5)` returns `5`

`max_opt`:
- `max_opt(Some(3), Some(7))` returns `Some(7)`
- `max_opt(Some(5), None)` returns `Some(5)`
- `max_opt(None, Some(3))` returns `Some(3)`
- `max_opt(None, None)` returns `None`

## Notes

- Use pattern matching on Option.
- This demonstrates working with Option as a return type for partial functions.
