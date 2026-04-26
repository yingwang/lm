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

def validate_age(age):
    if age < 0:
        return Err("age must be non-negative")
    if age >= 150:
        return Err("age must be under 150")
    return Ok(age)

def validate_name(name):
    if name == "":
        return Err("name must not be empty")
    return Ok(name)

def validate_user(name, age):
    r_name = validate_name(name)
    if isinstance(r_name, Err):
        return r_name
    r_age = validate_age(age)
    if isinstance(r_age, Err):
        return r_age
    return Ok(f"{name} (age {age})")

print(validate_age(25))
print(validate_age(-1))
print(validate_age(200))
print(validate_name("Alice"))
print(validate_name(""))
print(validate_user("Alice", 25))
print(validate_user("", 25))
print(validate_user("Alice", -1))
