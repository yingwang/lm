# Task: Integer Exponentiation

Write a function `power` that computes base raised to the exp power (base^exp) using recursion.

## Signature

```
fn power(base: Int, exp: Int) -> Int
```

## Behavior

- `power(2, 0)` returns `1`
- `power(2, 10)` returns `1024`
- `power(3, 4)` returns `81`
- `power(5, 3)` returns `125`
- `power(1, 100)` returns `1`

## Notes

- Assume exp >= 0.
- Any number to the power of 0 is 1.
- Use recursion since LM has no loops.
