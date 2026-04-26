function circle_circumference(radius: number): number {
  const pi = 3.14159;
  return 2 * pi * radius;
}

for (const r of [5, 10, 0]) {
  console.log(circle_circumference(r));
}
