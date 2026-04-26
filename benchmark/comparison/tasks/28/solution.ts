type Result<T> = { tag: "Ok"; value: T } | { tag: "Err"; message: string };

function safe_div(a: number, b: number): Result<number> {
  if (b === 0) return { tag: "Err", message: "division by zero" };
  return { tag: "Ok", value: Math.trunc(a / b) };
}

function chain_div(a: number, b: number, c: number): Result<number> {
  const r1 = safe_div(a, b);
  if (r1.tag === "Err") return r1;
  return safe_div(r1.value, c);
}

function formatResult(r: Result<number>): string {
  if (r.tag === "Ok") return `Ok(${r.value})`;
  return `Err(${r.message})`;
}

console.log(formatResult(safe_div(10, 2)));
console.log(formatResult(safe_div(10, 0)));
console.log(formatResult(safe_div(7, 2)));

console.log(formatResult(chain_div(100, 5, 4)));
console.log(formatResult(chain_div(100, 0, 4)));
console.log(formatResult(chain_div(100, 5, 0)));
