function list_contains(arr: number[], val: number): boolean {
  return arr.includes(val);
}

console.log(list_contains([1, 2, 3], 2) ? "true" : "false");
console.log(list_contains([1, 2, 3], 4) ? "true" : "false");
console.log(list_contains([], 1) ? "true" : "false");
