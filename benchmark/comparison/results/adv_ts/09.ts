function process_data(lst: number[]): number {
  return lst
    .filter(n => n >= 0)
    .map(n => n * 2)
    .reduce((a, b) => a + b, 0);
}

function run(lst: number[]): void {
  console.log(`Total: ${process_data(lst)}`);
}

run([1, 2, 3]);
run([-1, -2]);
run([5, -3, 5]);
