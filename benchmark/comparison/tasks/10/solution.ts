function is_prime(n: number): boolean {
  if (n < 2) return false;
  if (n === 2) return true;
  if (n % 2 === 0) return false;
  for (let i = 3; i * i <= n; i += 2) {
    if (n % i === 0) return false;
  }
  return true;
}

console.log(is_prime(2) ? "true" : "false");
console.log(is_prime(7) ? "true" : "false");
console.log(is_prime(10) ? "true" : "false");
console.log(is_prime(97) ? "true" : "false");
console.log(is_prime(1) ? "true" : "false");
console.log(is_prime(0) ? "true" : "false");
