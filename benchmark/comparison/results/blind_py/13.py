def sum_list(lst):
    total = 0
    for x in lst:
        total += x
    return total

print(sum_list([1, 2, 3]))
print(sum_list([]))
print(sum_list([10]))
print(sum_list([-1, 1]))
