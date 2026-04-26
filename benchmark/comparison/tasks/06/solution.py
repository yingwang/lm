def repeat(s, n, sep=" "):
    return sep.join([s] * n)

print(repeat("ha", 3, " "))
print(repeat("ab", 1, " "))
print(repeat("x", 0, " "))
print(repeat("hi", 2, "-"))
