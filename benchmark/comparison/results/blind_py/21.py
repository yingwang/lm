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

def map_result(r, addend):
    if isinstance(r, Ok):
        return Ok(r.v + addend)
    return r

def and_then_divide(r, divisor):
    if isinstance(r, Err):
        return r
    if divisor == 0:
        return Err("division by zero")
    return Ok(int(r.v / divisor))

print(map_result(Ok(5), 10))
print(map_result(Err("fail"), 10))
print(and_then_divide(Ok(10), 2))
print(and_then_divide(Ok(10), 0))
print(and_then_divide(Err("fail"), 2))
print(and_then_divide(map_result(Ok(20), 10), 3))
