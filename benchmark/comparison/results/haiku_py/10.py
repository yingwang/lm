def validate_username(username):
    if len(username) == 0:
        return "Err(username must not be empty)"
    if len(username) > 20:
        return "Err(username must be at most 20 characters)"
    return f"Ok({username})"

def validate_email(email):
    if "@" not in email:
        return "Err(email must contain @)"
    return f"Ok({email})"

def validate_password(password):
    if len(password) < 8:
        return "Err(password must be at least 8 characters)"
    return f"Ok({password})"

def register(username, email, password):
    # Validate username
    val_username = validate_username(username)
    if val_username.startswith("Err"):
        return val_username

    # Validate email
    val_email = validate_email(email)
    if val_email.startswith("Err"):
        return val_email

    # Validate password
    val_password = validate_password(password)
    if val_password.startswith("Err"):
        return val_password

    # All good
    return f"Ok({username} registered with {email})"

# Test
test_cases = [
    ("alice", "alice@example.com", "password123"),
    ("", "a@b", "pass1234"),
    ("alice", "noatsign", "pass1234"),
    ("alice", "a@b", "short"),
    ("aaaaabbbbbcccccddddde", "a@b", "pass1234"),
]

for username, email, password in test_cases:
    print(register(username, email, password))
