function formatList(arr: number[]): string {
  return "[" + arr.join(", ") + "]";
}

function list_reverse(arr: number[]): number[] {
  return [...arr].reverse();
}

console.log(formatList(list_reverse([1, 2, 3])));
console.log(formatList(list_reverse([])));
console.log(formatList(list_reverse([1])));
