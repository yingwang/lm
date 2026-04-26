function is_prime(n: number): boolean {
  if (n < 2) return false;
  for (let i = 2; i * i <= n; i++) {
    if (n % i === 0) return false;
  }
  return true;
}

console.log(String(is_prime(2)));
console.log(String(is_prime(3)));
console.log(String(is_prime(4)));
console.log(String(is_prime(17)));
console.log(String(is_prime(1)));
console.log(String(is_prime(0)));
