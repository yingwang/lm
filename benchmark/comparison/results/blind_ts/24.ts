type Option = { tag: "Some"; value: number } | { tag: "None" };

function Some(v: number): Option { return { tag: "Some", value: v }; }
const None: Option = { tag: "None" };

function max2(a: number, b: number): number {
  return a >= b ? a : b;
}

function max_opt(a: Option, b: Option): Option {
  if (a.tag === "Some" && b.tag === "Some") return Some(max2(a.value, b.value));
  if (a.tag === "Some") return a;
  if (b.tag === "Some") return b;
  return None;
}

function formatOpt(o: Option): string {
  if (o.tag === "Some") return `Some(${o.value})`;
  return "None";
}

console.log(max2(3, 7));
console.log(max2(10, 2));
console.log(max2(5, 5));
console.log(formatOpt(max_opt(Some(3), Some(7))));
console.log(formatOpt(max_opt(Some(5), None)));
console.log(formatOpt(max_opt(None, Some(3))));
console.log(formatOpt(max_opt(None, None)));
