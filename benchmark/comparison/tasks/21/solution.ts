type Result<T> = { tag: "Ok"; value: T } | { tag: "Err"; message: string };

function map_result(r: Result<number>, f: (x: number) => number): Result<number> {
  if (r.tag === "Ok") return { tag: "Ok", value: f(r.value) };
  return r;
}

function and_then_divide(r: Result<number>, divisor: number): Result<number> {
  if (r.tag === "Err") return r;
  if (divisor === 0) return { tag: "Err", message: "division by zero" };
  return { tag: "Ok", value: Math.trunc(r.value / divisor) };
}

function formatResult(r: Result<number>): string {
  if (r.tag === "Ok") return `Ok(${r.value})`;
  return `Err(${r.message})`;
}

// map_result tests
console.log(formatResult(map_result({ tag: "Ok", value: 10 }, x => x + 5)));
console.log(formatResult(map_result({ tag: "Err", message: "fail" }, x => x + 5)));

// and_then_divide tests
console.log(formatResult(and_then_divide({ tag: "Ok", value: 10 }, 2)));
console.log(formatResult(and_then_divide({ tag: "Ok", value: 10 }, 0)));
console.log(formatResult(and_then_divide({ tag: "Err", message: "fail" }, 2)));

// chained
console.log(formatResult(and_then_divide(map_result({ tag: "Ok", value: 10 }, x => x + 10), 2)));
