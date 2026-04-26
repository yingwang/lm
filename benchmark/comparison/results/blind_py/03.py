def is_palindrome(s):
    return s == s[::-1]

def fmt(b):
    return "true" if b else "false"

print(fmt(is_palindrome("racecar")))
print(fmt(is_palindrome("hello")))
print(fmt(is_palindrome("")))
print(fmt(is_palindrome("a")))
