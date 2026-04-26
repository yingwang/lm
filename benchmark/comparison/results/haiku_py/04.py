def parse_age(s):
    try:
        n = int(s)
        return f"Ok({n})"
    except ValueError:
        return "Err(not a number)"

def validate_age(n):
    if 0 <= n < 150:
        return f"Ok({n})"
    else:
        return "Err(age out of range)"

def categorize_age(n):
    if n < 13:
        return "child"
    elif n < 20:
        return "teen"
    else:
        return "adult"

def process_age(s):
    # Parse
    parsed = parse_age(s)
    if parsed.startswith("Err"):
        return parsed

    # Extract number from Ok(n)
    n = int(parsed[3:-1])

    # Validate
    validated = validate_age(n)
    if validated.startswith("Err"):
        return validated

    # Categorize
    category = categorize_age(n)
    return f"Ok({category})"

# Test
test_cases = ["10", "15", "25", "abc", "-5", "200"]
for case in test_cases:
    print(process_age(case))
