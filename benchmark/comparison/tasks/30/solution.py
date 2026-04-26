def validate_age(age):
    if age < 0:
        return ("Err", "age must be non-negative")
    if age >= 150:
        return ("Err", "age must be under 150")
    return ("Ok", age)

def validate_name(name):
    if name == "":
        return ("Err", "name must not be empty")
    return ("Ok", name)

def validate_user(name, age):
    name_r = validate_name(name)
    if name_r[0] == "Err":
        return name_r
    age_r = validate_age(age)
    if age_r[0] == "Err":
        return age_r
    return ("Ok", f"{name_r[1]} (age {age_r[1]})")

def fmt(result):
    if result[0] == "Ok":
        return f"Ok({result[1]})"
    return f"Err({result[1]})"

print(fmt(validate_age(25)))
print(fmt(validate_age(-1)))
print(fmt(validate_age(150)))
print(fmt(validate_name("Alice")))
print(fmt(validate_name("")))
print(fmt(validate_user("Alice", 25)))
print(fmt(validate_user("", 25)))
print(fmt(validate_user("Bob", -1)))
