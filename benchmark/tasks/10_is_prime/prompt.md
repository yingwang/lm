# Task: Prime Check

Write a function `is_prime` that checks if a number is prime.

## Signature

```
fn is_prime(n: Int) -> Bool
```

## Behavior

- `is_prime(2)` returns `true`
- `is_prime(3)` returns `true`
- `is_prime(4)` returns `false`
- `is_prime(17)` returns `true`
- `is_prime(1)` returns `false`
- `is_prime(0)` returns `false`

## Notes

- A prime number is greater than 1 and has no divisors other than 1 and itself.
- Use a helper function with trial division up to sqrt(n).
- Use `%` for modulo.
- Use recursion since LM has no loops.
