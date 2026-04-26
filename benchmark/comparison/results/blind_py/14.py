def filter_list(lst, pred):
    return [x for x in lst if pred(x)]

def gt3(x):
    return x > 3

def is_even(x):
    return x % 2 == 0

def fmt(lst):
    return "[" + ", ".join(str(x) for x in lst) + "]"

print(fmt(filter_list([1, 2, 3, 4, 5], gt3)))
print(fmt(filter_list([1, 2, 3], is_even)))
print(fmt(filter_list([], gt3)))
