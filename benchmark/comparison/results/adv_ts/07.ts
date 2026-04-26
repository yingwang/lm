type Option<T> = { tag: "Some"; value: T } | { tag: "None" };

function some<T>(value: T): Option<T> {
  return { tag: "Some", value };
}

function none<T>(): Option<T> {
  return { tag: "None" };
}

function format_option(o: Option<number>): string {
  return o.tag === "Some" ? `Some(${o.value})` : "None";
}

function safe_head(lst: number[]): Option<number> {
  if (lst.length === 0) return none();
  return some(lst[0]);
}

function safe_max(a: Option<number>, b: Option<number>): Option<number> {
  if (a.tag === "Some" && b.tag === "Some") return some(Math.max(a.value, b.value));
  if (a.tag === "Some") return a;
  if (b.tag === "Some") return b;
  return none();
}

function heads_max(a: number[], b: number[]): Option<number> {
  return safe_max(safe_head(a), safe_head(b));
}

console.log(format_option(heads_max([5, 1], [3, 2])));
console.log(format_option(heads_max([], [3])));
console.log(format_option(heads_max([3], [])));
console.log(format_option(heads_max([], [])));
