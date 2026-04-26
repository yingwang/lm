def flatten(lst):
    result = []
    for sub in lst:
        result.extend(sub)
    return result

def fmt(lst):
    return "[" + ", ".join(str(x) for x in lst) + "]"

print(fmt(flatten([[1, 2], [3], [4, 5]])))
print(fmt(flatten([[], [1], []])))
print(fmt(flatten([])))
