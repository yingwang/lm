def find_first_even(lst):
    for num in lst:
        if num % 2 == 0:
            return num
    return -1

# Test
print(find_first_even([1, 3, 2, 5]))
print(find_first_even([1, 3, 5]))
print(find_first_even([4, 2, 6]))
