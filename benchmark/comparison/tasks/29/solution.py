def parse_int(s):
    try:
        return ("Ok", int(s))
    except ValueError:
        return ("Err", f"not a number: {s}")

def compute_add(r1, r2):
    if r1[0] == "Err":
        return r1
    if r2[0] == "Err":
        return r2
    return ("Ok", r1[1] + r2[1])

def describe_result(result):
    if result[0] == "Ok":
        return f"Success: {result[1]}"
    return f"Error: {result[1]}"

def fmt(result):
    if result[0] == "Ok":
        return f"Ok({result[1]})"
    return f"Err({result[1]})"

print(fmt(parse_int("42")))
print(fmt(parse_int("abc")))
print(describe_result(("Ok", 7)))
print(describe_result(("Err", "bad")))
print(describe_result(compute_add(("Err", "bad"), ("Ok", 1))))
print(describe_result(compute_add(parse_int("42"), parse_int("10"))))
print(describe_result(compute_add(parse_int("42"), parse_int("abc"))))
