# Task: List Sum

Write a function `sum` that computes the sum of all integers in a list.

## Signature

```
fn sum(lst: List<Int>) -> Int
```

## Behavior

- `sum([1, 2, 3])` returns `6`
- `sum([])` returns `0`
- `sum([10])` returns `10`
- `sum([-1, 1])` returns `0`

## Notes

- Use recursion or a fold-style pattern.
- LM has `len`, `list_get` built-in functions for list operations.
- This task requires list builtins to be registered in the type checker (not yet done).
