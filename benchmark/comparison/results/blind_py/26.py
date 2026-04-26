def format_greeting(name, age):
    return f"Hello, {name}! You are {age} years old."

def print_greeting(name, age):
    print("=== Greeting ===")
    print(format_greeting(name, age))
    print("================")

print_greeting("Alice", 30)
print_greeting("Bob", 25)
