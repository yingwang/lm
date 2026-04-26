type Result = { tag: "Ok"; value: number } | { tag: "Err"; msg: string };

function Ok(value: number): Result { return { tag: "Ok", value }; }
function Err(msg: string): Result { return { tag: "Err", msg }; }

function formatResult(r: Result): string {
  if (r.tag === "Ok") return `Ok(${r.value})`;
  return `Err(${r.msg})`;
}

function parse_int(s: string): Result {
  const n = Number(s);
  if (!isNaN(n) && s.trim() !== "" && Number.isInteger(n)) return Ok(n);
  return Err("not a number: " + s);
}

function compute(a: Result, b: Result): Result {
  if (a.tag === "Err") return a;
  if (b.tag === "Err") return b;
  return Ok(a.value + b.value);
}

function describe_result(r: Result): string {
  if (r.tag === "Ok") return `Success: ${r.value}`;
  return `Error: ${r.msg}`;
}

console.log(formatResult(parse_int("42")));
console.log(formatResult(parse_int("abc")));
console.log(describe_result(compute(Ok(3), Ok(4))));
console.log(describe_result(compute(Err("bad"), Ok(4))));
console.log(describe_result(compute(Ok(3), Err("bad"))));
console.log(describe_result(compute(parse_int("42"), parse_int("10"))));
console.log(describe_result(compute(parse_int("42"), parse_int("abc"))));
