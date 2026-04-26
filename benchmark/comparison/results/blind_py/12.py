def power(base, exp):
    result = 1
    for _ in range(exp):
        result *= base
    return result

print(power(2, 0))
print(power(2, 10))
print(power(3, 4))
print(power(5, 3))
print(power(1, 100))
