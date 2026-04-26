def validate_username(s):
    if not s:
        return ("Err", "username must not be empty")
    if len(s) > 20:
        return ("Err", "username must be at most 20 characters")
    return ("Ok", s)

def validate_email(s):
    if "@" not in s:
        return ("Err", "email must contain @")
    return ("Ok", s)

def validate_password(s):
    if len(s) < 8:
        return ("Err", "password must be at least 8 characters")
    return ("Ok", s)

def register(u, e, p):
    r = validate_username(u)
    if r[0] == "Err":
        return f"Err({r[1]})"
    r = validate_email(e)
    if r[0] == "Err":
        return f"Err({r[1]})"
    r = validate_password(p)
    if r[0] == "Err":
        return f"Err({r[1]})"
    return f"Ok({u} registered with {e})"

tests = [
    ("alice", "alice@example.com", "password123"),
    ("", "a@b", "pass1234"),
    ("alice", "noatsign", "pass1234"),
    ("alice", "a@b", "short"),
    ("aaaaabbbbbcccccddddde", "a@b", "pass1234"),
]

for u, e, p in tests:
    print(register(u, e, p))
