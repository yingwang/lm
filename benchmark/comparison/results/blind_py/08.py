def factorial(n):
    if n < 0:
        return "Err(negative input)"
    result = 1
    for i in range(1, n + 1):
        result *= i
    return f"Ok({result})"

print(factorial(0))
print(factorial(5))
print(factorial(-1))
