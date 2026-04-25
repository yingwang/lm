# Task: List Contains

Write a function that checks if a list contains a given element.

## Signature

```
fn contains(lst: List<Int>, target: Int) -> Bool
```

## Behavior

- `contains([1,2,3], 2)` returns `true`
- `contains([1,2,3], 4)` returns `false`
- `contains([], 1)` returns `false`

## Notes

- Use recursion to search the list.
- LM has list literals plus `len` and `list_get` built-ins.
