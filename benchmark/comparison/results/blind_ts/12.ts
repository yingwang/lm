function power(base: number, exp: number): number {
  let result = 1;
  for (let i = 0; i < exp; i++) result *= base;
  return result;
}

console.log(power(2, 0));
console.log(power(2, 10));
console.log(power(3, 4));
console.log(power(5, 3));
console.log(power(1, 100));
