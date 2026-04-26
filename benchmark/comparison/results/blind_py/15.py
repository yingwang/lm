def reverse_list(lst):
    return lst[::-1]

def fmt(lst):
    return "[" + ", ".join(str(x) for x in lst) + "]"

print(fmt(reverse_list([1, 2, 3])))
print(fmt(reverse_list([])))
print(fmt(reverse_list([1])))
