def find_first_even(lst):
    for x in lst:
        if x % 2 == 0:
            return x
    return -1

print(find_first_even([1, 3, 2, 5]))
print(find_first_even([1, 3, 5]))
print(find_first_even([4, 2, 6]))
