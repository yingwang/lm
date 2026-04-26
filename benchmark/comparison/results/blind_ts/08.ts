function factorial(n: number): string {
  if (n < 0) return "Err(negative input)";
  let result = 1;
  for (let i = 2; i <= n; i++) result *= i;
  return `Ok(${result})`;
}

console.log(factorial(0));
console.log(factorial(5));
console.log(factorial(-1));
