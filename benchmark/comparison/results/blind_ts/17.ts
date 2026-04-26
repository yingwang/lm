function zip(a: number[], b: number[]): string[] {
  const len = Math.min(a.length, b.length);
  const result: string[] = [];
  for (let i = 0; i < len; i++) {
    result.push(`Pair(${a[i]}, ${b[i]})`);
  }
  return result;
}

function formatList(lst: string[]): string {
  return "[" + lst.join(", ") + "]";
}

console.log(formatList(zip([1, 2, 3], [4, 5, 6])));
console.log(formatList(zip([1, 2], [3])));
console.log(formatList(zip([], [1])));
