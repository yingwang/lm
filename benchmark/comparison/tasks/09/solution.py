def gcd(a, b):
    a, b = abs(a), abs(b)
    while b:
        a, b = b, a % b
    return a

print(gcd(12, 8))
print(gcd(100, 75))
print(gcd(7, 13))
print(gcd(0, 5))
print(gcd(10, 0))
