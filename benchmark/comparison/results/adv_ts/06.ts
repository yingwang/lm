function running_sum(lst: number[]): number[] {
  const result: number[] = [];
  let sum = 0;
  for (const n of lst) {
    sum += n;
    result.push(sum);
  }
  return result;
}

function format_list(lst: number[]): string {
  return `[${lst.join(", ")}]`;
}

for (const test of [[1, 2, 3, 4], [], [5], [-1, 2, 3]]) {
  console.log(format_list(running_sum(test)));
}
