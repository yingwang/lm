class Some:
    def __init__(self, v):
        self.v = v

class NoneOpt:
    pass

def max2(a, b):
    return a if a >= b else b

def max_opt(a, b):
    if isinstance(a, Some) and isinstance(b, Some):
        return Some(max2(a.v, b.v))
    if isinstance(a, Some):
        return a
    if isinstance(b, Some):
        return b
    return NoneOpt()

def fmt_opt(o):
    if isinstance(o, Some):
        return f"Some({o.v})"
    return "None"

print(max2(3, 7))
print(max2(10, 2))
print(max2(5, 5))
print(fmt_opt(max_opt(Some(3), Some(7))))
print(fmt_opt(max_opt(Some(5), NoneOpt())))
print(fmt_opt(max_opt(NoneOpt(), Some(3))))
print(fmt_opt(max_opt(NoneOpt(), NoneOpt())))
