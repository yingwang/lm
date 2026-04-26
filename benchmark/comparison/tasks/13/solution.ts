function list_sum(arr: number[]): number {
  let sum = 0;
  for (const x of arr) sum += x;
  return sum;
}

console.log(list_sum([1, 2, 3]));
console.log(list_sum([]));
console.log(list_sum([10]));
console.log(list_sum([1, -1]));
