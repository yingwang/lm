def is_palindrome(s):
    return "true" if s == s[::-1] else "false"

print(is_palindrome("racecar"))
print(is_palindrome("hello"))
print(is_palindrome(""))
print(is_palindrome("a"))
