def parse_age(s):
    try:
        return ("Ok", int(s))
    except ValueError:
        return ("Err", "not a number")

def validate_age(n):
    if 0 <= n < 150:
        return ("Ok", n)
    else:
        return ("Err", "age out of range")

def categorize_age(n):
    if n < 13:
        return "child"
    elif n < 20:
        return "teen"
    else:
        return "adult"

def process_age(s):
    result = parse_age(s)
    if result[0] == "Err":
        return f"Err({result[1]})"
    n = result[1]
    result = validate_age(n)
    if result[0] == "Err":
        return f"Err({result[1]})"
    return f"Ok({categorize_age(n)})"

for s in ["10", "15", "25", "abc", "-5", "200"]:
    print(process_age(s))
