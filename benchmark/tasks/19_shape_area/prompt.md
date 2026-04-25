# Task: Shape Area

Define a `Shape` algebraic data type and write a function to calculate its area.

## Types

```
type Shape =
    | Circle(Float)
    | Rect(Float, Float)
    | Triangle(Float, Float)
```

## Signature

```
fn area(s: Shape) -> Float
```

## Behavior

- `area(Circle(5.0))` returns `78.53975` (pi * r^2, using pi = 3.14159)
- `area(Rect(3.0, 4.0))` returns `12.0`
- `area(Triangle(6.0, 4.0))` returns `12.0` (0.5 * base * height)

## Notes

- Use pattern matching on the Shape ADT.
- Use `3.14159` as an approximation of pi.
- The function should be pure.
