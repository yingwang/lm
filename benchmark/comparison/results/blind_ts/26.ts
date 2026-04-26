function format_greeting(name: string, age: number): string {
  return `Hello, ${name}! You are ${age} years old.`;
}

function print_greeting(name: string, age: number): void {
  console.log("=== Greeting ===");
  console.log(format_greeting(name, age));
  console.log("================");
}

print_greeting("Alice", 30);
print_greeting("Bob", 25);
