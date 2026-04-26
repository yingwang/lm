function format_score(name: string, score: number): string {
  return `${name}: ${score} points`;
}

console.log(format_score("Alice", 42));
console.log(format_score("Bob", 0));
