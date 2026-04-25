# Task: List Flatten

Write a function that flattens a list of lists into a single list.

## Signature

```
fn flatten(lst: List<List<Int>>) -> List<Int>
```

## Behavior

- `flatten([[1,2],[3],[4,5]])` returns `[1,2,3,4,5]`
- `flatten([[],[1],[]])` returns `[1]`
- `flatten([])` returns `[]`

## Notes

- Concatenate all inner lists in order.
- LM has list literals plus `len`, `list_get`, and `list_push` built-ins.
