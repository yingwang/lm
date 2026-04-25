# Task: Collatz Sequence

Write a function `collatz_steps` that counts the number of steps in the Collatz sequence to reach 1.

## Signature

```
fn collatz_steps(n: Int) -> Int
```

## Behavior

The Collatz sequence: if n is even, next = n/2; if n is odd, next = 3n+1. Count steps until reaching 1.

- `collatz_steps(1)` returns `0`
- `collatz_steps(2)` returns `1`
- `collatz_steps(6)` returns `8`
- `collatz_steps(10)` returns `6`

## Notes

- LM has no modulo operator; use `n - (n / 2) * 2` to check if n is even.
- Use recursion since LM has no loops.
