function count_vowels(s: string): number {
  const vowels = new Set(["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"]);
  let count = 0;
  for (const c of s) {
    if (vowels.has(c)) count++;
  }
  return count;
}

console.log(count_vowels("hello"));
console.log(count_vowels("aeiou"));
console.log(count_vowels("xyz"));
console.log(count_vowels(""));
