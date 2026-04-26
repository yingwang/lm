function gcd(a: number, b: number): number {
  a = Math.abs(a);
  b = Math.abs(b);
  while (b !== 0) {
    const t = b;
    b = a % t;
    a = t;
  }
  return a;
}

console.log(gcd(8, 12));
console.log(gcd(50, 75));
console.log(gcd(7, 13));
console.log(gcd(0, 5));
console.log(gcd(10, 10));
