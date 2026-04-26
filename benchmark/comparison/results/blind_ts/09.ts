function gcd(a: number, b: number): number {
  a = Math.abs(a);
  b = Math.abs(b);
  while (b !== 0) {
    const t = b;
    b = a % b;
    a = t;
  }
  return a;
}

console.log(gcd(12, 8));
console.log(gcd(100, 75));
console.log(gcd(7, 13));
console.log(gcd(0, 5));
console.log(gcd(10, 10));
