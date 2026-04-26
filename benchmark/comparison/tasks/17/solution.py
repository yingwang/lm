def list_zip(a, b):
    pairs = [f"Pair({x}, {y})" for x, y in zip(a, b)]
    return "[" + ", ".join(pairs) + "]"

print(list_zip([1, 2, 3], [4, 5, 6]))
print(list_zip([1, 2], [3]))
print(list_zip([], []))
