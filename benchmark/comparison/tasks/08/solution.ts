type Result<T> = { tag: "Ok"; value: T } | { tag: "Err"; message: string };

function factorial(n: number): Result<number> {
  if (n < 0) return { tag: "Err", message: "negative input" };
  let result = 1;
  for (let i = 2; i <= n; i++) {
    result *= i;
  }
  return { tag: "Ok", value: result };
}

function formatResult(r: Result<number>): string {
  if (r.tag === "Ok") return `Ok(${r.value})`;
  return `Err(${r.message})`;
}

console.log(formatResult(factorial(0)));
console.log(formatResult(factorial(5)));
console.log(formatResult(factorial(-1)));
