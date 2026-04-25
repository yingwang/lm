# Task: Option Unwrap Or

Write a function `unwrap_or` that extracts the value from an `Option<Int>`, returning a default value if the option is `None`.

## Signature

```
fn unwrap_or(opt: Option<Int>, default: Int) -> Int
```

## Behavior

- `unwrap_or(Some(42), 0)` returns `42`
- `unwrap_or(None, 0)` returns `0`
- `unwrap_or(Some(-1), 99)` returns `-1`
- `unwrap_or(None, -1)` returns `-1`

## Notes

- Use pattern matching on the Option type.
- `Option` is a built-in type with variants `Some(value)` and `None`.
