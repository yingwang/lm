function reverse(lst: number[]): number[] {
  const result: number[] = [];
  for (let i = lst.length - 1; i >= 0; i--) result.push(lst[i]);
  return result;
}

function formatList(lst: number[]): string {
  return "[" + lst.join(", ") + "]";
}

console.log(formatList(reverse([1, 2, 3])));
console.log(formatList(reverse([])));
console.log(formatList(reverse([1])));
