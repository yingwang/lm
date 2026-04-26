def fizzbuzz(n):
    if n % 15 == 0:
        return "FizzBuzz"
    elif n % 3 == 0:
        return "Fizz"
    elif n % 5 == 0:
        return "Buzz"
    else:
        return str(n)

print(fizzbuzz(1))
print(fizzbuzz(3))
print(fizzbuzz(5))
print(fizzbuzz(15))
print(fizzbuzz(7))
