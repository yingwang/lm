function runningSum(lst: number[]): number[] {
  const result: number[] = [];
  let sum = 0;
  for (const num of lst) {
    sum += num;
    result.push(sum);
  }
  return result;
}

function formatList(lst: number[]): string {
  return "[" + lst.join(", ") + "]";
}

// Test cases
console.log(formatList(runningSum([1, 2, 3, 4])));
console.log(formatList(runningSum([])));
console.log(formatList(runningSum([5])));
console.log(formatList(runningSum([-1, 2, 3])));
