def safe_div(a, b):
    if b == 0:
        return ("Err", "division by zero")
    return ("Ok", int(a / b))

def chain_div(a, b, c):
    r = safe_div(a, b)
    if r[0] == "Err":
        return r
    return safe_div(r[1], c)

def fmt(result):
    if result[0] == "Ok":
        return f"Ok({result[1]})"
    return f"Err({result[1]})"

print(fmt(safe_div(10, 2)))
print(fmt(safe_div(10, 0)))
print(fmt(safe_div(10, 3)))
print(fmt(chain_div(100, 4, 5)))
print(fmt(chain_div(100, 0, 5)))
print(fmt(chain_div(100, 4, 0)))
