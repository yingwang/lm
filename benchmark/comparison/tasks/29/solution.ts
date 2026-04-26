type Result<T> = { tag: "Ok"; value: T } | { tag: "Err"; message: string };

function parse_int(s: string): Result<number> {
  const n = Number(s);
  if (isNaN(n) || s.trim() === "") return { tag: "Err", message: `not a number: ${s}` };
  if (!Number.isInteger(n)) return { tag: "Err", message: `not a number: ${s}` };
  return { tag: "Ok", value: n };
}

function compute(a: Result<number>, b: Result<number>): Result<number> {
  if (a.tag === "Err") return a;
  if (b.tag === "Err") return b;
  return { tag: "Ok", value: a.value + b.value };
}

function describe_result(r: Result<number>): string {
  if (r.tag === "Ok") return `Success: ${r.value}`;
  return `Error: ${r.message}`;
}

function formatResult(r: Result<number>): string {
  if (r.tag === "Ok") return `Ok(${r.value})`;
  return `Err(${r.message})`;
}

console.log(formatResult(parse_int("42")));
console.log(formatResult(parse_int("abc")));

console.log(describe_result(compute({ tag: "Ok", value: 3 }, { tag: "Ok", value: 4 })));
console.log(describe_result(compute({ tag: "Err", message: "bad" }, { tag: "Ok", value: 4 })));
console.log(describe_result(compute({ tag: "Ok", value: 3 }, { tag: "Err", message: "bad" })));

console.log(describe_result(compute(parse_int("42"), parse_int("10"))));
console.log(describe_result(compute(parse_int("42"), parse_int("abc"))));
