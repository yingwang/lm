# Task: Caesar Cipher

Write a function `caesar_encrypt` that shifts uppercase letters by N positions in the alphabet.

## Signature

```
fn caesar_encrypt(text: String, shift: Int) -> String
```

## Behavior

- `caesar_encrypt("ABC", 1)` returns `"BCD"`
- `caesar_encrypt("XYZ", 3)` returns `"ABC"`
- `caesar_encrypt("HELLO", 13)` returns `"URYYB"`

## Notes

- Only shift uppercase A-Z letters; leave other characters unchanged.
- Wrap around: after Z comes A.
- This task requires character-level string operations not yet available in the current interpreter.
