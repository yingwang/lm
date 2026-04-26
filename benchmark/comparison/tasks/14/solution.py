def fmt_list(lst):
    return "[" + ", ".join(str(x) for x in lst) + "]"

def list_filter(lst, pred):
    return [x for x in lst if pred(x)]

print(fmt_list(list_filter([1, 2, 3, 4, 5], lambda x: x > 3)))
print(fmt_list(list_filter([1, 2, 3], lambda x: x % 2 == 0)))
print(fmt_list(list_filter([], lambda x: x > 0)))
