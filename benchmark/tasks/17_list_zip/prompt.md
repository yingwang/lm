# Task: List Zip

Write a function that zips two lists into a list of pairs.

## Signature

```
fn zip(a: List<Int>, b: List<Int>) -> List<Pair<Int, Int>>
```

Or using a Pair ADT:

```
type Pair = Pair(Int, Int)
fn zip(a: List<Int>, b: List<Int>) -> List<Pair>
```

## Behavior

- `zip([1,2,3], [4,5,6])` returns `[Pair(1,4), Pair(2,5), Pair(3,6)]`
- `zip([1,2], [3])` returns `[Pair(1,3)]` (stop at shorter list)
- `zip([], [1])` returns `[]`

## Notes

- Stop at the length of the shorter list.
- LM has list literals plus `len`, `list_get`, and `list_push` built-ins.
