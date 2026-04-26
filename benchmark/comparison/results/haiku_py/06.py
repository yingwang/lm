def running_sum(lst):
    result = []
    total = 0
    for num in lst:
        total += num
        result.append(total)
    return result

# Test
print(running_sum([1, 2, 3, 4]))
print(running_sum([]))
print(running_sum([5]))
print(running_sum([-1, 2, 3]))
