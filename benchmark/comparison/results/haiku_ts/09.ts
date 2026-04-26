function processData(lst: number[]): number {
  // Filter negatives
  const filtered = lst.filter(x => x >= 0);
  // Double
  const doubled = filtered.map(x => x * 2);
  // Sum
  return doubled.reduce((sum, x) => sum + x, 0);
}

function run(lst: number[]): void {
  const result = processData(lst);
  console.log(`Total: ${result}`);
}

// Test cases
run([1, 2, 3]);
run([-1, -2]);
run([5, -3, 5]);
