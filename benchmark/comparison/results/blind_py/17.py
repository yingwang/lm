def zip_lists(a, b):
    result = []
    for i in range(min(len(a), len(b))):
        result.append((a[i], b[i]))
    return result

def fmt(pairs):
    items = [f"Pair({a}, {b})" for a, b in pairs]
    return "[" + ", ".join(items) + "]"

print(fmt(zip_lists([1, 2, 3], [4, 5, 6])))
print(fmt(zip_lists([1, 2], [3])))
print(fmt(zip_lists([], [1])))
