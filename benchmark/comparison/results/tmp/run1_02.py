def reverse(s: str) -> str:
    """
    Reverses a given string.

    Args:
        s: The input string.

    Returns:
        The reversed string.
    """
    return s[::-1]

print(reverse("hello"))
print(reverse("ab"))
print(reverse(""))
print(reverse("a"))