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

def safe_div(a, b):
    if b == 0:
        return Err("division by zero")
    return Ok(int(a / b))

def chain_div(a, b, c):
    r = safe_div(a, b)
    if isinstance(r, Err):
        return r
    return safe_div(r.v, c)

print(safe_div(10, 2))
print(safe_div(10, 0))
print(safe_div(7, 2))
print(chain_div(100, 5, 4))
print(chain_div(100, 0, 4))
print(chain_div(100, 5, 0))
