def collatz_steps(n: int) -> int:
    """
    Calculates the number of steps to reach 1 in the Collatz sequence.
    The rules are: if n is even, n = n / 2; if n is odd, n = 3n + 1.

    Args:
        n: The starting positive integer for the Collatz sequence.

    Returns:
        The number of steps required to reach 1.
        collatz_steps(1) returns 0.

    Raises:
        ValueError: If n is not a positive integer.
    """
    if not isinstance(n, int) or n <= 0:
        raise ValueError("Input n must be a positive integer for the Collatz sequence.")

    if n == 1:
        return 0

    steps = 0
    current_n = n

    while current_n != 1:
        if current_n % 2 == 0:
            current_n //= 2
        else:
            current_n = 3 * current_n + 1
        steps += 1

    return steps

print(collatz_steps(1))
print(collatz_steps(2))
print(collatz_steps(6))
print(collatz_steps(10))