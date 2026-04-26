function findFirstEven(lst: number[]): number {
  for (const num of lst) {
    if (num % 2 === 0) {
      return num;
    }
  }
  return -1;
}

// Test cases
console.log(findFirstEven([1, 3, 2, 5]));
console.log(findFirstEven([1, 3, 5]));
console.log(findFirstEven([4, 2, 6]));
