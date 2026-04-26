type Result = { tag: "Ok"; value: number } | { tag: "Err"; msg: string };

function Ok(value: number): Result { return { tag: "Ok", value }; }
function Err(msg: string): Result { return { tag: "Err", msg }; }

function formatResult(r: Result): string {
  if (r.tag === "Ok") return `Ok(${r.value})`;
  return `Err(${r.msg})`;
}

function map_result(r: Result, addend: number): Result {
  if (r.tag === "Ok") return Ok(r.value + addend);
  return r;
}

function and_then_divide(r: Result, divisor: number): Result {
  if (r.tag === "Err") return r;
  if (divisor === 0) return Err("division by zero");
  return Ok(Math.trunc(r.value / divisor));
}

console.log(formatResult(map_result(Ok(5), 10)));
console.log(formatResult(map_result(Err("fail"), 10)));
console.log(formatResult(and_then_divide(Ok(10), 2)));
console.log(formatResult(and_then_divide(Ok(10), 0)));
console.log(formatResult(and_then_divide(Err("fail"), 2)));
console.log(formatResult(and_then_divide(map_result(Ok(20), 10), 3)));
