# Task: Validate Input

Write validation functions that check multiple fields and report errors using Result types.

## Signatures

```
fn validate_age(age: Int) -> Result<Int, String>
fn validate_name(name: String) -> Result<String, String>
fn validate_user(name: String, age: Int) -> Result<String, String>
```

## Behavior

`validate_age`:
- `validate_age(25)` returns `Ok(25)`
- `validate_age(-1)` returns `Err("age must be non-negative")`
- `validate_age(200)` returns `Err("age must be under 150")`

`validate_name`:
- `validate_name("Alice")` returns `Ok("Alice")`
- `validate_name("")` returns `Err("name must not be empty")`

`validate_user` checks both name and age, returning a formatted success or the first error:
- `validate_user("Alice", 25)` returns `Ok("Alice (age 25)")`
- `validate_user("", 25)` returns `Err("name must not be empty")`
- `validate_user("Alice", -1)` returns `Err("age must be non-negative")`

## Notes

- Validate name first, then age. Return the first error encountered.
- Use pattern matching to chain validations.
