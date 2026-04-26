type Result = { tag: "Ok"; value: number } | { tag: "Err"; msg: string };

function Ok(value: number): Result { return { tag: "Ok", value }; }
function Err(msg: string): Result { return { tag: "Err", msg }; }

function formatResult(r: Result): string {
  if (r.tag === "Ok") return `Ok(${r.value})`;
  return `Err(${r.msg})`;
}

function safe_div(a: number, b: number): Result {
  if (b === 0) return Err("division by zero");
  return Ok(Math.trunc(a / b));
}

function chain_div(a: number, b: number, c: number): Result {
  const r1 = safe_div(a, b);
  if (r1.tag === "Err") return r1;
  return safe_div(r1.value, c);
}

console.log(formatResult(safe_div(10, 2)));
console.log(formatResult(safe_div(10, 0)));
console.log(formatResult(safe_div(7, 2)));
console.log(formatResult(chain_div(100, 5, 4)));
console.log(formatResult(chain_div(100, 0, 4)));
console.log(formatResult(chain_div(100, 5, 0)));
