def max2(a, b):
    return a if a > b else b

def max_opt(lst):
    if not lst:
        return "None"
    m = lst[0]
    for x in lst[1:]:
        if x > m:
            m = x
    return f"Some({m})"

print(max2(3, 7))
print(max2(10, 5))
print(max2(5, 5))
print(max_opt([3, 7, 1]))
print(max_opt([5]))
print(max_opt([1, 3, 2]))
print(max_opt([]))
