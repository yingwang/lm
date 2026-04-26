def collatz_steps(n):
    steps = 0
    while n != 1:
        if n % 2 == 0:
            n = n // 2
        else:
            n = 3 * n + 1
        steps += 1
    return steps

print(collatz_steps(1))
print(collatz_steps(2))
print(collatz_steps(6))
print(collatz_steps(10))
