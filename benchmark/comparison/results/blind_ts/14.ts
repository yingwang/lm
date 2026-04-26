function filter(lst: number[], pred: (x: number) => boolean): number[] {
  const result: number[] = [];
  for (const x of lst) {
    if (pred(x)) result.push(x);
  }
  return result;
}

function gt3(x: number): boolean { return x > 3; }
function is_even(x: number): boolean { return x % 2 === 0; }

function formatList(lst: number[]): string {
  return "[" + lst.join(", ") + "]";
}

console.log(formatList(filter([1, 2, 3, 4, 5], gt3)));
console.log(formatList(filter([1, 2, 3], is_even)));
console.log(formatList(filter([], gt3)));
