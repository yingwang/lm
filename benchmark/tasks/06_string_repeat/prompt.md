# Task: String Repeat

Write a function `repeat` that repeats a string N times with a separator between each copy.

## Signature

```
fn repeat(s: String, n: Int, sep: String) -> String
```

## Behavior

- `repeat("ha", 3, " ")` returns `"ha ha ha"`
- `repeat("ab", 1, ",")` returns `"ab"`
- `repeat("x", 0, ",")` returns `""`
- `repeat("hi", 2, "-")` returns `"hi-hi"`

## Notes

- If n <= 0, return an empty string.
- Use `++` for string concatenation.
- Use recursion since LM has no loops.
