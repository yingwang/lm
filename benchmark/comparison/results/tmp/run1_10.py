def is_prime(n: int) -> bool:
    """
    Checks if a given integer n is a prime number.

    A prime number is a natural number greater than 1 that has no positive
    divisors other than 1 and itself.
    0 and 1 are not considered prime numbers.
    """
    if n <= 1:
        return False
    if n == 2:
        return True
    if n % 2 == 0:  # All other even numbers are not prime
        return False

    # Check for odd divisors from 3 up to the square root of n
    # We only need to check up to sqrt(n) because if n has a divisor d > sqrt(n),
    # then it must also have a divisor n/d < sqrt(n).
    # We increment by 2 because we've already handled even numbers.
    i = 3
    while i * i <= n:
        if n % i == 0:
            return False
        i += 2

    return True

print(str(is_prime(2)).lower())
print(str(is_prime(3)).lower())
print(str(is_prime(4)).lower())
print(str(is_prime(17)).lower())
print(str(is_prime(1)).lower())
print(str(is_prime(0)).lower())