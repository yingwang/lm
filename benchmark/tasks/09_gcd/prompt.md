# Task: Greatest Common Divisor

Write a function `gcd` that computes the greatest common divisor of two positive integers using the Euclidean algorithm.

## Signature

```
fn gcd(a: Int, b: Int) -> Int
```

## Behavior

- `gcd(12, 8)` returns `4`
- `gcd(100, 75)` returns `25`
- `gcd(7, 13)` returns `1`
- `gcd(0, 5)` returns `5`
- `gcd(10, 10)` returns `10`

## Notes

- Use the Euclidean algorithm: gcd(a, 0) = a; gcd(a, b) = gcd(b, a mod b).
- LM has no modulo operator, but `a mod b` can be computed as `a - (a / b) * b` using integer division.
- Use recursion since LM has no loops.
