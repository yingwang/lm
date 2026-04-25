# Task: Expression Evaluator

Define a simple arithmetic expression ADT and write an evaluator.

## Types

```
type Expr =
    | Lit(Int)
    | Add(Int, Int)
    | Mul(Int, Int)
```

Note: Due to current LM limitations with recursive types, use a flat structure where Add and Mul take two Int operands directly.

## Signature

```
fn eval(e: Expr) -> Int
fn describe(e: Expr) -> String
```

## Behavior

`eval`:
- `eval(Lit(5))` returns `5`
- `eval(Add(3, 4))` returns `7`
- `eval(Mul(3, 4))` returns `12`

`describe`:
- `describe(Lit(5))` returns `"5"`
- `describe(Add(3, 4))` returns `"3 + 4"`
- `describe(Mul(3, 4))` returns `"3 * 4"`

## Notes

- Use pattern matching on the Expr ADT.
- The describe function demonstrates combining pattern matching with string operations.
