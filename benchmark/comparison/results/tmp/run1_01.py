def greet(name):
    """
    Takes a name string and returns a greeting string.

    Args:
        name (str): The name to greet.

    Returns:
        str: A greeting in the format "Hello, {name}!".
    """
    return f"Hello, {name}!"

print(greet("World"))
print(greet("LM"))
print(greet(""))