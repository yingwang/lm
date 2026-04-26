def fmt_list(lst):
    return "[" + ", ".join(str(x) for x in lst) + "]"

def list_reverse(lst):
    return lst[::-1]

print(fmt_list(list_reverse([1, 2, 3])))
print(fmt_list(list_reverse([])))
print(fmt_list(list_reverse([1])))
