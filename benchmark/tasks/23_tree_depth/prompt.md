# Task: Tree Depth

Define a binary tree ADT and write a function to calculate its depth.

## Types

```
type Tree =
    | Leaf(Int)
    | Node(Tree, Tree)
```

## Signature

```
fn depth(t: Tree) -> Int
```

## Behavior

- `depth(Leaf(1))` returns `1`
- `depth(Node(Leaf(1), Leaf(2)))` returns `2`
- `depth(Node(Node(Leaf(1), Leaf(2)), Leaf(3)))` returns `3`

## Notes

- The depth of a leaf is 1. The depth of a node is 1 + max depth of children.
- This task requires recursive types in the type checker, which is not yet supported.
