class Ok:
    def __init__(self, v):
        self.v = v
    def __repr__(self):
        return f"Ok({self.v})"

class Err:
    def __init__(self, msg):
        self.msg = msg
    def __repr__(self):
        return f"Err({self.msg})"

def parse_int(s):
    try:
        return Ok(int(s))
    except ValueError:
        return Err(f"not a number: {s}")

def compute(a, b):
    if isinstance(a, Err):
        return a
    if isinstance(b, Err):
        return b
    return Ok(a.v + b.v)

def describe_result(r):
    if isinstance(r, Ok):
        return f"Success: {r.v}"
    return f"Error: {r.msg}"

print(parse_int("42"))
print(parse_int("abc"))
print(describe_result(compute(Ok(3), Ok(4))))
print(describe_result(compute(Err("bad"), Ok(4))))
print(describe_result(compute(Ok(3), Err("bad"))))
print(describe_result(compute(parse_int("42"), parse_int("10"))))
print(describe_result(compute(parse_int("42"), parse_int("abc"))))
