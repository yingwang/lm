function sum(lst: number[]): number {
  let s = 0;
  for (const x of lst) s += x;
  return s;
}

console.log(sum([1, 2, 3]));
console.log(sum([]));
console.log(sum([10]));
console.log(sum([-1, 1]));
