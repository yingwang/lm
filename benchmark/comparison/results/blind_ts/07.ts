function fib(n: number): number {
  if (n === 0) return 0;
  if (n === 1) return 1;
  let a = 0, b = 1;
  for (let i = 2; i <= n; i++) {
    const tmp = a + b;
    a = b;
    b = tmp;
  }
  return b;
}

console.log(fib(0));
console.log(fib(1));
console.log(fib(2));
console.log(fib(5));
console.log(fib(10));
