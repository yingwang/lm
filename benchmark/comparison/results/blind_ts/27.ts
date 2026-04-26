function fizzbuzz(n: number): string {
  if (n % 15 === 0) return "FizzBuzz";
  if (n % 3 === 0) return "Fizz";
  if (n % 5 === 0) return "Buzz";
  return String(n);
}

console.log(fizzbuzz(1));
console.log(fizzbuzz(3));
console.log(fizzbuzz(5));
console.log(fizzbuzz(15));
console.log(fizzbuzz(7));
