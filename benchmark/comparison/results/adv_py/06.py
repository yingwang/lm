def running_sum(lst):
    result = []
    total = 0
    for x in lst:
        total += x
        result.append(total)
    return result

def fmt(lst):
    return "[" + ", ".join(str(x) for x in lst) + "]"

for test in [[1, 2, 3, 4], [], [5], [-1, 2, 3]]:
    print(fmt(running_sum(test)))
