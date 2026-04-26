def map_result(result, f):
    if result[0] == "Ok":
        return ("Ok", f(result[1]))
    return result

def and_then_divide(result, divisor):
    if result[0] == "Err":
        return result
    if divisor == 0:
        return ("Err", "division by zero")
    return ("Ok", result[1] // divisor)

def fmt(result):
    if result[0] == "Ok":
        return f"Ok({result[1]})"
    return f"Err({result[1]})"

# map_result tests
print(fmt(map_result(("Ok", 10), lambda x: x + 5)))
print(fmt(map_result(("Err", "fail"), lambda x: x + 5)))

# and_then_divide tests
print(fmt(and_then_divide(("Ok", 10), 2)))
print(fmt(and_then_divide(("Ok", 10), 0)))
print(fmt(and_then_divide(("Err", "fail"), 2)))

# chained
print(fmt(and_then_divide(map_result(("Ok", 20), lambda x: x + 10), 3)))
