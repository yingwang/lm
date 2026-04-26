function find_first_even(lst: number[]): number {
  for (const n of lst) {
    if (n % 2 === 0) return n;
  }
  return -1;
}

console.log(find_first_even([1, 3, 2, 5]));
console.log(find_first_even([1, 3, 5]));
console.log(find_first_even([4, 2, 6]));
