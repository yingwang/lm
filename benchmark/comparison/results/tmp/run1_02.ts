function reverse(s: string): string {
  return s.split('').reverse().join('');
}

console.log(reverse("hello"));
console.log(reverse("ab"));
console.log(reverse(""));
console.log(reverse("a"));