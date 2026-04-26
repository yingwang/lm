type Shape =
  | { tag: "Circle"; radius: number }
  | { tag: "Rect"; width: number; height: number }
  | { tag: "Triangle"; base: number; height: number };

function area(shape: Shape): number {
  switch (shape.tag) {
    case "Circle":
      return 3.14159 * shape.radius * shape.radius;
    case "Rect":
      return shape.width * shape.height;
    case "Triangle":
      return 0.5 * shape.base * shape.height;
  }
}

function formatArea(val: number): string {
  if (Number.isInteger(val)) {
    return val.toFixed(1);
  }
  // Remove trailing zeros but keep at least one decimal
  const s = val.toString();
  return s;
}

console.log(formatArea(area({ tag: "Circle", radius: 5 })));
console.log(formatArea(area({ tag: "Rect", width: 3, height: 4 })));
console.log(formatArea(area({ tag: "Triangle", base: 6, height: 4 })));
