type Option<T> = { tag: "Some"; value: T } | { tag: "None" };

function max2(a: number, b: number): number {
  return a >= b ? a : b;
}

function max_opt(arr: number[]): Option<number> {
  if (arr.length === 0) return { tag: "None" };
  let m = arr[0];
  for (let i = 1; i < arr.length; i++) {
    if (arr[i] > m) m = arr[i];
  }
  return { tag: "Some", value: m };
}

function formatOption(opt: Option<number>): string {
  if (opt.tag === "Some") return `Some(${opt.value})`;
  return "None";
}

console.log(max2(3, 7));
console.log(max2(10, 5));
console.log(max2(5, 5));

console.log(formatOption(max_opt([3, 7, 2])));
console.log(formatOption(max_opt([5])));
console.log(formatOption(max_opt([1, 3, 2])));
console.log(formatOption(max_opt([])));
