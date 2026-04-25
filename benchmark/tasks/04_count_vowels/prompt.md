# Task: Count Vowels

Write a function `count_vowels` that counts the number of vowels (a, e, i, o, u) in a string.

## Signature

```
fn count_vowels(s: String) -> Int
```

## Behavior

- `count_vowels("hello")` returns `2`
- `count_vowels("aeiou")` returns `5`
- `count_vowels("xyz")` returns `0`
- `count_vowels("")` returns `0`

## Notes

- Count both uppercase and lowercase vowels.
- Use `str_len(s)` and `char_at(s, i)` for character iteration.
