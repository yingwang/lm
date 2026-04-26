function flatten(lst: number[][]): number[] {
  const result: number[] = [];
  for (const sub of lst) {
    for (const x of sub) result.push(x);
  }
  return result;
}

function formatList(lst: number[]): string {
  return "[" + lst.join(", ") + "]";
}

console.log(formatList(flatten([[1, 2], [3], [4, 5]])));
console.log(formatList(flatten([[], [1], []])));
console.log(formatList(flatten([])));
