function formatList(arr: number[]): string {
  return "[" + arr.join(", ") + "]";
}

function list_filter(arr: number[], pred: (x: number) => boolean): number[] {
  return arr.filter(pred);
}

console.log(formatList(list_filter([1, 2, 3, 4, 5], x => x > 3)));
console.log(formatList(list_filter([1, 2, 3], x => x % 2 === 0)));
console.log(formatList(list_filter([], x => x > 0)));
