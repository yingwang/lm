function repeat(s: string, n: number, sep: string): string {
  if (n === 0) return "";
  const parts: string[] = [];
  for (let i = 0; i < n; i++) parts.push(s);
  return parts.join(sep);
}

console.log(repeat("ha", 3, " "));
console.log(repeat("ab", 1, ","));
console.log(repeat("x", 0, ","));
console.log(repeat("hi", 2, "-"));
