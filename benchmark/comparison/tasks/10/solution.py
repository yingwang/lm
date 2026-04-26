def is_prime(n):
    if n < 2:
        return False
    if n < 4:
        return True
    if n % 2 == 0 or n % 3 == 0:
        return False
    i = 5
    while i * i <= n:
        if n % i == 0 or n % (i + 2) == 0:
            return False
        i += 6
    return True

def fmt(b):
    return "true" if b else "false"

print(fmt(is_prime(2)))
print(fmt(is_prime(3)))
print(fmt(is_prime(4)))
print(fmt(is_prime(97)))
print(fmt(is_prime(1)))
print(fmt(is_prime(0)))
