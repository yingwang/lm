type Option<T> = { tag: "Some"; value: T } | { tag: "None" };

function unwrap_or<T>(opt: Option<T>, defaultVal: T): T {
  if (opt.tag === "Some") return opt.value;
  return defaultVal;
}

console.log(unwrap_or({ tag: "Some", value: 42 }, 0));
console.log(unwrap_or({ tag: "None" }, 0));
console.log(unwrap_or({ tag: "Some", value: -1 }, 0));
console.log(unwrap_or({ tag: "None" }, -1));
