function compute(x: number, y: number): number {
  return (x + y) * (x + y);
}

function show_result(value: number): void {
  console.log(`Result: ${value}`);
}

show_result(compute(3, 4));
show_result(compute(2, 3));
