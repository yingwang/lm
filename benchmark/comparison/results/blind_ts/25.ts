function compute(x: number, y: number): number {
  return (x + y) * (x + y);
}

function show_result(x: number): void {
  console.log(`Result: ${x}`);
}

show_result(compute(3, 4));
show_result(compute(0, 5));
