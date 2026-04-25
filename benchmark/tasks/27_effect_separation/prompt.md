# Task: Effect Separation

Split a computation into a pure logic layer and an IO presentation layer.

## Problem

Given a number, compute whether it's a Fizz, Buzz, FizzBuzz, or just a number. Then print the results.

## Signatures

```
fn fizzbuzz(n: Int) -> String           // pure
io fn print_fizzbuzz(from: Int, to: Int) -> Unit  // io
```

## Behavior

`fizzbuzz`:
- `fizzbuzz(3)` returns `"Fizz"`
- `fizzbuzz(5)` returns `"Buzz"`
- `fizzbuzz(15)` returns `"FizzBuzz"`
- `fizzbuzz(7)` returns `"7"`

`print_fizzbuzz(1, 5)` prints:
```
1
2
Fizz
4
Buzz
```

## Notes

- Divisible by 3: "Fizz"; by 5: "Buzz"; by both: "FizzBuzz"; otherwise: the number as string.
- LM has no modulo operator; compute `a mod b` as `a - (a / b) * b`.
- Use recursion for the loop from `from` to `to`.
- The pure function does the logic; the IO function just prints.
