def safe_head(lst):
    if lst:
        return ("Some", lst[0])
    else:
        return ("None",)

def safe_max(a, b):
    if a[0] == "None" and b[0] == "None":
        return ("None",)
    elif a[0] == "None":
        return b
    elif b[0] == "None":
        return a
    else:
        return ("Some", max(a[1], b[1]))

def heads_max(a, b):
    return safe_max(safe_head(a), safe_head(b))

def fmt(opt):
    if opt[0] == "None":
        return "None"
    else:
        return f"Some({opt[1]})"

for a, b in [([5, 1], [3, 2]), ([], [3]), ([3], []), ([], [])]:
    print(fmt(heads_max(a, b)))
