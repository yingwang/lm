# Task: IO Greeting

Write an IO function that prints a formatted multi-line greeting.

## Signatures

```
fn format_greeting(name: String, age: Int) -> String  // pure
io fn print_greeting(name: String, age: Int) -> Unit   // io
```

## Behavior

`format_greeting("Alice", 30)`:
- Returns the string `"Hello, Alice! You are 30 years old."`

`print_greeting("Alice", 30)` prints:
```
=== Greeting ===
Hello, Alice! You are 30 years old.
================
```

## Notes

- The pure function does formatting, the IO function handles output.
- This demonstrates LM's separation of pure logic from IO effects.
