function formatList(arr: number[]): string {
  return "[" + arr.join(", ") + "]";
}

function list_flatten(lists: number[][]): number[] {
  const result: number[] = [];
  for (const list of lists) {
    for (const item of list) {
      result.push(item);
    }
  }
  return result;
}

console.log(formatList(list_flatten([[1, 2], [3], [4, 5]])));
console.log(formatList(list_flatten([[1]])));
console.log(formatList(list_flatten([])));
