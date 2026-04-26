function list_zip(a: number[], b: number[]): string {
  const len = Math.min(a.length, b.length);
  const pairs: string[] = [];
  for (let i = 0; i < len; i++) {
    pairs.push(`Pair(${a[i]}, ${b[i]})`);
  }
  return "[" + pairs.join(", ") + "]";
}

console.log(list_zip([1, 2, 3], [4, 5, 6]));
console.log(list_zip([1, 2], [3]));
console.log(list_zip([], [1, 2]));
