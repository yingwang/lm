function power(base: number, exp: number): number {
  // According to mathematical definition, any non-zero number raised to the power of 0 is 1.
  // For 0^0, it's often defined as 1 in combinatorics and computer science contexts,
  // especially when dealing with non-negative integer exponents.
  if (exp === 0) {
    return 1;
  }

  let result = 1;
  // Multiply 'base' by itself 'exp' times.
  for (let i = 0; i < exp; i++) {
    result *= base;
  }

  return result;
}

console.log(power(2, 0));
console.log(power(2, 10));
console.log(power(3, 4));
console.log(power(5, 3));
console.log(power(1, 100));