def caesar_encrypt(text, shift):
    result = []
    for c in text:
        if 'A' <= c <= 'Z':
            result.append(chr((ord(c) - ord('A') + shift) % 26 + ord('A')))
        else:
            result.append(c)
    return ''.join(result)

print(caesar_encrypt("ABC", 1))
print(caesar_encrypt("XYZ", 3))
print(caesar_encrypt("HELLO", 13))
print(caesar_encrypt("A B!", 2))
