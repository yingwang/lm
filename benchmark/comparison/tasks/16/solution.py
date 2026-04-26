def list_contains(lst, val):
    return "true" if val in lst else "false"

print(list_contains([1, 2, 3], 2))
print(list_contains([1, 2, 3], 4))
print(list_contains([], 1))
