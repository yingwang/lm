type Option<T> = { tag: "Some"; value: T } | { tag: "None" };

function Some<T>(value: T): Option<T> { return { tag: "Some", value }; }
function None<T>(): Option<T> { return { tag: "None" }; }

function unwrap_or<T>(opt: Option<T>, def: T): T {
  if (opt.tag === "Some") return opt.value;
  return def;
}

console.log(unwrap_or(Some(42), 0));
console.log(unwrap_or(None<number>(), 0));
console.log(unwrap_or(Some(-1), 99));
console.log(unwrap_or(None<number>(), -1));
