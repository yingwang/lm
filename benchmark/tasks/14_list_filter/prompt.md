# Task: List Filter

Write a function that filters a list, keeping only elements that satisfy a predicate.

## Signature

```
fn filter(lst: List<Int>, pred: (Int) -> Bool) -> List<Int>
```

## Behavior

- Filter `[1,2,3,4,5]` keeping only elements > 3 returns `[4,5]`
- Filter `[1,2,3]` keeping only even numbers returns `[2]`
- Filter `[]` returns `[]`

## Notes

- Use recursion to build the new list.
- LM has list builtins and higher-order function support.
