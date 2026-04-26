function flatten<T>(lst: T[][]): T[] {
  return lst.flat();
}

function formatList<T>(lst: T[]): string {
  return `[${lst.join(', ')}]`;
}

console.log(formatList(flatten([[1, 2], [3], [4, 5]])));
console.log(formatList(flatten([[], [1], []])));
console.log(formatList(flatten([])));