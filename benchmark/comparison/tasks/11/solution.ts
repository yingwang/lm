function collatz_steps(n: number): number {
  let steps = 0;
  while (n !== 1) {
    if (n % 2 === 0) {
      n = n / 2;
    } else {
      n = 3 * n + 1;
    }
    steps++;
  }
  return steps;
}

console.log(collatz_steps(1));
console.log(collatz_steps(2));
console.log(collatz_steps(6));
console.log(collatz_steps(10));
