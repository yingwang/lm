def count_vowels(s):
    return sum(1 for c in s if c.lower() in "aeiou")

print(count_vowels("hello"))
print(count_vowels("aeiou"))
print(count_vowels("xyz"))
print(count_vowels(""))
