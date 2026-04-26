def fmt_list(lst):
    return "[" + ", ".join(str(x) for x in lst) + "]"

def list_flatten(lst):
    result = []
    for sub in lst:
        result.extend(sub)
    return result

print(fmt_list(list_flatten([[1, 2], [3], [4, 5]])))
print(fmt_list(list_flatten([[1]])))
print(fmt_list(list_flatten([])))
