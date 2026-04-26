def is_prime(n):
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    i = 3
    while i * i <= n:
        if n % i == 0:
            return False
        i += 2
    return True

def fmt(b):
    return "true" if b else "false"

print(fmt(is_prime(2)))
print(fmt(is_prime(3)))
print(fmt(is_prime(4)))
print(fmt(is_prime(17)))
print(fmt(is_prime(1)))
print(fmt(is_prime(0)))
