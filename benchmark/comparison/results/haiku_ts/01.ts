function formatScore(name: string, score: number): string {
  return `${name}: ${score} points`;
}

// Test cases
console.log(formatScore("Alice", 42));
console.log(formatScore("Bob", 0));
