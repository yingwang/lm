def contains(lst, target):
    for x in lst:
        if x == target:
            return True
    return False

def fmt(b):
    return "true" if b else "false"

print(fmt(contains([1, 2, 3], 2)))
print(fmt(contains([1, 2, 3], 4)))
print(fmt(contains([], 1)))
