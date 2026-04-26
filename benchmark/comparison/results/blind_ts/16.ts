function contains(lst: number[], target: number): boolean {
  for (const x of lst) {
    if (x === target) return true;
  }
  return false;
}

console.log(String(contains([1, 2, 3], 2)));
console.log(String(contains([1, 2, 3], 4)));
console.log(String(contains([], 1)));
